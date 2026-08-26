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
    [openPanel setAllowsMultipleSelection:NO];
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
    NBTDocument *_document;
    // this is linked to the document but a state of it, it is a view state
    NBTCollectionWrapper *_wrapper;
}

- (void)setDocument:(id)document
{
    if ([document isKindOfClass:[NBTDocument class]])
    {
        self->_document = document;
        self->_wrapper = [NBTCollectionWrapper wrapperWithTag:[self->_document tag]
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

    [window setTitle:@"Document"];
    [window setReleasedWhenClosed:NO];

    [self setWindow:window];
    [window center];
    
    NSView *contentView = [[NSView alloc] initWithFrame:frame];
    [window setContentView:contentView];
    
    [self windowDidLoad];
}

- (void)initOutlineView
{
    NSWindow *window = [self window];
    NSOutlineView *mainView = [[NSOutlineView alloc] initWithFrame:[window frame]];
    [mainView setHeaderView:nil];
    
    NSTableColumn *mainCol = [[NSTableColumn alloc] initWithIdentifier:@"NBTKey"];
    [mainCol setTitle:@"Value"];
    [mainView addTableColumn:mainCol];
    [mainView setOutlineTableColumn:mainCol];
    
    if (self->_wrapper)
    {
        [mainView setDataSource:self->_wrapper];
        [mainView setDelegate:self->_wrapper];
        [mainView reloadData];
    }
    else
    {
        NSLog(@"may not set NSOutlineView dataSource, wrapper is (null)");
    }
    
    NSScrollView *scrollView = [[NSScrollView alloc] initWithFrame:[[window contentView] bounds]];
    [scrollView setTranslatesAutoresizingMaskIntoConstraints:NO];
    [scrollView setHasVerticalScroller:YES];
    [scrollView setDocumentView:mainView];
    [[window contentView] addSubview:scrollView];
    
    [mainView setColumnAutoresizingStyle:NSTableViewUniformColumnAutoresizingStyle];
    
    [mainCol setResizingMask:NSTableColumnAutoresizingMask];
    [mainCol setMinWidth:50.0];
    [mainCol setMaxWidth:CGFLOAT_MAX];
    
    NSView *contentView = [window contentView];
    [NSLayoutConstraint activateConstraints:@[
        [[scrollView topAnchor] constraintEqualToAnchor:[contentView topAnchor]],
        [[scrollView bottomAnchor] constraintEqualToAnchor:[contentView bottomAnchor]],
        [[scrollView leadingAnchor] constraintEqualToAnchor:[contentView leadingAnchor]],
        [[scrollView trailingAnchor] constraintEqualToAnchor:[contentView trailingAnchor]],
    ]];
    
    [contentView layoutSubtreeIfNeeded];
    [mainView sizeLastColumnToFit];
    [mainView expandItem:nil];
}

- (void)windowDidLoad
{
    NSURL *fileUrl = [self->_document fileURL];
    
    if (fileUrl)
    {
        [[self window] setTitleWithRepresentedFilename:[fileUrl path]];
        [self initOutlineView];
    }
    else
    {
        NewDocumentSheetController *controller = [NewDocumentSheetController new];
        
        [[self window] beginSheet:[controller window]
                completionHandler:^(NSModalResponse returnCode) {
            if (returnCode == NSModalResponseOK)
            {
                NBTBaseTag *tag = [[[controller type] new] mutableCopy];
                    
                [self setDocument:[NBTDocument fromTag:tag
                                            compressed:[controller compressed]]];
                [self initOutlineView];
            }
            else
            {
                [self close];
            }
        }];
    }
}

@end

@implementation NewDocumentSheetController {
    NSButton *_Nullable _confirmButton;
    Class _Nullable _type;
    BOOL _compressed;
}

- (instancetype)init
{
    self->_type = NULL;
    self->_compressed = NO;
    
    NSWindow *window = [[NSWindow alloc] initWithContentRect:NSMakeRect(0, 0, 300, 170)
                                                   styleMask:NSWindowStyleMaskTitled
                                                     backing:NSBackingStoreBuffered
                                                       defer:NO];
    
    NSStackView *mainView = [NSStackView new];
    [window setContentView:mainView];
    [mainView setOrientation:NSUserInterfaceLayoutOrientationVertical];
    [mainView setAlignment:NSLayoutAttributeLeft];
    [mainView setDistribution:NSStackViewDistributionEqualSpacing];
    [mainView setEdgeInsets:NSEdgeInsetsMake(16.0, 16.0, 16.0, 16.0)];
    
    NSStackView *optionsView = [NSStackView new];
    [mainView addArrangedSubview:optionsView];
    [optionsView setOrientation:NSUserInterfaceLayoutOrientationVertical];
    [optionsView setAlignment:NSLayoutAttributeLeft];
    [optionsView setSpacing:8.0];
    
    [self _makeOptionsViewIn:optionsView];
    
    NSButton *cancelButton = [NSButton buttonWithTitle:@"Cancel"
                                                target:self
                                                action:@selector(clickedCancel:)];
    [cancelButton setTintProminence:NSTintProminenceNone];
    self->_confirmButton = [NSButton buttonWithTitle:@"Confirm"
                                              target:self
                                              action:@selector(clickedConfirm:)];
    [self->_confirmButton setTintProminence:NSTintProminencePrimary];
    [self->_confirmButton setEnabled:NO];
    // by default the confirm button is disabled because the configuration is invalid by default
    NSStackView *buttonsView = [NSStackView stackViewWithViews:@[cancelButton, self->_confirmButton]];
    [mainView addArrangedSubview:buttonsView];
    
    return (self = [self initWithWindow:window]);
}

- (void)_makeOptionsViewIn:(NSStackView *)mainView;
{
    [mainView addArrangedSubview:[NSTextField labelWithString:@"Tag Type"]];
    
    NSView *box = [NSView new];
    [mainView addArrangedSubview:box];
    
    NSStackView *typeStackView = [NSStackView stackViewWithViews:@[
        [NSButton radioButtonWithTitle:@"Compound"
                                target:self
                                action:@selector(selectedRadio:)],
        [NSButton radioButtonWithTitle:@"List"
                                target:self
                                action:@selector(selectedRadio:)],
    ]];
    [typeStackView setOrientation:NSUserInterfaceLayoutOrientationVertical];
    [typeStackView setAlignment:NSLayoutAttributeLeft];
    [typeStackView setTranslatesAutoresizingMaskIntoConstraints:NO];
    
    [box addSubview:typeStackView];
    [NSLayoutConstraint activateConstraints:@[
        [[typeStackView leadingAnchor] constraintEqualToAnchor:[box leadingAnchor]],
        [[typeStackView trailingAnchor] constraintEqualToAnchor:[box trailingAnchor]],
        [[typeStackView topAnchor] constraintEqualToAnchor:[box topAnchor]],
        [[typeStackView bottomAnchor] constraintEqualToAnchor:[box bottomAnchor]],
        [[box leadingAnchor] constraintEqualToAnchor:[mainView leadingAnchor]
                                            constant:8.0],
    ]];
    
    NSButton *checkbox = [NSButton checkboxWithTitle:@"Compressed"
                                              target:self
                                              action:@selector(selectedCompressed:)];
    [mainView addArrangedSubview:checkbox];
    [checkbox setState:self->_compressed ?
        NSControlStateValueOn : NSControlStateValueOff];
}

- (void)selectedRadio:(NSButton *)sender
{
    self->_type = [[sender title] isEqual:@"Compound"] ?
        [NBTCompound class] : [NBTList class];
    [self->_confirmButton setEnabled:YES];
}

- (void)selectedCompressed:(NSButton *)sender
{
    self->_compressed = [sender state] == NSControlStateValueOn;
}

- (void)clickedCancel:(id)sender
{
    NSWindow *parent = [[self window] sheetParent];
    [parent endSheet:[self window]
          returnCode:NSModalResponseCancel];
}

- (void)clickedConfirm:(id)sender
{
    NSWindow *parent = [[self window] sheetParent];
    [parent endSheet:[self window]
          returnCode:NSModalResponseOK];
}

- (Class)type
{
    return self->_type;
}

- (BOOL)compressed
{
    return self->_compressed;
}

@end
