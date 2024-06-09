use limine::{request::FramebufferRequest, BaseRevision};

#[used]
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

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
