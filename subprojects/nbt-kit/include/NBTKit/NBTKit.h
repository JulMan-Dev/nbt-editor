#pragma once

#import <Foundation/Foundation.h>

#include <stdint.h>

@class NBTByte, NBTModifiableByte, NBTShort, NBTModifiableShort, NBTInt, NBTModifiableInt, NBTLong, NBTModifiableLong,
    NBTFloat, NBTModifiableFloat, NBTDouble, NBTModifiableDouble, NBTByteArray, NBTModifiableByteArray, NBTString,
    NBTModifiableString, NBTList, NBTModifiableList, NBTCompound, NBTModifiableCompound, NBTIntArray, NBTModifiableIntArray,
    NBTLongArray, NBTModifiableLongArray;
@class NBTBaseTag;

@interface NBTBinaryParser : NSObject

+ (nonnull instancetype)allocWithZone:(null_unspecified struct _NSZone *)zone;
+ (nonnull instancetype)newWith:(nonnull NSData *)data;

- (nonnull instancetype)initWith:(nonnull NSData *)data;
- (void)dealloc;
- (nonnull NSData *)data;
- (uintptr_t)start;
- (nullable NBTByte *)takeByte;
- (nullable NBTShort *)takeShort;
- (nullable NBTInt *)takeInt;
- (nullable NBTLong *)takeLong;
- (nullable NBTFloat *)takeFloat;
- (nullable NBTDouble *)takeDouble;
- (nullable NBTByteArray *)takeByteArray;
- (nullable NBTString *)takeString;
- (nullable NBTList *)takeList;
- (nullable NBTCompound *)takeCompound;
- (nullable NBTIntArray *)takeIntArray;
- (nullable NBTLongArray *)takeLongArray;
- (nullable NBTBaseTag *)takeTag;

@end

@protocol NBTTag <NSObject, NSCopying, NSMutableCopying>
- (BOOL)isByte;
- (BOOL)isShort;
- (BOOL)isInt;
- (BOOL)isLong;
- (BOOL)isFloat;
- (BOOL)isDouble;
- (BOOL)isByteArray;
- (BOOL)isString;
- (BOOL)isList;
- (BOOL)isCompound;
- (BOOL)isIntArray;
- (BOOL)isLongArray;
@end

@interface NBTBaseTag : NSObject <NBTTag>
- (BOOL)isByte;
- (BOOL)isShort;
- (BOOL)isInt;
- (BOOL)isLong;
- (BOOL)isFloat;
- (BOOL)isDouble;
- (BOOL)isByteArray;
- (BOOL)isString;
- (BOOL)isList;
- (BOOL)isCompound;
- (BOOL)isIntArray;
- (BOOL)isLongArray;
@end

//#region NBT Byte

@interface NBTByte : NBTBaseTag

+ (nonnull instancetype)allocWithZone:(null_unspecified struct _NSZone *)zone;
+ (nonnull instancetype)newWith:(int8_t)value;

- (nonnull instancetype)initWith:(int8_t)value;
- (void)dealloc;
- (int8_t)value;

@end

@interface NBTModifiableByte : NBTByte

- (void)setValue:(int8_t)value;

@end

//#endregion

//#region NBT Short

@interface NBTShort : NBTBaseTag

+ (nonnull instancetype)allocWithZone:(null_unspecified struct _NSZone *)zone;
+ (nonnull instancetype)newWith:(int16_t)value;

- (nonnull instancetype)initWith:(int16_t)value;
- (void)dealloc;
- (int16_t)value;

@end

@interface NBTModifiableShort : NBTShort

- (void)setValue:(int16_t)value;

@end

//#endregion

//#region NBT Int

@interface NBTInt : NBTBaseTag

+ (nonnull instancetype)allocWithZone:(null_unspecified struct _NSZone *)zone;
+ (nonnull instancetype)newWith:(int32_t)value;

- (nonnull instancetype)initWith:(int32_t)value;
- (void)dealloc;
- (int32_t)value;

@end

@interface NBTModifiableInt : NBTInt

- (void)setValue:(int32_t)value;

@end

//#endregion

//#region NBT Long

@interface NBTLong : NBTBaseTag

+ (nonnull instancetype)allocWithZone:(null_unspecified struct _NSZone *)zone;
+ (nonnull instancetype)newWith:(int64_t)value;

- (nonnull instancetype)initWith:(int64_t)value;
- (void)dealloc;
- (int64_t)value;

@end

@interface NBTModifiableLong : NBTLong

- (void)setValue:(int64_t)value;

@end

//#endregion

//#region NBT Float

@interface NBTFloat : NBTBaseTag

+ (nonnull instancetype)allocWithZone:(null_unspecified struct _NSZone *)zone;
+ (nonnull instancetype)newWith:(float)value;

- (nonnull instancetype)initWith:(float)value;
- (void)dealloc;
- (float)value;

@end

@interface NBTModifiableFloat : NBTFloat

- (void)setValue:(float)value;

@end

//#endregion

//#region NBT Double

@interface NBTDouble : NBTBaseTag

+ (nonnull instancetype)allocWithZone:(null_unspecified struct _NSZone *)zone;
+ (nonnull instancetype)newWith:(double)value;

