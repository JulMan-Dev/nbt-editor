#import "NBTKit/NBTKit.h"
#include <Foundation/Foundation.h>
#import <objc/runtime.h>

#include <stdlib.h>

// NBTBinaryParser is implemented by the Rust crate.
// Other types are implemented here using Objective-C because it is
// easier than writing Objective-C-like code in Rust.

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wincomplete-implementation"

@implementation NBTBinaryParser {
    NSData * _Nonnull data;
    uintptr_t start;
}

+ (instancetype)newWith:(NSData *)data
{
    return [[self alloc] initWith:data];
}

- (instancetype)initWith:(NSData *)d
{
    self->data = d;
    self->start = 0;
    return self;
}

- (NSData *)data
{
    return self->data;
}

- (uintptr_t)start
{
    return self->start;
}

@end

@implementation NBTBinarySerializer {
    NSMutableData * _Nonnull data;
}

+ (instancetype)newWith:(NSData *)data
{
    return [[self alloc] initWith:data];
}

+ (instancetype)newWithMutable:(NSMutableData *)data
{
    return [[self alloc] initWithMutable:data];
}

- (instancetype)initWith:(NSData *)d
{
    self->data = [[NSMutableData alloc] initWithData:d];
    return self;
}

- (instancetype)initWithMutable:(NSMutableData *)d
{
    self->data = [d mutableCopy];
    return self;
}

- (NSData *)data
{
    return [NSData dataWithData:self->data];
}

- (NSMutableData *)mutableData
{
    return self->data;
}

- (id)copyWithZone:(NSZone *)zone
{
    // Immutable copy
    NBTBinarySerializer * copy = [NBTBinarySerializer allocWithZone:zone];
    copy->data = [self->data mutableCopy];
    return copy;
}

- (id)mutableCopyWithZone:(NSZone *)zone
{
    // Mutable copy
    NBTBinarySerializer * copy = [NBTBinarySerializer allocWithZone:zone];
    copy->data = [self->data mutableCopy];
    return copy;
}

@end

#pragma clang diagnostic pop

#define __IMPL_TEST(name, class1) - (BOOL)name { return [self isKindOfClass:[class1 class]]; }

@implementation NBTBaseTag

__IMPL_TEST(isByte, NBTByte)
__IMPL_TEST(isShort, NBTShort)
__IMPL_TEST(isInt, NBTInt)
__IMPL_TEST(isLong, NBTLong)
__IMPL_TEST(isFloat, NBTFloat)
__IMPL_TEST(isDouble, NBTDouble)
__IMPL_TEST(isByteArray, NBTByteArray)
__IMPL_TEST(isString, NBTString)
__IMPL_TEST(isList, NBTList)
__IMPL_TEST(isCompound, NBTCompound)
__IMPL_TEST(isIntArray, NBTIntArray)
__IMPL_TEST(isLongArray, NBTLongArray)

- (id)copyWithZone:(NSZone *)zone
{
    return nil;
}

- (id)mutableCopyWithZone:(NSZone *)zone
{
    return nil;
}

@end

@implementation NBTByte {
    @protected int8_t value;
}

+ (instancetype)newWith:(int8_t)value
{
    return [[self alloc] initWith:value];
}

- (instancetype)initWith:(int8_t)v
{
    self = [self init];
    self->value = v;
    return self;
}

- (int8_t)value
{
    return self->value;
}

- (id)copyWithZone:(NSZone *)zone
{
    if (![self isKindOfClass:[NBTModifiableByte class]])
    {
        return [self retain];
    }
    else
    {
        return [NBTByte newWith:self->value];
    }
}

- (id)mutableCopyWithZone:(NSZone *)zone
{
    return [NBTModifiableByte newWith:self->value];
}

@end

@implementation NBTModifiableByte

- (void)setValue:(int8_t)v
{
    self->value = v;
}

@end

@implementation NBTShort {
    @protected int16_t value;
}

+ (instancetype)newWith:(int16_t)value
{
    return [[self alloc] initWith:value];
}

- (instancetype)initWith:(int16_t)v
{
    self->value = v;
    return self;
}

- (int16_t)value
{
    return self->value;
}

- (id)copyWithZone:(NSZone *)zone
{
    if (![self isKindOfClass:[NBTModifiableShort class]])
    {
        return [self retain];
    }
    else
    {
        return [NBTShort newWith:self->value];
    }
}

- (id)mutableCopyWithZone:(NSZone *)zone
{
    return [NBTModifiableShort newWith:self->value];
}

@end

@implementation NBTModifiableShort

