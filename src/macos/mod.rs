mod cursor;
mod keyboard;
mod view;
mod window;

pub use window::*;

#[allow(non_upper_case_globals)]
mod consts {
    use cocoa::foundation::NSUInteger;

    pub const NSDragOperationNone: NSUInteger = 0;
    pub const NSDragOperationCopy: NSUInteger = 1;
    pub const NSDragOperationLink: NSUInteger = 2;
    pub const NSDragOperationGeneric: NSUInteger = 4;
    pub const NSDragOperationMove: NSUInteger = 16;
}
use consts::*;

#[link(name = "CoreFoundation", kind = "framework")]
#[link(name = "CoreVideo", kind = "framework")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn CGWarpMouseCursorPosition(point: cocoa::foundation::NSPoint) -> i32;
}
