use super::{Response, MAGIC_1, MAGIC_2};

#[repr(C)]
pub struct HhdmRequest {
    id: [u64; 4],
    revision: u64,
    pub response: Response<HhdmResponse>,
}
impl HhdmRequest {
    pub const fn new() -> Self {
        Self {
            id: [MAGIC_1, MAGIC_2, 0x48dcf1cb8ad2b852, 0x63984e959a98244b],
            revision: 0,
            response: Response::none(),
        }
    }
}
#[repr(C)]
pub struct HhdmResponse {
    revision: u64,
    pub offset: u64,
}
