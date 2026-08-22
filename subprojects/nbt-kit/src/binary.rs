use crate::kind::{ByteArray, Compound, IntArray, List, LongArray, Tag};
use crate::traits::*;
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use core::ops::Deref;

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct BinaryParser<'a> {
    inner: &'a [u8],
}

impl Deref for BinaryParser<'_> {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> From<&'a [u8]> for BinaryParser<'a> {
    fn from(value: &'a [u8]) -> Self {
        BinaryParser::from_ref(value)
    }
}

impl<'a> BinaryParser<'a> {
    pub const fn from_ref(buffer: &'a [u8]) -> Self {
        BinaryParser { inner: buffer }
    }

    pub const fn from_mut(buffer: &'a mut [u8]) -> Self {
        BinaryParser { inner: buffer }
    }

    pub const fn offset(&mut self, offset: usize) {
        self.inner = self.inner.split_at(offset).1;
    }
}

// We need to implement to &mut BinaryParser as we could update the reference, which need pointer to
// reference.
impl TagProducer for BinaryParser<'_> {
    fn take_tag(&mut self) -> Option<Tag> {
        let byte = self.inner.first()?;
        self.offset(1);

        match *byte {
            1 => self.take_byte().map(Tag::Byte),
            2 => self.take_short().map(Tag::Short),
            3 => self.take_int().map(Tag::Int),
            4 => self.take_long().map(Tag::Long),
            5 => self.take_float().map(Tag::Float),
            6 => self.take_double().map(Tag::Double),
            7 => self.take_byte_array().map(Tag::ByteArray),
            8 => self.take_string().map(Tag::String),
            9 => self.take_list().map(Tag::List),
            10 => self.take_compound().map(Tag::Compound),
            11 => self.take_int_array().map(Tag::IntArray),
            12 => self.take_long_array().map(Tag::LongArray),
            _ => panic!("unsupported tag type"),
        }
    }
}

// The implementations of TagProducers are unsafe, they assume you checked the type byte before
// taking the value. This check is always done in debug.

impl ByteProducer for BinaryParser<'_> {
    fn take_byte(&mut self) -> Option<i8> {
        let byte = *self.inner.first()? as i8;
        self.offset(1);
        Some(byte)
    }
}

impl ShortProducer for BinaryParser<'_> {
    fn take_short(&mut self) -> Option<i16> {
        let short = i16::from_be_bytes(*self.inner.first_chunk()?);
        self.offset(2);
        Some(short)
    }
}

impl IntProducer for BinaryParser<'_> {
    fn take_int(&mut self) -> Option<i32> {
        let int = i32::from_be_bytes(*self.inner.first_chunk()?);
        self.offset(4);
        Some(int)
    }
}

impl LongProducer for BinaryParser<'_> {
    fn take_long(&mut self) -> Option<i64> {
        let long = i64::from_be_bytes(*self.inner.first_chunk()?);
        self.offset(8);
        Some(long)
    }
}

impl FloatProducer for BinaryParser<'_> {
    fn take_float(&mut self) -> Option<f32> {
        let float = f32::from_be_bytes(*self.inner.first_chunk()?);
        self.offset(4);
        Some(float)
    }
}

impl DoubleProducer for BinaryParser<'_> {
    fn take_double(&mut self) -> Option<f64> {
        let double = f64::from_be_bytes(*self.inner.first_chunk()?);
        self.offset(8);
        Some(double)
    }
}

impl ByteArrayProducer for BinaryParser<'_> {
    fn take_byte_array(&mut self) -> Option<ByteArray> {
        let size = i64::from_be_bytes(*self.inner.first_chunk()?);
        self.offset(8);

        // get rid of vectors
        let mut buffer = Box::new_uninit_slice(size as usize);

        for ptr in buffer.iter_mut() {
            ptr.write(self.take_byte()?);
        }

        // SAFETY: all elements are initialized
        Some(unsafe { ByteArray::new(buffer.assume_init()) })
    }
}

