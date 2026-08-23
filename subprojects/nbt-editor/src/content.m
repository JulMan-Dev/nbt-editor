#import "content.h"
#import "controller.h"

@implementation NBTDocument {
    NBTBaseTag *tag;
}

- (instancetype)init
{
    self = [super init];
    if (self) {
        self->tag = nil;
    }
    return self;
}

- (NBTBaseTag *)tag
{
    return self->tag;
}

+ (BOOL)autosavesInPlace
{
    // when NBT editing, I don't think users expect files to be saved automatically
    // (NBT editing may be destructive)
    return NO;
}

- (void)makeWindowControllers
{
    NBTWindowController *windowController = [NBTWindowController new];

    if (windowController)
    {
        [self addWindowController:windowController];
        [windowController loadWindow];
        // AppKit cannot load the window for us :(
    }
}

- (NSData *)dataOfType:(NSString *)typeName
                 error:(NSError * _Nullable *)outError
{
    NBTBinarySerializer *serializer = [NBTBinarySerializer newWithMutable:[NSMutableData data]];
    [serializer writeTag:self->tag];
    NSData *data = [serializer mutableData];
    // mutableData so we borrow out the internal buffer of the serializer, avoiding to allocate a new buffer
    
    return data;
}

- (BOOL)readFromData:(NSData *)data
              ofType:(NSString *)typeName
               error:(NSError * _Nullable *)outError
{
    NBTBinaryParser *parser = [NBTBinaryParser newWith:data];
    NBTBaseTag *tag = [parser takeTag:YES];
    
    if (!tag)
    {
        *outError = [NSError errorWithDomain:NSCocoaErrorDomain
                                        code:NSFileReadCorruptFileError
                                    userInfo:nil];
        return NO;
    }
    
    self->tag = tag;
    return YES;
}

@end

@implementation NBTCollectionWrapper {
    NBTBaseTag *_Nullable _tag;
    __weak NBTCollectionWrapper *_Nullable _parent;
    NSArray<NBTCollectionWrapper *> *_children;
}

+ (instancetype)wrapperWithTag:(NBTBaseTag *)tag
                        parent:(NBTCollectionWrapper *)parent
{
    return [[self alloc] initWithTag:tag
                              parent:parent];
}

- (instancetype)initWithTag:(NBTBaseTag *)t
                     parent:(NBTCollectionWrapper *)parent
{
    self->_tag = t;
    self->_parent = parent;
    [self initializeChildren];
    return self;
}

- (void)initializeChildren
{
    if (!self->_tag)
    {
        self->_children = @[];
        return;
    }
    
    if ([self->_tag isList])
    {
        NBTList *list = (id)self->_tag;
        
        NSArray<NBTBaseTag *> *tags = [list value];
        NSMutableArray<NBTCollectionWrapper *> *array = [NSMutableArray arrayWithCapacity:[tags count]];
        
        for (NBTBaseTag *tag in tags)
        {
            [array addObject:[NBTCollectionWrapper wrapperWithTag:tag
                                                           parent:self]];
        }
        
        self->_children = array;
    }
    else if ([self->_tag isCompound])
    {
        NBTCompound *compound = (id)self->_tag;
        
        NSDictionary<NSString *, NBTBaseTag *> *tags = [compound value];
        NSMutableArray<NBTCollectionWrapper *> *array = [NSMutableArray arrayWithCapacity:[tags count]];
        
        for (NSString *key in tags)
        {
            [array addObject:[NBTCollectionWrapper wrapperWithTag:tags[key]
                                                           parent:self]];
        }
        
        self->_children = array;
    }
    else
    {
        self->_children = @[];
    }
}

- (NSInteger)outlineView:(NSOutlineView *)outlineView
  numberOfChildrenOfItem:(id)item
{
    if (item)
    {
        NBTCollectionWrapper *it = item;
        
        return [it outlineView:outlineView
        numberOfChildrenOfItem:nil];
    }
    
    return [self->_children count];
}

- (id)outlineView:(NSOutlineView *)outlineView
            child:(NSInteger)index
           ofItem:(id)item
{
    if (item)
    {
        NBTCollectionWrapper *it = item;
        
        return [it outlineView:outlineView
                         child:index
                        ofItem:nil];
    }
    
    if (index >= [self->_children count])
    {
        return nil;
    }
    
    return self->_children[index];
}

- (BOOL)outlineView:(NSOutlineView *)outlineView
   isItemExpandable:(id)item
{
    NBTCollectionWrapper *wrapper = item;
    
    return [wrapper->_tag isList] || [wrapper->_tag isCompound];
}

- (id)outlineView:(NSOutlineView *)outlineView
objectValueForTableColumn:(NSTableColumn *)tableColumn
           byItem:(id)item
{
    if (!tableColumn || !item)
    {
        return nil;
    }

    NBTCollectionWrapper *this = item;
    NSString *columnId = [tableColumn identifier];
    
    NSCell *cell = nil;

    if ([columnId isEqual:@"NBTKey"])
    {
        cell = [self keyStringForWrapper:this];
    }
    else if ([columnId isEqual:@"NBTValue"])
    {
        cell = [self valueStringForWrapper:this];
    }
    else
    {
        return nil;
    }
    
    [cell setWraps:NO];
    return cell;
}

