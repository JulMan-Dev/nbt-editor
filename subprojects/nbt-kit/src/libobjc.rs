extern crate objc2;

use crate::binary::BinaryParser;
use crate::kind::{Compound, List, Tag};
use crate::parser::*;
use core::slice::from_raw_parts;
use core::{mem, ptr};
use objc2::ffi::class_addMethod;
use objc2::rc::Retained;
use objc2::runtime::{AnyClass, AnyObject, Sel};
use objc2::{class, msg_send, sel};
use objc2_foundation::{NSArray, NSData, NSDictionary, NSException, NSMallocException, NSString};

#[ctor::ctor(unsafe)] static OBJC_NBT_TAG: &'static AnyClass = class!(NBTBaseTag);

#[ctor::ctor(unsafe)] static OBJC_NBT_BYTE: &'static AnyClass = class!(NBTByte);

#[inline(always)]
fn objc_byte(v: i8) -> Retained<AnyObject> {
    // SAFETY: +newWith: is implemented on NBTByte
    unsafe { msg_send![*OBJC_NBT_BYTE, newWith:v] }
}

#[ctor::ctor(unsafe)] static OBJC_NBT_SHORT: &'static AnyClass = class!(NBTShort);

#[inline(always)]
fn objc_short(v: i16) -> Retained<AnyObject> {
    // SAFETY: +newWith: is implemented on NBTShort
    unsafe { msg_send![*OBJC_NBT_SHORT, newWith:v] }
}

#[ctor::ctor(unsafe)] static OBJC_NBT_INT: &'static AnyClass = class!(NBTInt);

#[inline(always)]
fn objc_int(v: i32) -> Retained<AnyObject> {
    // SAFETY: +newWith: is implemented on NBTInt
    unsafe { msg_send![*OBJC_NBT_INT, newWith:v] }
}

#[ctor::ctor(unsafe)] static OBJC_NBT_LONG: &'static AnyClass = class!(NBTLong);

#[inline(always)]
fn objc_long(v: i64) -> Retained<AnyObject> {
    // SAFETY: +newWith: is implemented on NBTLong
    unsafe { msg_send![*OBJC_NBT_LONG, newWith:v] }
}
#[ctor::ctor(unsafe)] static OBJC_NBT_FLOAT: &'static AnyClass = class!(NBTFloat);

#[inline(always)]
fn objc_float(v: f32) -> Retained<AnyObject> {
    // SAFETY: +newWith: is implemented on NBTFloat
    unsafe { msg_send![*OBJC_NBT_FLOAT, newWith:v] }
}

#[ctor::ctor(unsafe)] static OBJC_NBT_DOUBLE: &'static AnyClass = class!(NBTDouble);

#[inline(always)]
fn objc_double(v: f64) -> Retained<AnyObject> {
    // SAFETY: +newWith: is implemented on NBTDouble
    unsafe { msg_send![*OBJC_NBT_DOUBLE, newWith:v] }
}

#[ctor::ctor(unsafe)] static OBJC_NBT_BYTE_ARRAY: &'static AnyClass = class!(NBTByteArray);

#[inline(always)]
fn objc_byte_array(data: &[u8]) -> Retained<AnyObject> {
    let ns_data = NSData::with_bytes(data);
    // SAFETY: +newWith: is implemented on NBTByteArray
    unsafe { msg_send![*OBJC_NBT_BYTE_ARRAY, newWith:Retained::as_ptr(&ns_data)] }
}

#[ctor::ctor(unsafe)] static OBJC_NBT_STRING: &'static AnyClass = class!(NBTString);

#[inline(always)]
fn objc_string(s: &str) -> Retained<AnyObject> {
    let ns_string = NSString::from_str(s);
    // SAFETY: +newWith: is implemented on NBTString
    unsafe { msg_send![*OBJC_NBT_STRING, newWith:Retained::as_ptr(&ns_string)] }
}

#[ctor::ctor(unsafe)] static OBJC_NBT_LIST: &'static AnyClass = class!(NBTList);

