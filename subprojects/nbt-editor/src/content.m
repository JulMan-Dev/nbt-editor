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

- (instancetype)initWithContentsOfURL:(NSURL *)url
                               ofType:(NSString *)typeName
                                error:(NSError * _Nullable * _Nullable)outError
{
    NSData *data = [[NSData alloc] initWithContentsOfURL:url
                                                 options:NSDataReadingMappedIfSafe
                                                   error:outError];
    
    if (!data)
    {
        // NSData should have already filled the error pointer
        return nil;
    }
    
    NBTBinaryParser *parser = [NBTBinaryParser newWith:data];
    NBTBaseTag *tag = [parser takeTag];
    
    if (!tag)
    {
        *outError = [NSError errorWithDomain:NSCocoaErrorDomain
                                        code:NSFileReadCorruptFileError
                                    userInfo:nil];
        return nil;
    }
    
    self->tag = tag;
    return self;
}

+ (BOOL)autosavesInPlace
{
    // when NBT editing, I don't think users expect files to be saved automatically
    // (NBT editing may be destructive)
    return NO;
}

- (void)makeWindowControllers
{
    NBTWindowController *windowController = [[NBTWindowController alloc] init];

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
    NBTBaseTag *tag = [parser takeTag];
    
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

@implementation NBTWindowContent {
    NBTBaseTag *tag;
}

- (instancetype)initWithTag:(NBTBaseTag *)t
{
    self->tag = t;
    return self;
}

@end
