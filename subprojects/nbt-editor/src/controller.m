#import "controller.h"
#import "content.h"

@implementation NBTDocumentController

+ (NBTDocumentController *)sharedDocumentController
{
    static NBTDocumentController *INSTANCE = nil;
    
    if (!INSTANCE)
    {
        INSTANCE = [NBTDocumentController new];
    }
    
    return INSTANCE;
}

- (void)beginOpenPanel:(NSOpenPanel *)openPanel
              forTypes:(NSArray<NSString *> *)inTypes
     completionHandler:(void (^)(NSInteger))completionHandler
{
    openPanel.allowsMultipleSelection = NO;
    [super beginOpenPanel:openPanel
                 forTypes:inTypes
        completionHandler:completionHandler];
}

- (NSString *)defaultType
{
    return @"net.minecraft.NBT";
}

- (__kindof NSDocument *)makeUntitledDocumentOfType:(NSString *)typeName
                                              error:(NSError * _Nullable *)outError
{
    if (![typeName isEqual:[self defaultType]])
    {
        *outError = [NSError errorWithDomain:NSCocoaErrorDomain
                                        code:NSFeatureUnsupportedError
                                    userInfo:nil];
        return nil;
    }
    
    return [NBTDocument new];
}

@end

@implementation NBTWindowController {
    NBTDocument *document;
}

- (void)setDocument:(id)document
{
    if ([document isKindOfClass:[NBTDocument class]])
    {
        self->document = document;
    }
}

- (void)loadWindow
{
    if ([self window])
    {
        return;
    }
    
    NSRect frame = NSMakeRect(0, 0, 800, 600);
    NSWindow *window = [[NSWindow alloc] initWithContentRect:frame
                                                   styleMask:(NSWindowStyleMaskTitled |
                                                              NSWindowStyleMaskClosable |
                                                              NSWindowStyleMaskResizable |
                                                              NSWindowStyleMaskMiniaturizable)
                                                     backing:NSBackingStoreBuffered
                                                       defer:NO];

    window.title = @"Document";
    window.releasedWhenClosed = NO;

    [self setWindow:window];
    [window center];

    NSView *contentView = [[NSView alloc] initWithFrame:frame];
    window.contentView = contentView;

    [self windowDidLoad];
}

@end