#[inline(always)]
fn objc_list(list: List) -> Retained<AnyObject> {
    let items: Vec<Retained<AnyObject>> = match list {
        List::Empty => vec![],
        List::Byte(bytes) => bytes.into_iter().map(objc_byte).collect(),
        List::Short(shorts) => shorts.into_iter().map(objc_short).collect(),
        List::Int(ints) => ints.into_iter().map(objc_int).collect(),
        List::Long(longs) => longs.into_iter().map(objc_long).collect(),
        List::Float(floats) => floats.into_iter().map(objc_float).collect(),
        List::Double(doubles) => doubles.into_iter().map(objc_double).collect(),
        List::ByteArray(arrays) => arrays.into_iter().map(|arr| {
            let bytes: &[i8] = &arr;
            // SAFETY: i8 and u8 are the same
            objc_byte_array(unsafe { mem::transmute(bytes) })
        }).collect(),
        List::String(strings) => strings.into_iter().map(|s| objc_string(&s)).collect(),
        List::List(lists) => lists.into_iter().map(objc_list).collect(),
        List::Compound(compounds) => compounds.into_iter().map(objc_compound).collect(),
        List::IntArray(ints) => ints.into_iter().map(|a| objc_int_array(&a)).collect(),
        List::LongArray(longs) => longs.into_iter().map(|a| objc_long_array(&a)).collect(),
    };

    let array_ptr = NSArray::from_retained_slice(&items);
    // SAFETY: NBTList implements +newWith:
    unsafe { msg_send![*OBJC_NBT_LIST, newWith:Retained::as_ptr(&array_ptr)] }
}

#[ctor::ctor(unsafe)] static OBJC_NBT_COMPOUND: &'static AnyClass = class!(NBTCompound);

#[inline(always)]
fn objc_compound(compound: Compound) -> Retained<AnyObject> {
    let (keys, values): (Vec<_>, Vec<_>) = compound
        .into_inner()
        .into_iter()
        .map(|(k, v)| (objc_string(&k).downcast::<NSString>().ok().unwrap(), objc_tag(v)))
        .unzip();

    // SAFETY: Retained<T> and &T share the same layout. Retained<T> cannot be null and
    //         are always well-aligned.
    let keys_slice: &[&NSString] = unsafe { mem::transmute(&keys[..]) };

    let ns_dict = NSDictionary::from_retained_objects(keys_slice, &values);
    // SAFETY: +newWith is implemented on NBTCompound
    unsafe { msg_send![*OBJC_NBT_COMPOUND, newWith:Retained::as_ptr(&ns_dict)] }
}

#[ctor::ctor(unsafe)] static OBJC_NBT_INT_ARRAY: &'static AnyClass = class!(NBTIntArray);

#[inline(always)]
fn objc_int_array(array: &[i32]) -> Retained<AnyObject> {
    // SAFETY: +newWith:len: is implemented on NBTIntArray
    unsafe { msg_send![*OBJC_NBT_INT_ARRAY, newWith:array.as_ptr(), len:array.len()] }
}

#[ctor::ctor(unsafe)] static OBJC_NBT_LONG_ARRAY: &'static AnyClass = class!(NBTLongArray);

#[inline(always)]
fn objc_long_array(array: &[i64]) -> Retained<AnyObject> {
    // SAFETY: +newWith:len: is implemented on NBTLongArray
    unsafe { msg_send![*OBJC_NBT_LONG_ARRAY, newWith:array.as_ptr(), len:array.len()] }
}

#[ctor::ctor(unsafe)] static OBJC_NBT_PARSER: &'static AnyClass = class!(NBTParser);

#[inline(always)]
fn objc_tag(tag: Tag) -> Retained<AnyObject> {
    match tag {
        Tag::Empty => {
            unsafe {
                NSException::new(
                    NSMallocException,
                    None,
                    None,
                ).unwrap().raise()
            }
        }
        Tag::Byte(v) => objc_byte(v),
        Tag::Short(v) => objc_short(v),
        Tag::Int(v) => objc_int(v),
        Tag::Long(v) => objc_long(v),
        Tag::Float(v) => objc_float(v),
        Tag::Double(v) => objc_double(v),
        // SAFETY: i8 and u8 are the same
        Tag::ByteArray(arr) => objc_byte_array(unsafe { mem::transmute(&arr[..]) }),
        Tag::String(s) => objc_string(&s),
        Tag::List(list) => objc_list(list),
        Tag::Compound(compound) => objc_compound(compound),
        Tag::IntArray(arr) => objc_int_array(&arr),
        Tag::LongArray(arr) => objc_long_array(&arr),
    }
}

