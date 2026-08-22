extern crate objc2;

use crate::binary::BinaryParser;
use crate::writer::BinarySerializer;
use crate::kind::{ByteArray, Compound, IntArray, List, LongArray, Tag};
use crate::traits::*;
use alloc::collections::BTreeMap;
use alloc::string::String;
use core::slice::from_raw_parts;
use core::{mem, ptr};
use objc2::ffi::class_addMethod;
use objc2::rc::Retained;
use objc2::runtime::{AnyClass, AnyObject, Sel};
use objc2::{class, msg_send, sel};
use objc2_foundation::{NSArray, NSData, NSDictionary, NSException, NSMallocException, NSMutableData, NSString};

#[ctor::ctor(unsafe)] static OBJC_NBT_TAG: &'static AnyClass = class!(NBTBaseTag);

#[ctor::ctor(unsafe)] static OBJC_NBT_BYTE: &'static AnyClass = class!(NBTByte);

#[inline(always)]
fn objc_byte(v: i8) -> Retained<AnyObject> {
    // SAFETY: +newWith: is implemented on NBTByte
    unsafe { msg_send![*OBJC_NBT_BYTE, newWith:v] }
}

/// # Safety
///
/// The retained value must be a subclass of NBTByte.
#[inline(always)]
unsafe fn rust_byte(v: &Retained<AnyObject>) -> i8 {
    // SAFETY: -value is implemented on NBTByte
    unsafe { msg_send![Retained::as_ptr(v), value] }
}

#[ctor::ctor(unsafe)] static OBJC_NBT_SHORT: &'static AnyClass = class!(NBTShort);

#[inline(always)]
fn objc_short(v: i16) -> Retained<AnyObject> {
    // SAFETY: +newWith: is implemented on NBTShort
    unsafe { msg_send![*OBJC_NBT_SHORT, newWith:v] }
}

/// # Safety
///
/// The retained value must be a subclass of NBTShort.
#[inline(always)]
unsafe fn rust_short(v: &Retained<AnyObject>) -> i16 {
    // SAFETY: -value is implemented on NBTShort
    unsafe { msg_send![Retained::as_ptr(v), value] }
}

#[ctor::ctor(unsafe)] static OBJC_NBT_INT: &'static AnyClass = class!(NBTInt);

#[inline(always)]
fn objc_int(v: i32) -> Retained<AnyObject> {
    // SAFETY: +newWith: is implemented on NBTInt
    unsafe { msg_send![*OBJC_NBT_INT, newWith:v] }
}

/// # Safety
///
/// The retained value must be a subclass of NBTInt.
#[inline(always)]
unsafe fn rust_int(v: &Retained<AnyObject>) -> i32 {
    // SAFETY: -value is implemented on NBTInt
    unsafe { msg_send![Retained::as_ptr(v), value] }
}

#[ctor::ctor(unsafe)] static OBJC_NBT_LONG: &'static AnyClass = class!(NBTLong);

#[inline(always)]
fn objc_long(v: i64) -> Retained<AnyObject> {
    // SAFETY: +newWith: is implemented on NBTLong
    unsafe { msg_send![*OBJC_NBT_LONG, newWith:v] }
}

/// # Safety
///
/// The retained value must be a subclass of NBTLong.
#[inline(always)]
unsafe fn rust_long(v: &Retained<AnyObject>) -> i64 {
    // SAFETY: -value is implemented on NBTLong
    unsafe { msg_send![Retained::as_ptr(v), value] }
}

#[ctor::ctor(unsafe)] static OBJC_NBT_FLOAT: &'static AnyClass = class!(NBTFloat);

#[inline(always)]
fn objc_float(v: f32) -> Retained<AnyObject> {
    // SAFETY: +newWith: is implemented on NBTFloat
    unsafe { msg_send![*OBJC_NBT_FLOAT, newWith:v] }
}

/// # Safety
///
/// The retained value must be a subclass of NBTFloat.
#[inline(always)]
unsafe fn rust_float(v: &Retained<AnyObject>) -> f32 {
    // SAFETY: -value is implemented on NBTFloat
    unsafe { msg_send![Retained::as_ptr(v), value] }
}

