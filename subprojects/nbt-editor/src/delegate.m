#import "delegate.h"

@implementation NBTEditorDelegate {
    NSApplication *application;
    NSDocumentController *controller;
}

- (void)applicationDidFinishLaunching:(NSNotification *)notification
{
    self->application = [NSApplication sharedApplication];
    self->controller = [NSDocumentController sharedDocumentController];
    
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
    [fileMenu addItemWithTitle:@"New"
                        action:@selector(newDocument:)
                 keyEquivalent:@"n"];
    NSMenuItem *fileItemMenu = [[NSMenuItem alloc] initWithTitle:@"File"
                                                          action:nil
                                                   keyEquivalent:@""];
    [fileItemMenu setSubmenu:fileMenu];
    [mainMenu addItem:fileItemMenu];
    
    [self->application setMainMenu:mainMenu];
    [self->application activateIgnoringOtherApps:YES];
}

@end
