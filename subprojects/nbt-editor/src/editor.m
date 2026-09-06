#import "editor.h"

@implementation NBTTagEditorController {
    NBTBaseTag *_Nonnull _baseTag;
    NBTBaseTag *_Nonnull _currentTag;
    BOOL _kindModifiable;
    NSString *_Nullable _key;
}

+ (nonnull instancetype)editorFrom:(nonnull NBTBaseTag *)baseTag
                    kindModifiable:(BOOL)kindModifiable
                         parentKey:(nullable NSString *)key
{
    NBTTagEditorController *controller = [self new];
    controller->_baseTag = baseTag;
    controller->_currentTag = [baseTag mutableCopy];
    controller->_kindModifiable = kindModifiable;
    controller->_key = key;
    return controller;
}

- (void)loadWindow
{
    NSWindow *window = [[NSWindow alloc] initWithContentRect:NSMakeRect(0, 0, 300, 300)
                                                   styleMask:NSWindowStyleMaskTitled | NSWindowStyleMaskResizable
                                                     backing:NSBackingStoreBuffered
                                                       defer:NO];
    [self setWindow:window];
}

@end