impl StringProducer for BinaryParser<'_> {
    fn take_string(&mut self) -> Option<String> {
        let size = u32::from_be_bytes(*self.inner.first_chunk()?);
        self.offset(4);

        let str = if cfg!(debug_assertions) {
            // the cost of reading the slice twice can be avoided by running in release mode
            String::from_utf8_lossy(&self.inner[0..size as usize]).into_owned()
        } else {
            // SAFETY: the NBT standard enforces strings to be valid UTF-8
            unsafe { String::from_utf8_unchecked(self.inner[0..size as usize].to_owned()) }
        };

        Some(str)
    }
}

impl ListProducer for BinaryParser<'_> {
    fn take_list(&mut self) -> Option<List> {
        let tag = *self.inner.first()?;
        self.offset(1);

        let size = u64::from_be_bytes(*self.inner.first_chunk()?);
        self.offset(8);

        if tag == 0 && size > 0 {
            todo!("handle this error");
        }

        macro_rules! impl_list {
            ($ty:ident => $expr:expr) => {{
                let mut buffer = Box::new_uninit_slice(size as usize);

                for ptr in buffer.iter_mut() {
                    ptr.write($expr);
                }

                // SAFETY: all elements are initialized
                Some(List::$ty(unsafe { buffer.assume_init() }))
            }};
        }

        match tag {
            0 => unreachable!(),
            1 => impl_list!(Byte => self.take_byte()?),
            2 => impl_list!(Short => self.take_short()?),
            3 => impl_list!(Int => self.take_int()?),
            4 => impl_list!(Long => self.take_long()?),
            5 => impl_list!(Float => self.take_float()?),
            6 => impl_list!(Double => self.take_double()?),
            7 => impl_list!(ByteArray => self.take_byte_array()?),
            8 => impl_list!(String => self.take_string()?),
            9 => impl_list!(List => self.take_list()?),
            10 => impl_list!(Compound => self.take_compound()?),
            11 => impl_list!(IntArray => self.take_int_array()?),
            12 => impl_list!(LongArray => self.take_long_array()?),
            _ => panic!("unsupported tag type"),
        }
    }
}

impl CompoundProducer for BinaryParser<'_> {
    fn take_compound(&mut self) -> Option<Compound> {
        let mut tree = BTreeMap::new();

        loop {
            let tag = *self.inner.first()?;
            self.offset(1);

            if tag == 0 {
                break Some(Compound::new(tree));
            }

            let string = self.take_string()?;

            let tag = match tag {
                0 => unreachable!(),
                1 => Tag::Byte(self.take_byte()?),
                2 => Tag::Short(self.take_short()?),
                3 => Tag::Int(self.take_int()?),
                4 => Tag::Long(self.take_long()?),
                5 => Tag::Float(self.take_float()?),
                6 => Tag::Double(self.take_double()?),
                7 => Tag::ByteArray(self.take_byte_array()?),
                8 => Tag::String(self.take_string()?),
                9 => Tag::List(self.take_list()?),
                10 => Tag::Compound(self.take_compound()?),
                11 => Tag::IntArray(self.take_int_array()?),
                12 => Tag::LongArray(self.take_long_array()?),
                _ => panic!("unsupported tag type"),
            };

            tree.insert(string, tag);
        }
    }
}

impl IntArrayProducer for BinaryParser<'_> {
    fn take_int_array(&mut self) -> Option<IntArray> {
        let size = u64::from_be_bytes(*self.inner.first_chunk()?);
        self.offset(8);

        let mut buffer = Box::new_uninit_slice(size as usize);

        for ptr in buffer.iter_mut() {
            ptr.write(self.take_int()?);
        }

        // SAFETY: all elements are initialized
        Some(unsafe { IntArray::new(buffer.assume_init()) })
    }
}

impl LongArrayProducer for BinaryParser<'_> {
    fn take_long_array(&mut self) -> Option<LongArray> {
        let size = u64::from_be_bytes(*self.inner.first_chunk()?);
        self.offset(8);

        let mut buffer = Box::new_uninit_slice(size as usize);

        for ptr in buffer.iter_mut() {
            ptr.write(self.take_long()?);
        }

        // SAFETY: all elements are initialized
        Some(unsafe { LongArray::new(buffer.assume_init()) })
    }
}
