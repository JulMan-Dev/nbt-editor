use core::ops::{Deref, DerefMut};
use crate::kind::{ByteArray, Compound, IntArray, List, LongArray, Tag};
use crate::traits::*;

#[derive(Debug, Clone)]
pub struct BinarySerializer {
    inner: Vec<u8>,
}

impl Deref for BinarySerializer {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for BinarySerializer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl BinarySerializer {
    pub const fn new(inner: Vec<u8>) -> Self {
        Self { inner }
    }
}

impl TagWriter for BinarySerializer {
    fn write_tag(&mut self, value: Tag) {
        match value {
            Tag::Empty => {
                self.inner.push(0);
            }
            Tag::Byte(byte) => {
                self.inner.push(1);
                self.write_byte(byte);
            }
            Tag::Short(short) => {
                self.inner.push(2);
                self.write_short(short);
            }
            Tag::Int(int) => {
                self.inner.push(3);
                self.write_int(int);
            }
            Tag::Long(long) => {
                self.inner.push(4);
                self.write_long(long);
            }
            Tag::Float(float) => {
                self.inner.push(5);
                self.write_float(float);
            }
            Tag::Double(double) => {
                self.inner.push(6);
                self.write_double(double);
            }
            Tag::ByteArray(array) => {
                self.inner.push(7);
                self.write_byte_array(array);
            }
            Tag::String(str) => {
                self.inner.push(8);
                self.write_string(str);
            }
            Tag::List(list) => {
                self.inner.push(9);
                self.write_list(list);
            }
            Tag::Compound(compound) => {
                self.inner.push(10);
                self.write_compound(compound);
            }
            Tag::IntArray(array) => {
                self.inner.push(11);
                self.write_int_array(array);
            }
            Tag::LongArray(array) => {
                self.inner.push(12);
                self.write_long_array(array);
            }
        }
    }
}

impl ByteWriter for BinarySerializer {
    fn write_byte(&mut self, value: i8) {
        self.inner.push(value as u8);
    }
}

impl ShortWriter for BinarySerializer {
    fn write_short(&mut self, value: i16) {
        self.inner.extend(value.to_be_bytes());
    }
}

impl IntWriter for BinarySerializer {
    fn write_int(&mut self, value: i32) {
        self.inner.extend(value.to_be_bytes());
    }
}

impl LongWriter for BinarySerializer {
    fn write_long(&mut self, value: i64) {
        self.inner.extend(value.to_be_bytes());
    }
}

impl FloatWriter for BinarySerializer {
    fn write_float(&mut self, value: f32) {
        self.inner.extend(value.to_be_bytes());
    }
}

impl DoubleWriter for BinarySerializer {
    fn write_double(&mut self, value: f64) {
        self.inner.extend(value.to_be_bytes());
    }
}

impl ByteArrayWriter for BinarySerializer {
    fn write_byte_array(&mut self, value: ByteArray) {
        self.inner.extend((value.len() as u32).to_be_bytes());
        self.inner.extend(value.into_inner().into_iter().map(|x| x as u8));
    }
}

impl StringWriter for BinarySerializer {
    fn write_string(&mut self, value: String) {
        self.inner.extend((value.len() as u16).to_be_bytes());
        self.inner.extend(value.as_bytes());
    }
}

impl ListWriter for BinarySerializer {
    fn write_list(&mut self, value: List) {
        macro_rules! impl_list {
            (($id:expr, $ty:literal) |$v:ident| $expr:expr) => {{
                self.inner.push($ty);
                self.inner.extend(($id.len() as u32).to_be_bytes());

                for $v in $id {
                    $expr
                }
            }}
        }

        match value {
            List::Empty => impl_list!(([0; 0], 0) |_x| unreachable!()),
            List::Byte(bytes) => impl_list!((bytes, 1) |b| self.write_byte(b)),
            List::Short(shorts) => impl_list!((shorts, 2) |s| self.write_short(s)),
            List::Int(ints) => impl_list!((ints, 3) |i| self.write_int(i)),
            List::Long(longs) => impl_list!((longs, 4) |l| self.write_long(l)),
            List::Float(floats) => impl_list!((floats, 5) |f| self.write_float(f)),
            List::Double(doubles) => impl_list!((doubles, 6) |d| self.write_double(d)),
            List::ByteArray(arrays) => impl_list!((arrays, 7) |a| self.write_byte_array(a)),
            List::String(strings) => impl_list!((strings, 8) |s| self.write_string(s)),
            List::List(lists) => impl_list!((lists, 9) |l| self.write_list(l)),
            List::Compound(compounds) => impl_list!((compounds, 10) |c| self.write_compound(c)),
            List::IntArray(arrays) => impl_list!((arrays, 11) |a| self.write_int_array(a)),
            List::LongArray(arrays) => impl_list!((arrays, 12) |a| self.write_long_array(a)),
        }
    }
}

impl CompoundWriter for BinarySerializer {
    fn write_compound(&mut self, value: Compound) {
        for (k, v) in value.into_inner() {
            let string = {
                let mut bytes = BinarySerializer::new(Vec::new());
                bytes.write_string(k);
                bytes.inner
            };

            match v {
                Tag::Empty => unreachable!(),
                Tag::Byte(b) => {
                    self.inner.push(1);
                    self.inner.extend(string);
                    self.write_byte(b);
                }
                Tag::Short(s) => {
                    self.inner.push(2);
                    self.inner.extend(string);
                    self.write_short(s);
                }
                Tag::Int(i) => {
                    self.inner.push(3);
                    self.inner.extend(string);
                    self.write_int(i);
                }
                Tag::Long(l) => {
                    self.inner.push(4);
                    self.inner.extend(string);
                    self.write_long(l);
                }
                Tag::Float(f) => {
                    self.inner.push(5);
                    self.inner.extend(string);
                    self.write_float(f);
                }
                Tag::Double(d) => {
                    self.inner.push(6);
                    self.inner.extend(string);
                    self.write_double(d);
                }
                Tag::ByteArray(a) => {
                    self.inner.push(7);
                    self.inner.extend(string);
                    self.write_byte_array(a);
                }
                Tag::String(s) => {
                    self.inner.push(8);
                    self.inner.extend(string);
                    self.write_string(s);
                }
                Tag::List(l) => {
                    self.inner.push(9);
                    self.inner.extend(string);
                    self.write_list(l);
                }
                Tag::Compound(c) => {
                    self.inner.push(10);
                    self.inner.extend(string);
                    self.write_compound(c);
                }
                Tag::IntArray(a) => {
                    self.inner.push(11);
                    self.inner.extend(string);
                    self.write_int_array(a);
                }
                Tag::LongArray(a) => {
                    self.inner.push(12);
                    self.inner.extend(string);
                    self.write_long_array(a);
                }
            }
        }
    }
}

impl IntArrayWriter for BinarySerializer {
    fn write_int_array(&mut self, value: IntArray) {
        self.inner.extend((value.len() as u32).to_be_bytes());
        for i in value.into_inner() {
            self.write_int(i);
        }
    }
}

impl LongArrayWriter for BinarySerializer {
    fn write_long_array(&mut self, value: LongArray) {
        self.inner.extend((value.len() as u32).to_be_bytes());
        for l in value.into_inner() {
            self.write_long(l);
        }
    }
}