- (NSCell *)keyStringForWrapper:(NBTCollectionWrapper *)this
{
    // getting parent for acquiring the associated key
    if (!this->_parent)
    {
        return nil;
    }
    
    NBTBaseTag *tag = this->_parent->_tag;
    
    if ([tag isCompound])
    {
        NSDictionary<NSString *, NBTBaseTag *> *entries = [(NBTCompound *)tag value];
        
        for (NSString *key in entries)
        {
            // only testing against the point is fine
            if (entries[key] == this->_tag)
            {
                return [[NSCell alloc] initTextCell:key];
            }
        }
    }
    else if ([tag isList])
    {
        NSArray<NBTBaseTag *> *entries = [(NBTList *)tag value];
        
        for (uintptr_t i = 0; i < [entries count]; i++)
        {
            if (entries[i] == this->_tag)
            {
                return [[NSCell alloc] initTextCell:[NSString stringWithFormat:@"%ld", i]];
            }
        }
    }
    else
    {
        // no key, no ui information
        return nil;
    }
    
    @throw [NSException exceptionWithName:NSGenericException
                                   reason:[NSString stringWithFormat:@"cannot compute the key of %p", tag]
                                 userInfo:nil];
}

- (NSCell *)valueStringForWrapper:(NBTCollectionWrapper *)this
{
    NBTBaseTag *tag = this->_tag;
    NSString *value = nil;
    
    if ([tag isByte]) value = [NSString stringWithFormat:@"%d (byte)", [(NBTByte *)tag value]];
    else if ([tag isShort]) value = [NSString stringWithFormat:@"%d (short)", [(NBTShort *)tag value]];
    else if ([tag isInt]) value = [NSString stringWithFormat:@"%d (integer)", [(NBTInt *)tag value]];
    else if ([tag isLong]) value = [NSString stringWithFormat:@"%lld (long)", [(NBTLong *)tag value]];
    else if ([tag isFloat]) value = [NSString stringWithFormat:@"%f (float)", [(NBTFloat *)tag value]];
    else if ([tag isDouble]) value = [NSString stringWithFormat:@"%lf (double)", [(NBTDouble *)tag value]];
    else if ([tag isByteArray])
    {
        size_t i = [[(NBTByteArray *)tag data] length];
        value = [NSString stringWithFormat:@"<%lu byte%s>", i, i > 1 ? "s" : ""];
    }
    else if ([tag isString]) value = [NSString stringWithFormat:@"“%@”", [(NBTString *)tag value]];
    else if ([tag isList])
    {
        size_t i = [[(NBTList *)tag value] count];
        value = [NSString stringWithFormat:@"[%lu item%s …]", i, i > 1 ? "s" : ""];
    }
    else if ([tag isCompound])
    {
        size_t i = [[(NBTCompound *)tag value] count];
        value = [NSString stringWithFormat:@"{ %lu item%s … }", i, i > 1 ? "s" : ""];
    }
    else if ([tag isIntArray])
    {
        size_t i = [(NBTIntArray *)tag len];
        value = [NSString stringWithFormat:@"<%lu integer%s>", i, i > 1 ? "s" : ""];
    }
    else if ([tag isLongArray])
    {
        size_t i = [(NBTLongArray *)tag len];
        value = [NSString stringWithFormat:@"<%lu long%s>", i, i > 1 ? "s" : ""];
    }
    else
    {
        @throw [NSException exceptionWithName:NSInvalidArgumentException
                                       reason:@"unknown tag class"
                                     userInfo:nil];
    }
    
    return [[NSCell alloc] initTextCell:value];
}

- (NSCell *)outlineView:(NSOutlineView *)outlineView
 dataCellForTableColumn:(NSTableColumn *)tableColumn
                   item:(id)item
{
    if (!tableColumn)
    {
        return nil;
    }
    
    return [[NSTextFieldCell alloc] initTextCell:@""];
}

- (id)copyWithZone:(NSZone *)zone
{
    NBTCollectionWrapper *copy = [NBTCollectionWrapper allocWithZone:zone];
    copy->_tag = [self->_tag copyWithZone:zone];
    copy->_parent = self->_parent;
    copy->_children = [[NSArray alloc] initWithArray:self->_children
                                           copyItems:YES];
    for (NBTCollectionWrapper *child in copy->_children)
    {
        child->_parent = copy;
    }
    return copy;
}

- (id)mutableCopyWithZone:(NSZone *)zone
{
    NBTCollectionWrapper *copy = [NBTCollectionWrapper allocWithZone:zone];
    copy->_tag = [self->_tag mutableCopyWithZone:zone];
    copy->_parent = self->_parent;
    copy->_children = [[NSMutableArray alloc] initWithArray:self->_children
                                                  copyItems:YES];
    for (NBTCollectionWrapper *child in copy->_children)
    {
        child->_parent = copy;
    }
    return copy;
}

@end
