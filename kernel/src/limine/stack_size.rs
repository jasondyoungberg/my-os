use super::{Response, MAGIC_1, MAGIC_2};

#[repr(C)]
pub struct StackSizeRequest {
    id: [u64; 4],
    revision: u64,
    pub response: Response<StackSizeResponse>,
    stack_size: u64,
}
impl StackSizeRequest {
    pub const fn new(stack_size: u64) -> Self {
        Self {
            id: [MAGIC_1, MAGIC_2, 0x224ef0460a8e8926, 0xe1cb0fc25f46ea3d],
            revision: 0,
            response: Response::none(),
            stack_size,
        }
    }
}
#[repr(C)]
pub struct StackSizeResponse {
    revision: u64,
}
