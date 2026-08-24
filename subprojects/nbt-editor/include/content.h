#import <Foundation/Foundation.h>
#import <AppKit/AppKit.h>

#import "NBTKit/NBTKit.h"

@interface NBTDocument : NSDocument

- (nullable NBTBaseTag *)tag;
- (BOOL)compressed;

@end

@interface NBTCollectionWrapper : NSObject <NSOutlineViewDataSource, NSOutlineViewDelegate, NSCopying, NSMutableCopying>

+ (nonnull instancetype)wrapperWithTag:(nullable NBTBaseTag *)tag
                                parent:(nullable NBTCollectionWrapper *)parent;

- (nonnull instancetype)initWithTag:(nullable NBTBaseTag *)tag
                             parent:(nullable NBTCollectionWrapper *)parent;

@end