- (void)setValue:(int16_t)v
{
    self->value = v;
}

@end

@implementation NBTInt {
    @protected int32_t value;
}

+ (instancetype)newWith:(int32_t)value
{
    return [[self alloc] initWith:value];
}

- (instancetype)initWith:(int32_t)v
{
    self->value = v;
    return self;
}

- (int32_t)value
{
    return self->value;
}

- (id)copyWithZone:(NSZone *)zone
{
    if (![self isKindOfClass:[NBTModifiableInt class]])
    {
        return [self retain];
    }
    else
    {
        return [NBTInt newWith:self->value];
    }
}

- (id)mutableCopyWithZone:(NSZone *)zone
{
    return [NBTModifiableInt newWith:self->value];
}

@end

@implementation NBTModifiableInt

- (void)setValue:(int32_t)v
{
    self->value = v;
}

@end

@implementation NBTLong {
    @protected int64_t value;
}

+ (instancetype)newWith:(int64_t)value
{
    return [[self alloc] initWith:value];
}

- (instancetype)initWith:(int64_t)v
{
    self->value = v;
    return self;
}

- (int64_t)value
{
    return self->value;
}

- (id)copyWithZone:(NSZone *)zone
{
    if (![self isKindOfClass:[NBTModifiableLong class]])
    {
        return [self retain];
    }
    else
    {
        return [NBTLong newWith:self->value];
    }
}

- (id)mutableCopyWithZone:(NSZone *)zone
{
    return [NBTModifiableLong newWith:self->value];
}

@end

@implementation NBTModifiableLong

- (void)setValue:(int64_t)v
{
    self->value = v;
}

@end

@implementation NBTFloat {
    @protected float value;
}

+ (instancetype)newWith:(float)value
{
    return [[self alloc] initWith:value];
}

- (instancetype)initWith:(float)v
{
    self->value = v;
    return self;
}

- (float)value
{
    return self->value;
}

- (id)copyWithZone:(NSZone *)zone
{
    if (![self isKindOfClass:[NBTModifiableFloat class]])
    {
        return [self retain];
    }
    else
    {
        return [NBTFloat newWith:self->value];
    }
}

- (id)mutableCopyWithZone:(NSZone *)zone
{
    return [NBTModifiableFloat newWith:self->value];
}

@end

@implementation NBTModifiableFloat

- (void)setValue:(float)v
{
    self->value = v;
}

@end

@implementation NBTDouble {
    @protected double value;
}

+ (instancetype)newWith:(double)value
{
    return [[self alloc] initWith:value];
}

- (instancetype)initWith:(double)v
{
    self->value = v;
    return self;
}

- (double)value
{
    return self->value;
}

- (id)copyWithZone:(NSZone *)zone
{
    if (![self isKindOfClass:[NBTModifiableDouble class]])
    {
        return [self retain];
    }
    else
    {
        return [NBTDouble newWith:self->value];
    }
}

- (id)mutableCopyWithZone:(NSZone *)zone
{
    return [NBTModifiableDouble newWith:self->value];
}

@end

@implementation NBTModifiableDouble

- (void)setValue:(double)v
{
    self->value = v;
}

@end

@implementation NBTByteArray {
    NSData * _Nonnull data;
}

+ (instancetype)newWith:(NSData *)value
{
    return [[self alloc] initWith:value];
}

- (instancetype)initWith:(NSData *)value
{
    self->data = value;
    return self;
}

- (void)dealloc
{
    [self->data release];
    [super dealloc];
}

- (nonnull NSData *)data
{
    return self->data;
}

- (id)copyWithZone:(NSZone *)zone
{
    if (![self isKindOfClass:[NBTModifiableByteArray class]])
    {
        return [self retain];
    }
    else
    {
        return [NBTByteArray newWith:[self->data copy]];
    }
}

- (id)mutableCopyWithZone:(NSZone *)zone
{
    return [NBTModifiableByteArray newWith:[self->data mutableCopy]];
}

@end

@implementation NBTModifiableByteArray {
    NSMutableData * _Nonnull data;
}

- (instancetype)initWith:(NSData *)value
{
    self->data = [NSMutableData dataWithData:value];
    return self;
}

- (void)dealloc
{
    [self->data release];
    [super dealloc];
}

- (void)setData:(NSData *)value
{
    [self->data setData:value];
}

- (NSMutableData *)mutableData
{
    return self->data;
}

@end

@implementation NBTString {
    NSString * _Nonnull data;
}

+ (instancetype)newWith:(NSString *)value
{
    return [[self alloc] initWith:value];
}

