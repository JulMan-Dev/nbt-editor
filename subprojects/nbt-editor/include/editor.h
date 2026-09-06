#import <Foundation/Foundation.h>
#import <AppKit/AppKit.h>

#import "NBTKit/NBTKit.h"

@interface NBTTagEditorController : NSWindowController

+ (nonnull instancetype)editorFrom:(nonnull NBTBaseTag *)baseTag
                    kindModifiable:(BOOL)kindModifiable
                         parentKey:(nullable NSString *)key;

@end
