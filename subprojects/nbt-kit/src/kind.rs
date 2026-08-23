use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;

macro_rules! impl_tag_list {
    { $($id:ident$(($ty:ty))?),+ $(,)? } => {
        #[derive(PartialEq, Debug)]
        pub enum Tag {
            $($id$(($ty))?),+
        }

        #[derive(PartialEq, Debug)]
        pub enum List {
            $($id$((Box<[$ty]>))?),+
        }
    };
}

impl_tag_list! {
    Empty,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(ByteArray),
    String(String),
    List(List),
    Compound(Compound),
    IntArray(IntArray),
    LongArray(LongArray),
}

macro_rules! transparent {
    ($($vis:vis struct $id:ident = $ty:ty;)+) => {
        $(#[derive(PartialEq, Debug)]
        #[repr(transparent)]
        $vis struct $id($ty);

        impl $id {
            pub const fn new(v: $ty) -> Self {
                Self(v)
            }

            pub fn into_inner(self) -> $ty {
                self.0
            }

            pub const fn as_inner_ref(&self) -> &$ty {
                &self.0
            }

            pub const fn as_inner_mut(&mut self) -> &mut $ty {
                &mut self.0
            }
        }

        impl core::ops::Deref for $id {
            type Target = $ty;

            #[inline]
            fn deref(&self) -> &Self::Target {
                self.as_inner_ref()
            }
        }

        impl core::ops::DerefMut for $id {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.as_inner_mut()
            }
        }

        impl core::borrow::Borrow<$ty> for $id {
            #[inline]
            fn borrow(&self) -> &$ty {
                self.as_inner_ref()
            }
        }

        impl core::borrow::BorrowMut<$ty> for $id {
            #[inline]
            fn borrow_mut(&mut self) -> &mut $ty {
                self.as_inner_mut()
            }
        }

        impl From<$ty> for $id {
            #[inline]
            fn from(value: $ty) -> Self {
                Self::new(value)
            }
        }

        impl From<$id> for $ty {
            #[inline]
            fn from(value: $id) -> Self {
                value.into_inner()
            }
        })+
    };
}

transparent! {
    pub struct ByteArray = Box<[i8]>;
    pub struct Compound = BTreeMap<String, Tag>;
    pub struct IntArray = Box<[i32]>;
    pub struct LongArray = Box<[i64]>;
}
