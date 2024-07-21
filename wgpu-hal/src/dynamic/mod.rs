mod command;
mod device;
mod queue;
mod surface;

use wgt::WasmNotSendSync;

use crate::BufferBinding;

pub use command::DynCommandEncoder;
pub use device::DynDevice;
pub use queue::DynQueue;
pub use surface::{DynAcquiredSurfaceTexture, DynSurface};

// TODO: docs
pub trait DynResource: WasmNotSendSync + 'static {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/// Utility macro for implementing `DynResource` for a list of types.
macro_rules! impl_dyn_resource {
    ($($type:ty),*) => {
        $(
            impl crate::DynResource for $type {
                fn as_any(&self) -> &dyn std::any::Any {
                    self
                }

                fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                    self
                }
            }
        )*
    };
}
pub(crate) use impl_dyn_resource;

trait DynResourceExt {
    fn expect_downcast_ref<T: DynResource>(&self) -> &T;
    fn expect_downcast_mut<T: DynResource>(&mut self) -> &mut T;
}

impl<R: DynResource + ?Sized> DynResourceExt for R {
    fn expect_downcast_ref<'a, T: DynResource>(&'a self) -> &'a T {
        self.as_any()
            .downcast_ref()
            .expect("Resource doesn't have the expected backend type.")
    }

    fn expect_downcast_mut<'a, T: DynResource>(&'a mut self) -> &'a mut T {
        self.as_any_mut()
            .downcast_mut()
            .expect("Resource doesn't have the expected backend type.")
    }
}

pub trait DynAccelerationStructure: DynResource + std::fmt::Debug {}
pub trait DynBindGroup: DynResource + std::fmt::Debug {}
pub trait DynBindGroupLayout: DynResource + std::fmt::Debug {}
pub trait DynBuffer: DynResource + std::fmt::Debug {}
pub trait DynCommandBuffer: DynResource + std::fmt::Debug {}
pub trait DynComputePipeline: DynResource + std::fmt::Debug {}
pub trait DynFence: DynResource + std::fmt::Debug {}
pub trait DynPipelineCache: DynResource + std::fmt::Debug {}
pub trait DynPipelineLayout: DynResource + std::fmt::Debug {}
pub trait DynQuerySet: DynResource + std::fmt::Debug {}
pub trait DynRenderPipeline: DynResource + std::fmt::Debug {}
pub trait DynSampler: DynResource + std::fmt::Debug {}
pub trait DynShaderModule: DynResource + std::fmt::Debug {}
pub trait DynSurfaceTexture: DynResource + std::fmt::Debug {}
pub trait DynTexture: DynResource + std::fmt::Debug {}
pub trait DynTextureView: DynResource + std::fmt::Debug {}

impl<'a> BufferBinding<'a, dyn DynBuffer> {
    pub fn expect_downcast<B: DynBuffer>(self) -> BufferBinding<'a, B> {
        BufferBinding {
            buffer: self.buffer.expect_downcast_ref(),
            offset: self.offset,
            size: self.size,
        }
    }
}
