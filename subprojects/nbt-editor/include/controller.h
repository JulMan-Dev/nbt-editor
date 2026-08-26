#import <Foundation/Foundation.h>
#import <AppKit/AppKit.h>

#import "NBTKit/NBTKit.h"
#import "content.h"

@interface NBTDocumentController : NSDocumentController

+ (nonnull NBTDocumentController *)sharedDocumentController;

@end

@interface NBTWindowController : NSWindowController

@end

@interface NewDocumentSheetController : NSWindowController

- (nonnull Class)type;
- (BOOL)compressed;

@end