- (nonnull instancetype)initWith:(double)value;
- (void)dealloc;
- (double)value;

@end

@interface NBTModifiableDouble : NBTDouble

- (void)setValue:(double)value;

@end

//#endregion

//#region NBT Byte Array

@interface NBTByteArray : NBTBaseTag

+ (nonnull instancetype)allocWithZone:(null_unspecified struct _NSZone *)zone;
+ (nonnull instancetype)newWith:(nonnull NSData *)value;

- (nonnull instancetype)initWith:(nonnull NSData *)value;
- (void)dealloc;
- (nonnull NSData *)data;

@end

@interface NBTModifiableByteArray : NBTByteArray

- (void)setData:(nonnull NSData *)value;
- (nonnull NSMutableData *)mutableData;

@end

//#endregion

//#region NBT String

@interface NBTString : NBTBaseTag

+ (nonnull instancetype)allocWithZone:(null_unspecified struct _NSZone *)zone;
+ (nonnull instancetype)newWith:(nonnull NSString *)value;

- (nonnull instancetype)initWith:(nonnull NSString *)value;
- (void)dealloc;
- (nonnull NSString *)value;

@end

@interface NBTModifiableString : NBTString

- (void)setValue:(nonnull NSString *)value;
- (nonnull NSMutableString *)mutableData;

@end

//#endregion

//#region NBT List

@interface NBTList : NBTBaseTag

+ (nonnull instancetype)allocWithZone:(null_unspecified struct _NSZone *)zone;
+ (nonnull instancetype)newWith:(nonnull NSArray<NBTBaseTag *> *)value;

- (nonnull instancetype)initWith:(nonnull NSArray<NBTBaseTag *> *)value;
- (void)dealloc;
- (nonnull NSArray<NBTBaseTag *> *)value;
- (nullable Class)type;

@end

@interface NBTModifiableList : NBTList

- (void)setValue:(nonnull NSArray<NBTBaseTag *> *)value;
- (nonnull NSMutableArray<NBTBaseTag *> *)mutableValue;

@end

//#endregion

//#region NBT Compound

@interface NBTCompound : NBTBaseTag

+ (nonnull instancetype)allocWithZone:(null_unspecified struct _NSZone *)zone;
+ (nonnull instancetype)newWith:(nonnull NSDictionary<NSString *, NBTBaseTag *> *)value;

- (nonnull instancetype)initWith:(nonnull NSDictionary<NSString *, NBTBaseTag *> *)value;
- (void)dealloc;
- (nonnull NSDictionary<NSString *, NBTBaseTag *> *)value;

@end

@interface NBTModifiableCompound : NBTCompound

- (void)setValue:(nonnull NSDictionary<NSString *, NBTBaseTag *> *)value;
- (nonnull NSMutableDictionary<NSString *, NBTBaseTag *> *)mutableValue;

@end

//#endregion

//#region NBT Int Array

@interface NBTIntArray : NBTBaseTag

+ (nonnull instancetype)allocWithZone:(null_unspecified struct _NSZone *)zone;
+ (nonnull instancetype)newWith:(nullable const int32_t *)data
                            len:(uintptr_t)len;

- (nonnull instancetype)initWith:(nullable const int32_t *)data
                             len:(uintptr_t)len;
- (void)dealloc;
- (nullable const int32_t *)ptr;
- (uintptr_t)len;

@end

@interface NBTModifiableIntArray : NBTIntArray

/// Copies the integers from the `data` buffer, replacing the old in-place buffer.
- (void)setData:(nullable const int32_t *)data
            len:(uintptr_t)len;
- (void)pushOne:(int32_t)value;
- (void)insertOne:(int32_t)value
               at:(uintptr_t)index;
- (void)pushCopy:(nullable const uint32_t *)data
             len:(uintptr_t)len;
- (void)insertCopy:(nullable const uint32_t *)data
               len:(uintptr_t)len
                at:(uintptr_t)index;
- (nullable int32_t *)mutablePtr;
- (uintptr_t)cap;

@end

//#endregion

//#region NBT Long Array

@interface NBTLongArray : NBTBaseTag

+ (nonnull instancetype)allocWithZone:(null_unspecified struct _NSZone *)zone;
+ (nonnull instancetype)newWith:(nullable const int64_t *)data
                            len:(uintptr_t)len;

- (nonnull instancetype)initWith:(nullable const int64_t *)data
                             len:(uintptr_t)len;
- (void)dealloc;
- (nullable const int64_t *)ptr;
- (uintptr_t)len;

@end

@interface NBTModifiableLongArray : NBTLongArray

/// Copies the integers from the `data` buffer, replacing the old in-place buffer.
- (void)setData:(nullable const int64_t *)data
            len:(uintptr_t)len;
- (void)pushOne:(int64_t)value;
- (void)insertOne:(int64_t)value
               at:(uintptr_t)index;
- (void)pushCopy:(nullable const uint64_t *)data
             len:(uintptr_t)len;
- (void)insertCopy:(nullable const uint64_t *)data
               len:(uintptr_t)len
                at:(uintptr_t)index;
- (nullable int64_t *)mutablePtr;
- (uintptr_t)cap;

@end

//#endregion
