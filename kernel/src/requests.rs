use limine::{
    request::{FramebufferRequest, MemoryMapRequest},
    response::{FramebufferResponse, MemoryMapResponse},
    BaseRevision,
};
use spin::Lazy;

#[used]
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[link_section = ".requests"]
static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

pub static FRAMEBUFFER_RESPONSE: Lazy<&FramebufferResponse> = Lazy::new(|| {
    FRAMEBUFFER_REQUEST
        .get_response()
        .expect("verify() was not called before accessing the framebuffer response")
});
pub static MEMORY_MAP_RESPONSE: Lazy<&MemoryMapResponse> = Lazy::new(|| {
    MEMORY_MAP_REQUEST
        .get_response()
        .expect("verify() was not called before accessing the memory map response")
});

/// Verifies that the requests were completed
pub fn verify() {
    assert!(
        BASE_REVISION.is_supported(),
        "Base revision is not supported"
    );
    assert!(
        FRAMEBUFFER_REQUEST.get_response().is_some(),
        "Framebuffer request was not completed"
    );
}
