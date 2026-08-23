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
    // this is linked to the document but a state of it, it is a view state
    NBTCollectionWrapper *wrapper;
}

- (void)setDocument:(id)document
{
    if ([document isKindOfClass:[NBTDocument class]])
    {
        self->document = document;
        self->wrapper = [NBTCollectionWrapper wrapperWithTag:[self->document tag]
                                                      parent:nil];
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
    [window setContentView:contentView];
    
    NSOutlineView *mainView = [[NSOutlineView alloc] initWithFrame:frame];
    [contentView addSubview:mainView];
    
    NSScrollView *scrollView = [[NSScrollView alloc] initWithFrame:frame];
    [scrollView setHasHorizontalScroller:YES];
    [scrollView setDocumentView:mainView];
    [contentView addSubview:scrollView];
    
    NSTableColumn *mainCol = [[NSTableColumn alloc] initWithIdentifier:@"NBTKey"];
    [mainCol setTitle:@"Key"];
    [mainView addTableColumn:mainCol];
    [mainView setOutlineTableColumn:mainCol];
    
    NSTableColumn *col = [[NSTableColumn alloc] initWithIdentifier:@"NBTValue"];
    [mainCol setTitle:@"Value"];
    [mainView addTableColumn:col];
    
    if (self->wrapper)
    {
        [mainView setDataSource:self->wrapper];
        [mainView setDelegate:self->wrapper];
        [mainView reloadData];
    }
    else
    {
        NSLog(@"may not set NSOutlineView dataSource, wrapper is (null)");
    }
    
    [self windowDidLoad];
}

@end
