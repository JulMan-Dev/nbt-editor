#import "delegate.h"
#import "content.h"
#import "controller.h"

@implementation NBTEditorDelegate

- (void)applicationWillFinishLaunching:(NSNotification *)notification
{
    // this ensures the NBTDocumentController is made at least once
    (void)[NBTDocumentController sharedDocumentController];
}

- (void)applicationDidFinishLaunching:(NSNotification *)notification
{
    // making the menu
    NSMenu *mainMenu = [NSMenu new];
    
    // Application menu
    NSString *appName = [[NSBundle mainBundle] objectForInfoDictionaryKey:@"CFBundleExecutable"];
    
    NSMenu *appMenu = [[NSMenu alloc] initWithTitle:[appName copy]];

    [appMenu addItemWithTitle:[NSString stringWithFormat:@"About %@", appName]
                       action:@selector(orderFrontStandardAboutPanel:)
                keyEquivalent:@""];

    [appMenu addItem:[NSMenuItem separatorItem]];

    [appMenu addItemWithTitle:[NSString stringWithFormat:@"Quit %@", appName]
                       action:@selector(terminate:)
                keyEquivalent:@"q"];

    NSMenuItem *appMenuItem = [[NSMenuItem alloc] initWithTitle:appName
                                                         action:nil
                                                  keyEquivalent:@""];

    [appMenuItem setSubmenu:appMenu];
    [mainMenu addItem:appMenuItem];
    
    // File submenu
    NSMenu *fileMenu = [[NSMenu alloc] initWithTitle:@"File"];
    
    NSMenuItem *newItem = [[NSMenuItem alloc] initWithTitle:@"New"
                                                      action:@selector(newDocument:)
                                               keyEquivalent:@"n"];
    newItem.target = nil;
    [fileMenu addItem:newItem];
    
    NSMenuItem *openItem = [[NSMenuItem alloc] initWithTitle:@"Open…"
                                                       action:@selector(openDocument:)
                                                keyEquivalent:@"o"];
    openItem.target = nil;
    [fileMenu addItem:openItem];

    NSMenuItem *saveItem = [[NSMenuItem alloc] initWithTitle:@"Save"
                                                       action:@selector(saveDocument:)
                                                keyEquivalent:@"s"];
    saveItem.target = nil;
    [fileMenu addItem:saveItem];

    NSMenuItem *saveAsItem = [[NSMenuItem alloc] initWithTitle:@"Save As…"
                                                         action:@selector(saveDocumentAs:)
                                                  keyEquivalent:@"S"]; // Shift+Cmd+S
    saveAsItem.keyEquivalentModifierMask = NSEventModifierFlagCommand | NSEventModifierFlagShift;
    saveAsItem.target = nil;
    [fileMenu addItem:saveAsItem];
    
    NSMenuItem *fileItemMenu = [[NSMenuItem alloc] initWithTitle:@"File"
                                                          action:nil
                                                   keyEquivalent:@""];
    [fileItemMenu setSubmenu:fileMenu];
    [mainMenu addItem:fileItemMenu];
    
    [[NSApplication sharedApplication] setMainMenu:mainMenu];
    [[NSApplication sharedApplication] activateIgnoringOtherApps:YES];
    
    // checking arguments
    NSArray *args = [[NSProcessInfo processInfo] arguments];
    NBTDocumentController *controller = [NBTDocumentController sharedDocumentController];
    
    if ([args count] > 1)
    {
        for (uintptr_t i = 1; i < [args count]; i++)
        {
            NSURL *url = [[NSURL alloc] initFileURLWithPath:args[i]];
            
            NSError *error = nil;
            NBTDocument *document = [[NBTDocument alloc] initWithContentsOfURL:url
                                                                        ofType:@"net.minecraft.NBT"
                                                                         error:&error];
            
            if (document) {
                [[NSDocumentController sharedDocumentController] addDocument:document];
                [document makeWindowControllers];
                for (NSWindowController *controller in [document windowControllers])
                {
                    [controller showWindow:nil];
                }
            } else {
                NSLog(@"Failed to open %@, error: %@", url, error);
            }
        }
    }
    else
    {
        [controller openUntitledDocumentAndDisplay:YES
                                             error:nil];
    }
}

@end