#[ctor::ctor(unsafe)] static OBJC_NBT_DOUBLE: &'static AnyClass = class!(NBTDouble);

#[inline(always)]
fn objc_double(v: f64) -> Retained<AnyObject> {
    // SAFETY: +newWith: is implemented on NBTDouble
    unsafe { msg_send![*OBJC_NBT_DOUBLE, newWith:v] }
}

/// # Safety
///
/// The retained value must be a subclass of NBTDouble.
#[inline(always)]
unsafe fn rust_double(v: &Retained<AnyObject>) -> f64 {
    // SAFETY: -value is implemented on NBTDouble
    unsafe { msg_send![Retained::as_ptr(v), value] }
}

#[ctor::ctor(unsafe)] static OBJC_NBT_BYTE_ARRAY: &'static AnyClass = class!(NBTByteArray);

#[inline(always)]
fn objc_byte_array(data: &[u8]) -> Retained<AnyObject> {
    let ns_data = NSData::with_bytes(data);
    // SAFETY: +newWith: is implemented on NBTByteArray
    unsafe { msg_send![*OBJC_NBT_BYTE_ARRAY, newWith:Retained::as_ptr(&ns_data)] }
}

/// # Safety
///
/// The retained value must be a subclass of NBTByteArray.
#[inline(always)]
unsafe fn rust_byte_array(v: &Retained<AnyObject>) -> ByteArray {
    // SAFETY: -data is implemented on NBTByteArray
    let ns_data: Retained<NSData> = unsafe { msg_send![Retained::as_ptr(v), data] };
    // SAFETY: -bytes is implemented on NSData
    let ptr: *const u8 = unsafe { msg_send![Retained::as_ptr(&ns_data), bytes] };
    let len = ns_data.len();

    if ptr.is_null() {
        ByteArray::new(Box::new([]))
    } else {
        // SAFETY: ptr is nonnull and aligned
        let slice = unsafe { from_raw_parts(ptr, len) };
        // SAFETY: i8 and u8 are the same
        let bytes: &[i8] = unsafe { mem::transmute(slice) };
        ByteArray::new(bytes.to_vec().into_boxed_slice())
    }
}

#[ctor::ctor(unsafe)] static OBJC_NBT_STRING: &'static AnyClass = class!(NBTString);

#[inline(always)]
fn objc_string(s: &str) -> Retained<AnyObject> {
    let ns_string = NSString::from_str(s);
    // SAFETY: +newWith: is implemented on NBTString
    unsafe { msg_send![*OBJC_NBT_STRING, newWith:Retained::as_ptr(&ns_string)] }
}

