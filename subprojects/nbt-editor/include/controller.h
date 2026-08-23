#import <Foundation/Foundation.h>
#import <AppKit/AppKit.h>

#import "NBTKit/NBTKit.h"

@interface NBTDocumentController : NSDocumentController

+ (nonnull NBTDocumentController *)sharedDocumentController;

@end

@interface NBTWindowController : NSWindowController

@end
