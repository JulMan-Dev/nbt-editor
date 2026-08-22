#import <Foundation/Foundation.h>
#import <AppKit/AppKit.h>

#import "delegate.h"

int main(int argc, const char **argv)
{
    NSApplication *app = [NSApplication sharedApplication];
    [app setDelegate:[NBTEditorDelegate new]];
    return NSApplicationMain(argc, argv);
}
