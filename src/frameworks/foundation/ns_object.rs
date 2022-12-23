//! `NSObject`, the root of most class hierarchies in Objective-C.

use crate::objc::{id, objc_classes, ClassExports, TrivialHostObject};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSObject

+ (id)alloc {
    env.objc.alloc_object(this, Box::new(TrivialHostObject), &mut env.mem)
}

- (id)init {
    this
}

@end

};