- (instancetype)initWith:(NSString *)value
{
    self->data = value;
    return self;
}

- (void)dealloc
{
    [self->data release];
    [super dealloc];
}

- (NSString *)value
{
    return self->data;
}

- (id)copyWithZone:(NSZone *)zone
{
    if (![self isKindOfClass:[NBTModifiableString class]])
    {
        return [self retain];
    }
    else
    {
        return [NBTString newWith:[self->data copy]];
    }
}

- (id)mutableCopyWithZone:(NSZone *)zone
{
    return [NBTModifiableString newWith:self->data];
}

@end

@implementation NBTModifiableString {
    NSMutableString * _Nonnull data;
}

- (instancetype)initWith:(NSString *)value
{
    self->data = [value mutableCopy];
    return self;
}

- (void)dealloc
{
    [self->data release];
    [super dealloc];
}

- (void)setValue:(NSString *)value
{
    [self initWith:value];
}

- (nonnull NSMutableString *)mutableData
{
    return self->data;
}

@end

@implementation NBTList {
    @protected Class _Nullable type;
    NSArray<NBTBaseTag *> * _Nonnull data;
}

+ (instancetype)newWith:(NSArray<NBTBaseTag *> *)value
{
    // we are not doing the copy here because Mutable variant would copy,
    // copying twice, ineffient.
    return [[self alloc] initWith:value];
}

- (instancetype)initWith:(NSArray<NBTBaseTag *> *)value
{
    self->type = [NBTList validateArray:value];
    self->data = value;
    return self;
}

+ (nullable Class)validateArray:(NSArray<NBTBaseTag *> *)value
{
    Class type = nil;

    for (NBTBaseTag * tag in value)
    {
        if (!type)
        {
            if ([tag isByte]) type = [NBTByte class];
            else if ([tag isShort]) type = [NBTShort class];
            else if ([tag isInt]) type = [NBTInt class];
            else if ([tag isLong]) type = [NBTLong class];
            else if ([tag isFloat]) type = [NBTFloat class];
            else if ([tag isDouble]) type = [NBTDouble class];
            else if ([tag isByteArray]) type = [NBTByteArray class];
            else if ([tag isString]) type = [NBTString class];
            else if ([tag isList]) type = [NBTList class];
            else if ([tag isCompound]) type = [NBTCompound class];
            else if ([tag isIntArray]) type = [NBTIntArray class];
            else if ([tag isLongArray]) type = [NBTLongArray class];
            else
            {
                @throw [NSException exceptionWithName:NSInvalidArgumentException
                                               reason:@"Passed array doesn't contain NBT data"
                                             userInfo:nil];
            }
        }
        else if (![tag isKindOfClass:type])
        {
            return nil;
        }
    }

    return type;
}

- (void)dealloc
{
    [self->data release];
    [super dealloc];
}

- (NSArray<NBTBaseTag *> *)value
{
    return self->data;
}

- (Class)type
{
    return self->type;
}

- (id)copyWithZone:(NSZone *)zone
{
    if (![self isKindOfClass:[NBTModifiableList class]])
    {
        return [self retain];
    }
    else
    {
        return [NBTList newWith:[[NSArray init] initWithArray:self->data
                                                    copyItems:YES]];
    }
}

- (id)mutableCopyWithZone:(NSZone *)zone
{
    return [NBTModifiableList newWith:[self->data retain]];
}

@end

@implementation NBTModifiableList {
    NSMutableArray<NBTBaseTag *> * _Nonnull data;
}

- (instancetype)initWith:(NSArray<NBTBaseTag *> *)value
{
    self->type = [NBTList validateArray:value];

    // coping every elements as mutable, to be fully mutable
    NBTBaseTag ** elements = calloc([value count], sizeof(id));
    for (uintptr_t i = 0; i < [value count]; i++)
    {
        elements[i] = [value[i] mutableCopy];
    }

    self->data = [[NSMutableArray alloc] initWithObjects:elements
                                                   count:[value count]];
    free(elements);
    [value release]; // following Apple's convention, init take ownership
    return self;
}

- (void)dealloc
{
    [self->data release];
    [super dealloc];
}

- (void)setValue:(NSArray<NBTBaseTag *> *)value
{
    [self initWith:value];
}

- (NSMutableArray<NBTBaseTag *> *)mutableValue
{
    return self->data;
}

@end

@implementation NBTCompound {
    NSDictionary<NSString *, NBTBaseTag *> * _Nonnull data;
}