#[ctor::ctor(unsafe)]
unsafe fn load_objc_classes() {
    unsafe fn get_parser<'data>(this: *const AnyObject) -> BinaryParser<'data> {
        // SAFETY: -(NSData *)data is implemented on NBTBinaryParser
        let data: Retained<NSData> = unsafe { msg_send![this, data] };
        // SAFETY: -(uintptr_t)start is implemented on NBTBinaryParser
        let start: usize = unsafe { msg_send![this, start] };

        // SAFETY: -(const void *)bytes is implemented on NSData
        let ptr: *const u8 = unsafe { msg_send![Retained::as_ptr(&data), bytes] };
        let len = data.len();

        if ptr.is_null() {
            BinaryParser::from_ref(&[])
        } else {
            // SAFETY: ptr is nonnull and aligned (always because aligment of u8 is 1)
            let slice = unsafe { from_raw_parts(ptr, len) };

            BinaryParser::from_ref(&slice[start..])
        }
    }

    macro_rules! impl_binding {
        ($this:expr => |mut $id:ident| $expr:expr) => {{
            let this = $this;
            // SAFETY: $this should be a &(NBTBinaryParser *) equivalent
            let original = unsafe { get_parser(Retained::as_ptr(this)) };
            let mut $id = original.clone();

            let value = $expr;

            let original: &[u8] = &original;
            let after: &[u8] = &$id;

            if let Some(first) = after.first() {
                let to_add = original.element_offset(first).unwrap();

                // SAFETY: start is an ivar of NBTBinaryParser
                unsafe {
                    let start: *mut usize = OBJC_NBT_PARSER.instance_variable(c"start")
                        .unwrap().load_ptr(&this);
                    *start += to_add;
                }
            }

            value
        }};
    }

    let encoding = c"@@:";

    unsafe {
        let is_ok = class_addMethod(ptr::from_ref(*OBJC_NBT_PARSER).cast_mut(), sel!(takeByte), {
            extern "C-unwind" fn take(this: Retained<AnyObject>, _cmd: Sel) -> Option<Retained<AnyObject>> {
                impl_binding!(&this => |mut parser| Some(objc_byte(parser.take_byte()?)))
            }

            mem::transmute(take as extern "C-unwind" fn (_, _) -> _)
        }, encoding.as_ptr()).as_bool();
        assert!(is_ok);
    }

    unsafe {
        let is_ok = class_addMethod(ptr::from_ref(*OBJC_NBT_PARSER).cast_mut(), sel!(takeShort), {
            extern "C-unwind" fn take(this: Retained<AnyObject>, _cmd: Sel) -> Option<Retained<AnyObject>> {
                impl_binding!(&this => |mut parser| Some(objc_short(parser.take_short()?)))
            }

            mem::transmute(take as extern "C-unwind" fn (_, _) -> _)
        }, encoding.as_ptr()).as_bool();
        assert!(is_ok);
    }

    unsafe {
        let is_ok = class_addMethod(ptr::from_ref(*OBJC_NBT_PARSER).cast_mut(), sel!(takeInt), {
            extern "C-unwind" fn take(this: Retained<AnyObject>, _cmd: Sel) -> Option<Retained<AnyObject>> {
                impl_binding!(&this => |mut parser| Some(objc_int(parser.take_int()?)))
            }

            mem::transmute(take as extern "C-unwind" fn (_, _) -> _)
        }, encoding.as_ptr()).as_bool();
        assert!(is_ok);
    }

    unsafe {
        let is_ok = class_addMethod(ptr::from_ref(*OBJC_NBT_PARSER).cast_mut(), sel!(takeLong), {
            extern "C-unwind" fn take(this: Retained<AnyObject>, _cmd: Sel) -> Option<Retained<AnyObject>> {
                impl_binding!(&this => |mut parser| Some(objc_long(parser.take_long()?)))
            }

            mem::transmute(take as extern "C-unwind" fn (_, _) -> _)
        }, encoding.as_ptr()).as_bool();
        assert!(is_ok);
    }

    unsafe {
        let is_ok = class_addMethod(ptr::from_ref(*OBJC_NBT_PARSER).cast_mut(), sel!(takeFloat), {
            extern "C-unwind" fn take_byte(this: Retained<AnyObject>, _cmd: Sel) -> Option<Retained<AnyObject>> {
                impl_binding!(&this => |mut parser| Some(objc_float(parser.take_float()?)))
            }

            mem::transmute(take_byte as extern "C-unwind" fn (_, _) -> _)
        }, encoding.as_ptr()).as_bool();
        assert!(is_ok);
    }

    unsafe {
        let is_ok = class_addMethod(ptr::from_ref(*OBJC_NBT_PARSER).cast_mut(), sel!(takeDouble), {
            extern "C-unwind" fn take(this: Retained<AnyObject>, _cmd: Sel) -> Option<Retained<AnyObject>> {
                impl_binding!(&this => |mut parser| Some(objc_double(parser.take_double()?)))
            }

            mem::transmute(take as extern "C-unwind" fn (_, _) -> _)
        }, encoding.as_ptr()).as_bool();
        assert!(is_ok);
    }

    unsafe {
        let is_ok = class_addMethod(ptr::from_ref(*OBJC_NBT_PARSER).cast_mut(), sel!(takeByteArray), {
            extern "C-unwind" fn take(this: Retained<AnyObject>, _cmd: Sel) -> Option<Retained<AnyObject>> {
                impl_binding!(&this => |mut parser| {
                    let value = parser.take_byte_array()?;
                    // SAFETY: i8 and u8 are the same
                    let bytes: Box<[u8]> = unsafe { mem::transmute(value.into_inner()) };
                    Some(objc_byte_array(&bytes))
                })
            }

            mem::transmute(take as extern "C-unwind" fn (_, _) -> _)
        }, encoding.as_ptr()).as_bool();
        assert!(is_ok);
    }

    unsafe {
        let is_ok = class_addMethod(ptr::from_ref(*OBJC_NBT_PARSER).cast_mut(), sel!(takeString), {
            extern "C-unwind" fn take(this: Retained<AnyObject>, _cmd: Sel) -> Option<Retained<AnyObject>> {
                impl_binding!(&this => |mut parser| Some(objc_string(&parser.take_string()?)))
            }

            mem::transmute(take as extern "C-unwind" fn (_, _) -> _)
        }, encoding.as_ptr()).as_bool();
        assert!(is_ok);
    }

    unsafe {
        let is_ok = class_addMethod(ptr::from_ref(*OBJC_NBT_PARSER).cast_mut(), sel!(takeList), {
            extern "C-unwind" fn take(this: Retained<AnyObject>, _cmd: Sel) -> Option<Retained<AnyObject>> {
                impl_binding!(&this => |mut parser| Some(objc_list(parser.take_list()?)))
            }

            mem::transmute(take as extern "C-unwind" fn (_, _) -> _)
        }, encoding.as_ptr()).as_bool();
        assert!(is_ok);
    }

    unsafe {
        let is_ok = class_addMethod(ptr::from_ref(*OBJC_NBT_PARSER).cast_mut(), sel!(takeCompound), {
            extern "C-unwind" fn take(this: Retained<AnyObject>, _cmd: Sel) -> Option<Retained<AnyObject>> {
                impl_binding!(&this => |mut parser| Some(objc_compound(parser.take_compound()?)))
            }

            mem::transmute(take as extern "C-unwind" fn (_, _) -> _)
        }, encoding.as_ptr()).as_bool();
        assert!(is_ok);
    }

    unsafe {
        let is_ok = class_addMethod(ptr::from_ref(*OBJC_NBT_PARSER).cast_mut(), sel!(takeIntArray), {
            extern "C-unwind" fn take(this: Retained<AnyObject>, _cmd: Sel) -> Option<Retained<AnyObject>> {
                impl_binding!(&this => |mut parser| Some(objc_int_array(&**parser.take_int_array()?)))
            }

            mem::transmute(take as extern "C-unwind" fn (_, _) -> _)
        }, encoding.as_ptr()).as_bool();
        assert!(is_ok);
    }

    unsafe {
        let is_ok = class_addMethod(ptr::from_ref(*OBJC_NBT_PARSER).cast_mut(), sel!(takeLongArray), {
            extern "C-unwind" fn take(this: Retained<AnyObject>, _cmd: Sel) -> Option<Retained<AnyObject>> {
                impl_binding!(&this => |mut parser| Some(objc_long_array(&**parser.take_long_array()?)))
            }

            mem::transmute(take as extern "C-unwind" fn (_, _) -> _)
        }, encoding.as_ptr()).as_bool();
        assert!(is_ok);
    }

    unsafe {
        let is_ok = class_addMethod(ptr::from_ref(*OBJC_NBT_PARSER).cast_mut(), sel!(takeTag), {
            extern "C-unwind" fn take(this: Retained<AnyObject>, _cmd: Sel) -> Option<Retained<AnyObject>> {
                impl_binding!(&this => |mut parser| Some(objc_tag(parser.take_tag()?)))
            }

            mem::transmute(take as extern "C-unwind" fn (_, _) -> _)
        }, encoding.as_ptr()).as_bool();
        assert!(is_ok);
    }
}
