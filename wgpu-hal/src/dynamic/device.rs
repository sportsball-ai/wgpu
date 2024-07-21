use crate::{Device, DynBuffer, DynResource};

use super::DynResourceExt;

pub trait DynDevice: DynResource {
    unsafe fn destroy_buffer(&self, buffer: Box<dyn DynBuffer>);
}

// For all destroy methods we'd ideally cast the box and then unbox it with `Box::into_inner`.
// Unfortunately, the latter is only available on nightly Rust.
//
// Another alternative would be for `D::destroy_buffer` (and similar) to take a `Box<D::A::Buffer>`.
// However, that would require casting the box first to `Box<dyn Any>` for which we need
// super trait casting (https://rust-lang.github.io/rfcs/3324-dyn-upcasting.html)
// which as of writing is still being stabilized.
//
// So instead, we use the `expect_downcast_mut` and pass a mutable reference to the box.

impl<D: Device + DynResource> DynDevice for D {
    unsafe fn destroy_buffer(&self, mut buffer: Box<dyn DynBuffer>) {
        let buffer = buffer.expect_downcast_mut();
        unsafe { self.destroy_buffer(buffer) };
    }
}
