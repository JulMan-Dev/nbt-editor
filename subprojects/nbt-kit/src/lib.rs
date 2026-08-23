extern crate std;
extern crate core;
extern crate alloc;

pub mod binary;
pub mod traits;
pub mod kind;
#[cfg(all(feature = "objc", not(test)))]
pub mod libobjc;
pub mod writer;

#[cfg(test)]
mod tests {
    use crate::binary::BinaryParser;
    use crate::kind::Tag;
    use crate::traits::{TagProducer, TagWriter};
    use crate::writer::BinarySerializer;

    #[test]
    fn byte() {
        let mut serializer = BinarySerializer::new(vec![]);
        serializer.write_tag(Tag::Byte(20));
        let source = &**serializer;

        let mut parser = BinaryParser::from(source);
        assert_eq!(parser.take_tag(true), Some(Tag::Byte(20)));
    }
}