+ (instancetype)newWith:(NSDictionary<NSString *, NBTBaseTag *> *)value
{
    return [[self alloc] initWith:value];
}

- (instancetype)initWith:(NSDictionary<NSString *, NBTBaseTag *> *)value
{
    self->data = value;
    return self;
}

- (void)dealloc
{
    [self->data release];
    [super dealloc];
}

- (NSDictionary<NSString *,NBTBaseTag *> *)value
{
    return self->data;
}

- (id)copyWithZone:(NSZone *)zone
{
    if (![self isKindOfClass:[NBTModifiableCompound class]])
    {
        return [self retain];
    }
    else
    {
        return [NBTCompound newWith:[[NSDictionary alloc] initWithDictionary:self->data
                                                                   copyItems:YES]];
    }
}

- (id)mutableCopyWithZone:(NSZone *)zone
{
    return [NBTModifiableCompound newWith:[self->data retain]];
}

@end

@implementation NBTModifiableCompound {
    NSMutableDictionary<NSString *, NBTBaseTag *> * _Nonnull data;
}

- (instancetype)initWith:(NSDictionary<NSString *, NBTBaseTag *> *)value
{
    // making a mutable deep copy
    self->data = [[NSMutableDictionary alloc] initWithCapacity:[value count]];

    for (NSString * i in value)
    {
        self->data[i] = [value[i] mutableCopy];
    }

    [value release];
    return self;
}

- (void)dealloc
{
    [self->data release];
    [super dealloc];
}

- (void)setValue:(NSDictionary<NSString *, NBTBaseTag *> *)value
{
    [self setValue:value];
}

- (NSMutableDictionary<NSString *,NBTBaseTag *> *)mutableValue
{
    return self->data;
}

@end

@implementation NBTIntArray {
    @protected int32_t * _Nullable data;
    @protected uintptr_t len;
}

+ (instancetype)newWith:(const int32_t *)data
                    len:(uintptr_t)len
{
    return [[self alloc] initWith:data
                              len:len];
}

- (instancetype)initWith:(const int32_t *)d
                     len:(uintptr_t)l
{
    if (!d || !l)
    {
        self->data = nil;
        self->len = l;
        return self;
    }

    int32_t * ptr = calloc(l, sizeof(int32_t));
    if (!ptr)
    {
        @throw [NSException exceptionWithName:NSMallocException
                                       reason:nil
                                     userInfo:nil];
    }

    memcpy(ptr, d, l * sizeof(int32_t));
    self->data = ptr;
    self->len = l;
    return self;
}

- (void)dealloc
{
    if (self->data)
    {
        free(self->data);
    }

    [super dealloc];
}

- (const int32_t *)ptr
{
    return self->data;
}

- (uintptr_t)len
{
    return len;
}

- (id)copyWithZone:(NSZone *)zone
{
    if (![self isKindOfClass:[NBTModifiableIntArray class]])
    {
        return [self retain];
    }
    else
    {
        // coping buffer
        return [NBTIntArray newWith:self->data
                                len:self->len];
    }
}

- (id)mutableCopyWithZone:(NSZone *)zone
{
    return [NBTModifiableIntArray newWith:self->data
                                      len:self->len];
}

@end

@implementation NBTModifiableIntArray {
    uintptr_t cap;
}

- (void)dataAllocate:(uintptr_t)moreElements
{
    if (moreElements == 0)
    {
        return;
    }

    if (self->cap == 0 || !self->data)
    {
        self->cap = moreElements;

        self->data = calloc(self->cap, sizeof(int32_t));
    }
    else
    {
        while (self->cap < moreElements)
        {
            self->cap *= 2;
        }

        self->data = realloc(self->data, self->cap * sizeof(int32_t));
    }

    if (!self->data)
    {
        @throw [NSException exceptionWithName:NSMallocException
                                       reason:nil
                                     userInfo:nil];
    }
}

- (void)setData:(const int32_t *)d
            len:(uintptr_t)l
{
    if (!d && !l)
    {
        return;
    }

    if (self->cap < l)
    {
        [self dataAllocate:l - self->cap];
    }

    memcpy(self->data, d, l * sizeof(int32_t));
    self->len = l;
}

- (void)pushOne:(int32_t)value
{
    [self dataAllocate:1];

    self->data[self->len] = value;
    self->len++;
}

- (void)insertOne:(int32_t)value
               at:(uintptr_t)index
{
    [self dataAllocate:1];

    if (self->len != index)
    {
        memmove(self->data + index + 1, self->data + index, sizeof(int32_t));
    }

    self->data[index] = value;
    self->len++;
}