/// # Safety
///
/// The retained value must be a subclass of NBTString.
#[inline(always)]
unsafe fn rust_string(v: &Retained<AnyObject>) -> String {
    // SAFETY: -value is implemented on NBTString
    let ns_string: Retained<NSString> = unsafe { msg_send![Retained::as_ptr(v), value] };
    ns_string.to_string()
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

/// # Safety
///
/// The retained value must be a subclass of NBTList.
#[inline(always)]
unsafe fn rust_list(v: &Retained<AnyObject>) -> List {
    // SAFETY: -type is implemented on NBTList
    let element_class: *const AnyClass = unsafe { msg_send![Retained::as_ptr(v), type] };
    // SAFETY: -value is implemented on NBTList
    let ns_array: Retained<NSArray<AnyObject>> = unsafe { msg_send![Retained::as_ptr(v), value] };

    if element_class.is_null() || ns_array.len() == 0 {
        return List::Empty;
    }

    // SAFETY: the pointer may not be null and is well-aligned.
    let element_class = unsafe { &*element_class };

    if ptr::eq(element_class, *OBJC_NBT_BYTE) {
        // SAFETY: all items are NBTByte, we checked the class pointer.
        List::Byte(ns_array.iter().map(|obj| unsafe { rust_byte(&obj) }).collect())
    } else if ptr::eq(element_class, *OBJC_NBT_SHORT) {
        // SAFETY: all items are NBTShort, we checked the class pointer.
        List::Short(ns_array.iter().map(|obj| unsafe { rust_short(&obj) }).collect())
    } else if ptr::eq(element_class, *OBJC_NBT_INT) {
        // SAFETY: all items are NBTInt, we checked the class pointer.
        List::Int(ns_array.iter().map(|obj| unsafe { rust_int(&obj) }).collect())
    } else if ptr::eq(element_class, *OBJC_NBT_LONG) {
        // SAFETY: all items are NBTLong, we checked the class pointer.
        List::Long(ns_array.iter().map(|obj| unsafe { rust_long(&obj) }).collect())
    } else if ptr::eq(element_class, *OBJC_NBT_FLOAT) {
        // SAFETY: all items are NBTFloat, we checked the class pointer.
        List::Float(ns_array.iter().map(|obj| unsafe { rust_float(&obj) }).collect())
    } else if ptr::eq(element_class, *OBJC_NBT_DOUBLE) {
        // SAFETY: all items are NBTDouble, we checked the class pointer.
        List::Double(ns_array.iter().map(|obj| unsafe { rust_double(&obj) }).collect())
    } else if ptr::eq(element_class, *OBJC_NBT_BYTE_ARRAY) {
        // SAFETY: all items are NBTByteArray, we checked the class pointer.
        List::ByteArray(ns_array.iter().map(|obj| unsafe { rust_byte_array(&obj) }).collect())
    } else if ptr::eq(element_class, *OBJC_NBT_STRING) {
        // SAFETY: all items are NBTString, we checked the class pointer.
        List::String(ns_array.iter().map(|obj| unsafe { rust_string(&obj) }).collect())
    } else if ptr::eq(element_class, *OBJC_NBT_LIST) {
        // SAFETY: all items are NBTList, we checked the class pointer.
        List::List(ns_array.iter().map(|obj| unsafe { rust_list(&obj) }).collect())
    } else if ptr::eq(element_class, *OBJC_NBT_COMPOUND) {
        // SAFETY: all items are NBTCompound, we checked the class pointer.
        List::Compound(ns_array.iter().map(|obj| unsafe { rust_compound(&obj) }).collect())
    } else if ptr::eq(element_class, *OBJC_NBT_INT_ARRAY) {
        // SAFETY: all items are NBTIntArray, we checked the class pointer.
        List::IntArray(ns_array.iter().map(|obj| unsafe { rust_int_array(&obj) }).collect())
    } else if ptr::eq(element_class, *OBJC_NBT_LONG_ARRAY) {
        // SAFETY: all items are NBTLongArray, we checked the class pointer.
        List::LongArray(ns_array.iter().map(|obj| unsafe { rust_long_array(&obj) }).collect())
    } else {
        List::Empty
    }
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

/// # Safety
///
/// The retained value must be a subclass of NBTCompound.
#[inline(always)]
unsafe fn rust_compound(v: &Retained<AnyObject>) -> Compound {
    // SAFETY: -value is implemented on NBTCompound
    let ns_dict: Retained<NSDictionary<NSString, AnyObject>> = unsafe { msg_send![Retained::as_ptr(v), value] };

    let mut tree = BTreeMap::new();

    for key in ns_dict.keys() {
        if let Some(x) = ns_dict.objectForKey(&key) {
            tree.insert(key.to_string(), unsafe { rust_tag(&x) });
        }
    }

    Compound::new(tree)
}

#[ctor::ctor(unsafe)] static OBJC_NBT_INT_ARRAY: &'static AnyClass = class!(NBTIntArray);

#[inline(always)]
fn objc_int_array(array: &[i32]) -> Retained<AnyObject> {
    // SAFETY: +newWith:len: is implemented on NBTIntArray
    unsafe { msg_send![*OBJC_NBT_INT_ARRAY, newWith:array.as_ptr(), len:array.len()] }
}

/// # Safety
///
/// The retained value must be a subclass of NBTIntArray.
#[inline(always)]
unsafe fn rust_int_array(v: &Retained<AnyObject>) -> IntArray {
    // SAFETY: -ptr is implemented on NBTIntArray
    let ptr: *const i32 = unsafe { msg_send![Retained::as_ptr(v), ptr] };
    // SAFETY: -len is implemented on NBTIntArray
    let len: usize = unsafe { msg_send![Retained::as_ptr(v), len] };

    if ptr.is_null() {
        IntArray::new(Box::new([]))
    } else {
        // SAFETY: ptr is nonnull and aligned
        let slice = unsafe { from_raw_parts(ptr, len) };
        IntArray::new(slice.to_vec().into_boxed_slice())
    }
}

#[ctor::ctor(unsafe)] static OBJC_NBT_LONG_ARRAY: &'static AnyClass = class!(NBTLongArray);

#[inline(always)]
fn objc_long_array(array: &[i64]) -> Retained<AnyObject> {
    // SAFETY: +newWith:len: is implemented on NBTLongArray
    unsafe { msg_send![*OBJC_NBT_LONG_ARRAY, newWith:array.as_ptr(), len:array.len()] }
}

/// # Safety
///
/// The retained value must be a subclass of NBTLongArray.
#[inline(always)]
unsafe fn rust_long_array(v: &Retained<AnyObject>) -> LongArray {
    // SAFETY: -ptr is implemented on NBTLongArray
    let ptr: *const i64 = unsafe { msg_send![Retained::as_ptr(v), ptr] };
    // SAFETY: -len is implemented on NBTLongArray
    let len: usize = unsafe { msg_send![Retained::as_ptr(v), len] };

    if ptr.is_null() {
        LongArray::new(Box::new([]))
    } else {
        // SAFETY: ptr is nonnull and aligned
        let slice = unsafe { from_raw_parts(ptr, len) };
        LongArray::new(slice.to_vec().into_boxed_slice())
    }
}

#[ctor::ctor(unsafe)] static OBJC_NBT_PARSER: &'static AnyClass = class!(NBTParser);

#[ctor::ctor(unsafe)] static OBJC_NBT_SERIALIZER: &'static AnyClass = class!(NBTBinarySerializer);

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

/// # Safety
///
/// The retained value must be a subclass of NBTBaseTag.
#[inline(always)]
unsafe fn rust_tag(v: &Retained<AnyObject>) -> Tag {
    // SAFETY: -isKindOfClass: is implemented on NSObject
    let is_byte: bool = unsafe { msg_send![Retained::as_ptr(v), isKindOfClass:*OBJC_NBT_BYTE] };
    if is_byte {
        return Tag::Byte(unsafe { rust_byte(v) });
    }

    let is_short: bool = unsafe { msg_send![Retained::as_ptr(v), isKindOfClass:*OBJC_NBT_SHORT] };
    if is_short {
        return Tag::Short(unsafe { rust_short(v) });
    }

    let is_int: bool = unsafe { msg_send![Retained::as_ptr(v), isKindOfClass:*OBJC_NBT_INT] };
    if is_int {
        return Tag::Int(unsafe { rust_int(v) });
    }

    let is_long: bool = unsafe { msg_send![Retained::as_ptr(v), isKindOfClass:*OBJC_NBT_LONG] };
    if is_long {
        return Tag::Long(unsafe { rust_long(v) });
    }

    let is_float: bool = unsafe { msg_send![Retained::as_ptr(v), isKindOfClass:*OBJC_NBT_FLOAT] };
    if is_float {
        return Tag::Float(unsafe { rust_float(v) });
    }

    let is_double: bool = unsafe { msg_send![Retained::as_ptr(v), isKindOfClass:*OBJC_NBT_DOUBLE] };
    if is_double {
        return Tag::Double(unsafe { rust_double(v) });
    }

    let is_byte_array: bool = unsafe { msg_send![Retained::as_ptr(v), isKindOfClass:*OBJC_NBT_BYTE_ARRAY] };
    if is_byte_array {
        return Tag::ByteArray(unsafe { rust_byte_array(v) });
    }

    let is_string: bool = unsafe { msg_send![Retained::as_ptr(v), isKindOfClass:*OBJC_NBT_STRING] };
    if is_string {
        return Tag::String(unsafe { rust_string(v) });
    }

    let is_list: bool = unsafe { msg_send![Retained::as_ptr(v), isKindOfClass:*OBJC_NBT_LIST] };
    if is_list {
        return Tag::List(unsafe { rust_list(v) });
    }

    let is_compound: bool = unsafe { msg_send![Retained::as_ptr(v), isKindOfClass:*OBJC_NBT_COMPOUND] };
    if is_compound {
        return Tag::Compound(unsafe { rust_compound(v) });
    }

    let is_int_array: bool = unsafe { msg_send![Retained::as_ptr(v), isKindOfClass:*OBJC_NBT_INT_ARRAY] };
    if is_int_array {
        return Tag::IntArray(unsafe { rust_int_array(v) });
    }

    let is_long_array: bool = unsafe { msg_send![Retained::as_ptr(v), isKindOfClass:*OBJC_NBT_LONG_ARRAY] };
    if is_long_array {
        return Tag::LongArray(unsafe { rust_long_array(v) });
    }

    Tag::Empty
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

    macro_rules! impl_write {
        ($this:expr => |$serializer:ident| $write_expr:expr) => {{
            let mutable_data: Retained<NSMutableData> = unsafe { msg_send![Retained::as_ptr(&$this), mutableData] };
            let mut $serializer = BinarySerializer::new(Vec::new());
            $write_expr;
            unsafe { msg_send![Retained::as_ptr(&mutable_data), appendBytes:$serializer.as_ptr(), length:$serializer.len()] };
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

    let write_encoding = c"v@:@";

    unsafe {
        let is_ok = class_addMethod(ptr::from_ref(*OBJC_NBT_SERIALIZER).cast_mut(), sel!(writeByte:), {
            extern "C-unwind" fn write(this: Retained<AnyObject>, _cmd: Sel, value: Retained<AnyObject>) {
                impl_write!(this => |serializer| serializer.write_byte(unsafe { rust_byte(&value) }));
            }

            mem::transmute(write as extern "C-unwind" fn (_, _, _))
        }, write_encoding.as_ptr()).as_bool();
        assert!(is_ok);
    }

    unsafe {
        let is_ok = class_addMethod(ptr::from_ref(*OBJC_NBT_SERIALIZER).cast_mut(), sel!(writeShort:), {
            extern "C-unwind" fn write(this: Retained<AnyObject>, _cmd: Sel, value: Retained<AnyObject>) {
                impl_write!(this => |serializer| serializer.write_short(unsafe { rust_short(&value) }));
            }

            mem::transmute(write as extern "C-unwind" fn (_, _, _))
        }, write_encoding.as_ptr()).as_bool();
        assert!(is_ok);
    }

    unsafe {
        let is_ok = class_addMethod(ptr::from_ref(*OBJC_NBT_SERIALIZER).cast_mut(), sel!(writeInt:), {
            extern "C-unwind" fn write(this: Retained<AnyObject>, _cmd: Sel, value: Retained<AnyObject>) {
                impl_write!(this => |serializer| serializer.write_int(unsafe { rust_int(&value) }))
            }

            mem::transmute(write as extern "C-unwind" fn (_, _, _))
        }, write_encoding.as_ptr()).as_bool();
        assert!(is_ok);
    }

    unsafe {
        let is_ok = class_addMethod(ptr::from_ref(*OBJC_NBT_SERIALIZER).cast_mut(), sel!(writeLong:), {
            extern "C-unwind" fn write(this: Retained<AnyObject>, _cmd: Sel, value: Retained<AnyObject>) {
                impl_write!(this => |serializer| serializer.write_long(unsafe { rust_long(&value) }))
            }

            mem::transmute(write as extern "C-unwind" fn (_, _, _))
        }, write_encoding.as_ptr()).as_bool();
        assert!(is_ok);
    }

    unsafe {
        let is_ok = class_addMethod(ptr::from_ref(*OBJC_NBT_SERIALIZER).cast_mut(), sel!(writeFloat:), {
            extern "C-unwind" fn write(this: Retained<AnyObject>, _cmd: Sel, value: Retained<AnyObject>) {
                impl_write!(this => |serializer| serializer.write_float(unsafe { rust_float(&value) }))
            }

            mem::transmute(write as extern "C-unwind" fn (_, _, _))
        }, write_encoding.as_ptr()).as_bool();
        assert!(is_ok);
    }

    unsafe {
        let is_ok = class_addMethod(ptr::from_ref(*OBJC_NBT_SERIALIZER).cast_mut(), sel!(writeDouble:), {
            extern "C-unwind" fn write(this: Retained<AnyObject>, _cmd: Sel, value: Retained<AnyObject>) {
                impl_write!(this => |serializer| serializer.write_double(unsafe { rust_double(&value) }))
            }

            mem::transmute(write as extern "C-unwind" fn (_, _, _))
        }, write_encoding.as_ptr()).as_bool();
        assert!(is_ok);
    }

    unsafe {
        let is_ok = class_addMethod(ptr::from_ref(*OBJC_NBT_SERIALIZER).cast_mut(), sel!(writeByteArray:), {
            extern "C-unwind" fn write(this: Retained<AnyObject>, _cmd: Sel, value: Retained<AnyObject>) {
                impl_write!(this => |serializer| serializer.write_byte_array(unsafe { rust_byte_array(&value) }))
            }

            mem::transmute(write as extern "C-unwind" fn (_, _, _))
        }, write_encoding.as_ptr()).as_bool();
        assert!(is_ok);
    }

    unsafe {
        let is_ok = class_addMethod(ptr::from_ref(*OBJC_NBT_SERIALIZER).cast_mut(), sel!(writeString:), {
            extern "C-unwind" fn write(this: Retained<AnyObject>, _cmd: Sel, value: Retained<AnyObject>) {
                impl_write!(this => |serializer| serializer.write_string(unsafe { rust_string(&value) }))
            }

            mem::transmute(write as extern "C-unwind" fn (_, _, _))
        }, write_encoding.as_ptr()).as_bool();
        assert!(is_ok);
    }

    unsafe {
        let is_ok = class_addMethod(ptr::from_ref(*OBJC_NBT_SERIALIZER).cast_mut(), sel!(writeList:), {
            extern "C-unwind" fn write(this: Retained<AnyObject>, _cmd: Sel, value: Retained<AnyObject>) {
                impl_write!(this => |serializer| serializer.write_list(unsafe { rust_list(&value) }))
            }

            mem::transmute(write as extern "C-unwind" fn (_, _, _))
        }, write_encoding.as_ptr()).as_bool();
        assert!(is_ok);
    }

    unsafe {
        let is_ok = class_addMethod(ptr::from_ref(*OBJC_NBT_SERIALIZER).cast_mut(), sel!(writeCompound:), {
            extern "C-unwind" fn write(this: Retained<AnyObject>, _cmd: Sel, value: Retained<AnyObject>) {
                impl_write!(this => |serializer| serializer.write_compound(unsafe { rust_compound(&value) }))
            }

            mem::transmute(write as extern "C-unwind" fn (_, _, _))
        }, write_encoding.as_ptr()).as_bool();
        assert!(is_ok);
    }

    unsafe {
        let is_ok = class_addMethod(ptr::from_ref(*OBJC_NBT_SERIALIZER).cast_mut(), sel!(writeIntArray:), {
            extern "C-unwind" fn write(this: Retained<AnyObject>, _cmd: Sel, value: Retained<AnyObject>) {
                impl_write!(this => |serializer| serializer.write_int_array(unsafe { rust_int_array(&value) }))
            }

            mem::transmute(write as extern "C-unwind" fn (_, _, _))
        }, write_encoding.as_ptr()).as_bool();
        assert!(is_ok);
    }

    unsafe {
        let is_ok = class_addMethod(ptr::from_ref(*OBJC_NBT_SERIALIZER).cast_mut(), sel!(writeLongArray:), {
            extern "C-unwind" fn write(this: Retained<AnyObject>, _cmd: Sel, value: Retained<AnyObject>) {
                impl_write!(this => |serializer| serializer.write_long_array(unsafe { rust_long_array(&value) }))
            }

            mem::transmute(write as extern "C-unwind" fn (_, _, _))
        }, write_encoding.as_ptr()).as_bool();
        assert!(is_ok);
    }

    unsafe {
        let is_ok = class_addMethod(ptr::from_ref(*OBJC_NBT_SERIALIZER).cast_mut(), sel!(writeTag:), {
            extern "C-unwind" fn write(this: Retained<AnyObject>, _cmd: Sel, value: Retained<AnyObject>) {
                impl_write!(this => |serializer| serializer.write_tag(unsafe { rust_tag(&value) }))
            }

            mem::transmute(write as extern "C-unwind" fn (_, _, _))
        }, write_encoding.as_ptr()).as_bool();
        assert!(is_ok);
    }
}
