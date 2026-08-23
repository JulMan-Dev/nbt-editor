#import <Foundation/Foundation.h>
#import <AppKit/AppKit.h>

#import "NBTKit/NBTKit.h"

@interface NBTDocument : NSDocument

@end

@interface NBTWindowContent : NSObject

- (nonnull instancetype)initWithTag:(nonnull NBTBaseTag *)tag;

@end