- (void)pushCopy:(const uint32_t *)d
             len:(uintptr_t)l
{
    [self dataAllocate:l];

    if (!d || !l)
    {
        return;
    }

    memcpy(self->data + self->len, d, l * sizeof(int32_t));
    self->len += l;
}

- (void)insertCopy:(const uint32_t *)d
               len:(uintptr_t)l
                at:(uintptr_t)index
{
    [self dataAllocate:l];

    if (!d || !l)
    {
        return;
    }

    if (self->len != index)
    {
        memmove(self->data + index + l, self->data + index, l * sizeof(int32_t));
    }

    memcpy(self->data + index, d, l * sizeof(int32_t));
    self->len += l;
}

- (int32_t *)mutablePtr
{
    return self->data;
}

- (uintptr_t)cap
{
    return self->cap;
}

@end

@implementation NBTLongArray {
    @protected int64_t * _Nullable data;
    @protected uintptr_t len;
}

+ (instancetype)newWith:(const int64_t *)data
                    len:(uintptr_t)len
{
    return [[self alloc] initWith:data
                              len:len];
}

- (instancetype)initWith:(const int64_t *)d
                     len:(uintptr_t)l
{
    if (!d || !l)
    {
        self->data = nil;
        self->len = l;
        return self;
    }

    int64_t * ptr = calloc(l, sizeof(int64_t));
    if (!ptr)
    {
        @throw [NSException exceptionWithName:NSMallocException
                                       reason:nil
                                     userInfo:nil];
    }

    memcpy(ptr, d, l * sizeof(int64_t));
    self->data = ptr;
    self->len = l;
    return self;
}

- (void)dealloc
{
    if (self->data)
    {
        free(self->data);
    }

    [super dealloc];
}

- (const int64_t *)ptr
{
    return self->data;
}

- (uintptr_t)len
{
    return len;
}

- (id)copyWithZone:(NSZone *)zone
{
    if (![self isKindOfClass:[NBTModifiableLongArray class]])
    {
        return [self retain];
    }
    else
    {
        // coping buffer
        return [NBTLongArray newWith:self->data
                                 len:self->len];
    }
}

- (id)mutableCopyWithZone:(NSZone *)zone
{
    return [NBTModifiableLongArray newWith:self->data
                                       len:self->len];
}

@end

@implementation NBTModifiableLongArray {
    uintptr_t cap;
}

- (void)dataAllocate:(uintptr_t)moreElements
{
    if (moreElements == 0)
    {
        return;
    }

    if (self->cap == 0 || !self->data)
    {
        self->cap = moreElements;

        self->data = calloc(self->cap, sizeof(int64_t));
    }
    else
    {
        while (self->cap < moreElements)
        {
            self->cap *= 2;
        }

        self->data = realloc(self->data, self->cap * sizeof(int64_t));
    }

    if (!self->data)
    {
        @throw [NSException exceptionWithName:NSMallocException
                                       reason:nil
                                     userInfo:nil];
    }
}

- (void)setData:(const int64_t *)d
            len:(uintptr_t)l
{
    if (!d && !l)
    {
        return;
    }

    if (self->cap < l)
    {
        [self dataAllocate:l - self->cap];
    }

    memcpy(self->data, d, l * sizeof(int64_t));
    self->len = l;
}

- (void)pushOne:(int64_t)value
{
    [self dataAllocate:1];

    self->data[self->len] = value;
    self->len++;
}

- (void)insertOne:(int64_t)value
               at:(uintptr_t)index
{
    [self dataAllocate:1];

    if (self->len != index)
    {
        memmove(self->data + index + 1, self->data + index, sizeof(int64_t));
    }

    self->data[index] = value;
    self->len++;
}

- (void)pushCopy:(const uint64_t *)d
             len:(uintptr_t)l
{
    [self dataAllocate:l];

    if (!d || !l)
    {
        return;
    }

    memcpy(self->data + self->len, d, l * sizeof(int64_t));
    self->len += l;
}

- (void)insertCopy:(const uint64_t *)d
               len:(uintptr_t)l
                at:(uintptr_t)index
{
    [self dataAllocate:l];

    if (!d || !l)
    {
        return;
    }

    if (self->len != index)
    {
        memmove(self->data + index + l, self->data + index, l * sizeof(int64_t));
    }

    memcpy(self->data + index, d, l * sizeof(int64_t));
    self->len += l;
}

- (int64_t *)mutablePtr
{
    return self->data;
}

- (uintptr_t)cap
{
    return self->cap;
}

@end
