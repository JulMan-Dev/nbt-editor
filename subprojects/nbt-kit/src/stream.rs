use std::collections::BTreeMap;
use std::io::Read;
use std::iter::repeat_with;
use flate2::read::DeflateDecoder;
use crate::kind::{ByteArray, Compound, IntArray, List, LongArray, Tag};
use crate::traits::*;

pub struct NbtReader<T> {
    inner: T
}

impl<T: Read> NbtReader<T> {
    pub const fn new(reader: T) -> Self {
        Self { inner: reader  }
    }
}

impl<T: Read> TagProducer for NbtReader<T> {
    fn take_tag(&mut self, root: bool) -> Option<Tag> {
        let byte = self.take_byte(false)?;

        match byte {
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

    fn take_compressed_tag(&mut self, root: bool) -> Option<Tag> {
        let mut reader = NbtReader::new(DeflateDecoder::new(&mut self.inner));
        reader.take_tag(root)
    }
}

impl<T: Read> ByteProducer for NbtReader<T> {
    fn take_byte(&mut self, _root: bool) -> Option<i8> {
        let mut buf = [0];
        self.inner.read_exact(&mut buf).ok()?;
        Some(buf[0] as i8)
    }
}

impl<T: Read> ShortProducer for NbtReader<T> {
    fn take_short(&mut self, _root: bool) -> Option<i16> {
        let mut buf = [0, 0];
        self.inner.read_exact(&mut buf).ok()?;
        Some(i16::from_be_bytes(buf))
    }
}

impl<T: Read> IntProducer for NbtReader<T> {
    fn take_int(&mut self, _root: bool) -> Option<i32> {
        let mut buf = [0; 4];
        self.inner.read_exact(&mut buf).ok()?;
        Some(i32::from_be_bytes(buf))
    }
}

impl<T: Read> LongProducer for NbtReader<T> {
    fn take_long(&mut self, _root: bool) -> Option<i64> {
        let mut buf = [0; 8];
        self.inner.read_exact(&mut buf).ok()?;
        Some(i64::from_be_bytes(buf))
    }
}

impl<T: Read> FloatProducer for NbtReader<T> {
    fn take_float(&mut self, _root: bool) -> Option<f32> {
        let mut buf = [0; 4];
        self.inner.read_exact(&mut buf).ok()?;
        Some(f32::from_be_bytes(buf))
    }
}

impl<T: Read> DoubleProducer for NbtReader<T> {
    fn take_double(&mut self, _root: bool) -> Option<f64> {
        let mut buf = [0; 8];
        self.inner.read_exact(&mut buf).ok()?;
        Some(f64::from_be_bytes(buf))
    }
}

impl<T: Read> ByteArrayProducer for NbtReader<T> {
    fn take_byte_array(&mut self, _root: bool) -> Option<ByteArray> {
        let length = self.take_int(false)? as usize;
        let bytes = repeat_with(|| self.take_byte(false))
            .take(length)
            .fold(Some(Vec::with_capacity(length)), |acc, x| {
                match (acc, x) {
                    (Some(mut acc), Some(i)) => {
                        acc.push(i);
                        Some(acc)
                    }
                    _ => None,
                }
            })?;

        Some(ByteArray::new(bytes.into_boxed_slice()))
    }
}

impl<T: Read> StringProducer for NbtReader<T> {
    fn take_string(&mut self, _root: bool) -> Option<String> {
        let size = self.take_short(false)? as usize;
        let mut bytes = vec![0; size];
        self.inner.read_exact(&mut bytes).ok()?;

        let str = if cfg!(debug_assertions) {
            // the cost of reading the slice twice can be avoided by running in release mode
            String::from_utf8_lossy(&bytes).into_owned()
        } else {
            // SAFETY: the NBT standard enforces strings to be valid UTF-8
            unsafe { String::from_utf8_unchecked(bytes) }
        };

        Some(str)
    }
}

impl<T: Read> ListProducer for NbtReader<T> {
    fn take_list(&mut self, _root: bool) -> Option<List> {
        let tag = self.take_byte(false)?;
        let size = self.take_int(false)? as usize;

        if tag == 0 {
            if size > 0 {
                todo!("handle this error");
            }

            return Some(List::Empty);
        }

        macro_rules! impl_list {
            ($ty:ident => $expr:expr) => {{
                let mut buffer = Box::new_uninit_slice(size);

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

impl<T: Read> CompoundProducer for NbtReader<T> {
    fn take_compound(&mut self, mut root: bool) -> Option<Compound> {
        let mut tree = BTreeMap::new();

        loop {
            if root {
                root = false;
                let mut buf = [0, 0];
                self.inner.read_exact(&mut buf).ok()?;
                if buf != [0, 0] {
                    return None;
                }
            }

            let tag = self.take_byte(false)?;

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

impl<T: Read> IntArrayProducer for NbtReader<T> {
    fn take_int_array(&mut self, _root: bool) -> Option<IntArray> {
        let length = self.take_int(false)? as usize;
        let ints = repeat_with(|| self.take_int(false))
            .take(length)
            .fold(Some(Vec::with_capacity(length)), |acc, x| {
                match (acc, x) {
                    (Some(mut acc), Some(i)) => {
                        acc.push(i);
                        Some(acc)
                    }
                    _ => None,
                }
            })?;

        Some(IntArray::new(ints.into_boxed_slice()))
    }
}

impl<T: Read> LongArrayProducer for NbtReader<T> {
    fn take_long_array(&mut self, _root: bool) -> Option<LongArray> {
        let length = self.take_int(false)? as usize;
        let longs = repeat_with(|| self.take_long(false))
            .take(length)
            .fold(Some(Vec::with_capacity(length)), |acc, x| {
                match (acc, x) {
                    (Some(mut acc), Some(i)) => {
                        acc.push(i);
                        Some(acc)
                    }
                    _ => None,
                }
            })?;

        Some(LongArray::new(longs.into_boxed_slice()))
    }
}
