use alloc::string::String;
use crate::kind::{ByteArray, Compound, IntArray, List, LongArray, Tag};

pub trait ByteProducer {
    fn take_byte(&mut self, root: bool) -> Option<i8>;
}

pub trait ByteWriter {
    fn write_byte(&mut self, value: i8);
}

pub trait ShortProducer {
    fn take_short(&mut self, root: bool) -> Option<i16>;
}

pub trait ShortWriter {
    fn write_short(&mut self, value: i16);
}

pub trait IntProducer {
    fn take_int(&mut self, root: bool) -> Option<i32>;
}

pub trait IntWriter {
    fn write_int(&mut self, value: i32);
}

pub trait LongProducer {
    fn take_long(&mut self, root: bool) -> Option<i64>;
}

pub trait LongWriter {
    fn write_long(&mut self, value: i64);
}

pub trait FloatProducer {
    fn take_float(&mut self, root: bool) -> Option<f32>;
}

pub trait FloatWriter {
    fn write_float(&mut self, value: f32);
}

pub trait DoubleProducer {
    fn take_double(&mut self, root: bool) -> Option<f64>;
}

pub trait DoubleWriter {
    fn write_double(&mut self, value: f64);
}

pub trait ByteArrayProducer {
    fn take_byte_array(&mut self, root: bool) -> Option<ByteArray>;
}

pub trait ByteArrayWriter {
    fn write_byte_array(&mut self, value: ByteArray);
}

pub trait StringProducer {
    fn take_string(&mut self, root: bool) -> Option<String>;
}

pub trait StringWriter {
    fn write_string(&mut self, value: String);
}

pub trait ListProducer {
    fn take_list(&mut self, root: bool) -> Option<List>;
}

pub trait ListWriter {
    fn write_list(&mut self, value: List);
}

pub trait CompoundProducer {
    fn take_compound(&mut self, root: bool) -> Option<Compound>;
}

pub trait CompoundWriter {
    fn write_compound(&mut self, value: Compound);
}

pub trait IntArrayProducer  {
    fn take_int_array(&mut self, root: bool) -> Option<IntArray>;
}

pub trait IntArrayWriter {
    fn write_int_array(&mut self, value: IntArray);
}

pub trait LongArrayProducer {
    fn take_long_array(&mut self, root: bool) -> Option<LongArray>;
}

pub trait LongArrayWriter {
    fn write_long_array(&mut self, value: LongArray);
}

pub trait TagProducer {
    fn take_tag(&mut self, root: bool) -> Option<Tag>;

    fn take_compressed_tag(&mut self, root: bool) -> Option<Tag>;
}

pub trait TagWriter {
    fn write_tag(&mut self, value: Tag);
}
