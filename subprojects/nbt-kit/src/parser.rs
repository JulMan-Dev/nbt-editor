use alloc::string::String;
use crate::kind::{ByteArray, Compound, IntArray, List, LongArray, Tag};

pub trait ByteProducer {
    fn take_byte(&mut self) -> Option<i8>;
}

pub trait ShortProducer {
    fn take_short(&mut self) -> Option<i16>;
}

pub trait IntProducer {
    fn take_int(&mut self) -> Option<i32>;
}

pub trait LongProducer {
    fn take_long(&mut self) -> Option<i64>;
}

pub trait FloatProducer {
    fn take_float(&mut self) -> Option<f32>;
}

pub trait DoubleProducer {
    fn take_double(&mut self) -> Option<f64>;
}

pub trait ByteArrayProducer {
    fn take_byte_array(&mut self) -> Option<ByteArray>;
}

pub trait StringProducer {
    fn take_string(&mut self) -> Option<String>;
}

pub trait ListProducer {
    fn take_list(&mut self) -> Option<List>;
}

pub trait CompoundProducer {
    fn take_compound(&mut self) -> Option<Compound>;
}

pub trait IntArrayProducer  {
    fn take_int_array(&mut self) -> Option<IntArray>;
}

pub trait LongArrayProducer {
    fn take_long_array(&mut self) -> Option<LongArray>;
}

pub trait TagProducer {
    fn take_tag(&mut self) -> Option<Tag>;
}
