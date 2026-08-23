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
    fn take_tag(&mut self, root: bool) -> Option<Tag> {
        let byte = self.inner.first()?;
        self.offset(1);

        match *byte {
            1 => self.take_byte(root).map(Tag::Byte),
            2 => self.take_short(root).map(Tag::Short),
            3 => self.take_int(root).map(Tag::Int),
            4 => self.take_long(root).map(Tag::Long),
            5 => self.take_float(root).map(Tag::Float),
            6 => self.take_double(root).map(Tag::Double),
            7 => self.take_byte_array(root).map(Tag::ByteArray),
            8 => self.take_string(root).map(Tag::String),
            9 => self.take_list(root).map(Tag::List),
            10 => self.take_compound(root).map(Tag::Compound),
            11 => self.take_int_array(root).map(Tag::IntArray),
            12 => self.take_long_array(root).map(Tag::LongArray),
            _ => panic!("unsupported tag type"),
        }
    }
}

// The implementations of TagProducers are unsafe, they assume you checked the type byte before
// taking the value. This check is always done in debug.

impl ByteProducer for BinaryParser<'_> {
    fn take_byte(&mut self, _root: bool) -> Option<i8> {
        let byte = *self.inner.first()? as i8;
        self.offset(1);
        Some(byte)
    }
}

impl ShortProducer for BinaryParser<'_> {
    fn take_short(&mut self, _root: bool) -> Option<i16> {
        let short = i16::from_be_bytes(*self.inner.first_chunk()?);
        self.offset(2);
        Some(short)
    }
}

impl IntProducer for BinaryParser<'_> {
    fn take_int(&mut self, _root: bool) -> Option<i32> {
        let int = i32::from_be_bytes(*self.inner.first_chunk()?);
        self.offset(4);
        Some(int)
    }
}

impl LongProducer for BinaryParser<'_> {
    fn take_long(&mut self, _root: bool) -> Option<i64> {
        let long = i64::from_be_bytes(*self.inner.first_chunk()?);
        self.offset(8);
        Some(long)
    }
}

impl FloatProducer for BinaryParser<'_> {
    fn take_float(&mut self, _root: bool) -> Option<f32> {
        let float = f32::from_be_bytes(*self.inner.first_chunk()?);
        self.offset(4);
        Some(float)
    }
}

impl DoubleProducer for BinaryParser<'_> {
    fn take_double(&mut self, _root: bool) -> Option<f64> {
        let double = f64::from_be_bytes(*self.inner.first_chunk()?);
        self.offset(8);
        Some(double)
    }
}

impl ByteArrayProducer for BinaryParser<'_> {
    fn take_byte_array(&mut self, _root: bool) -> Option<ByteArray> {
        let size = i32::from_be_bytes(*self.inner.first_chunk()?);
        self.offset(4);

        // get rid of vectors
        let mut buffer = Box::new_uninit_slice(size as usize);

        for ptr in buffer.iter_mut() {
            ptr.write(self.take_byte(false)?);
        }

        // SAFETY: all elements are initialized
        Some(unsafe { ByteArray::new(buffer.assume_init()) })
    }
}

impl StringProducer for BinaryParser<'_> {
    fn take_string(&mut self, _root: bool) -> Option<String> {
        let size = u16::from_be_bytes(*self.inner.first_chunk()?);
        self.offset(2);

        let str = if cfg!(debug_assertions) {
            // the cost of reading the slice twice can be avoided by running in release mode
            String::from_utf8_lossy(&self.inner[0..size as usize]).into_owned()
        } else {
            // SAFETY: the NBT standard enforces strings to be valid UTF-8
            unsafe { String::from_utf8_unchecked(self.inner[0..size as usize].to_owned()) }
        };
        self.offset(size as usize);

        Some(str)
    }
}

impl ListProducer for BinaryParser<'_> {
    fn take_list(&mut self, _root: bool) -> Option<List> {
        let tag = *self.inner.first()?;
        self.offset(1);

        let size = u32::from_be_bytes(*self.inner.first_chunk()?);
        self.offset(4);

        if tag == 0 {
            if size > 0 {
                todo!("handle this error");
            }

            return Some(List::Empty);
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
            1 => impl_list!(Byte => self.take_byte(false)?),
            2 => impl_list!(Short => self.take_short(false)?),
            3 => impl_list!(Int => self.take_int(false)?),
            4 => impl_list!(Long => self.take_long(false)?),
            5 => impl_list!(Float => self.take_float(false)?),
            6 => impl_list!(Double => self.take_double(false)?),
            7 => impl_list!(ByteArray => self.take_byte_array(false)?),
            8 => impl_list!(String => self.take_string(false)?),
            9 => impl_list!(List => self.take_list(false)?),
            10 => impl_list!(Compound => self.take_compound(false)?),
            11 => impl_list!(IntArray => self.take_int_array(false)?),
            12 => impl_list!(LongArray => self.take_long_array(false)?),
            _ => panic!("unsupported tag type"),
        }
    }
}

impl CompoundProducer for BinaryParser<'_> {
    fn take_compound(&mut self, mut root: bool) -> Option<Compound> {
        let mut tree = BTreeMap::new();

        loop {
            if root {
                root = false;
                if Some(&[0, 0]) != self.inner.first_chunk() {
                    return None;
                }
                self.offset(2);
            }

            let tag = *self.inner.first()?;
            self.offset(1);

            if tag == 0 {
                break Some(Compound::new(tree));
            }

            let string = self.take_string(false)?;

            let tag = match tag {
                0 => unreachable!(),
                1 => Tag::Byte(self.take_byte(false)?),
                2 => Tag::Short(self.take_short(false)?),
                3 => Tag::Int(self.take_int(false)?),
                4 => Tag::Long(self.take_long(false)?),
                5 => Tag::Float(self.take_float(false)?),
                6 => Tag::Double(self.take_double(false)?),
                7 => Tag::ByteArray(self.take_byte_array(false)?),
                8 => Tag::String(self.take_string(false)?),
                9 => Tag::List(self.take_list(false)?),
                10 => Tag::Compound(self.take_compound(false)?),
                11 => Tag::IntArray(self.take_int_array(false)?),
                12 => Tag::LongArray(self.take_long_array(false)?),
                _ => panic!("unsupported tag type"),
            };

            tree.insert(string, tag);
        }
    }
}

impl IntArrayProducer for BinaryParser<'_> {
    fn take_int_array(&mut self, _root: bool) -> Option<IntArray> {
        let size = u32::from_be_bytes(*self.inner.first_chunk()?);
        self.offset(4);

        let mut buffer = Box::new_uninit_slice(size as usize);

        for ptr in buffer.iter_mut() {
            ptr.write(self.take_int(false)?);
        }

        // SAFETY: all elements are initialized
        Some(unsafe { IntArray::new(buffer.assume_init()) })
    }
}

impl LongArrayProducer for BinaryParser<'_> {
    fn take_long_array(&mut self, _root: bool) -> Option<LongArray> {
        let size = u32::from_be_bytes(*self.inner.first_chunk()?);
        self.offset(4);

        let mut buffer = Box::new_uninit_slice(size as usize);

        for ptr in buffer.iter_mut() {
            ptr.write(self.take_long(false)?);
        }

        // SAFETY: all elements are initialized
        Some(unsafe { LongArray::new(buffer.assume_init()) })
    }
}
