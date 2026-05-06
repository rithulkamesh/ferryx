#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FerryxAbiVersion {
    pub major: u16,
    pub minor: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FerryxStatusCode {
    Ok = 0,
    NullPointer = 1,
    InvalidUtf8 = 2,
    InternalError = 255,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FerryxResult {
    pub status: FerryxStatusCode,
    pub payload_len: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FerryxBufferView {
    pub ptr: *const u8,
    pub len: usize,
    pub item_size: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FerryxArrowArrayStub {
    pub length: i64,
    pub null_count: i64,
    pub offset: i64,
}

pub const ABI_VERSION: FerryxAbiVersion = FerryxAbiVersion { major: 0, minor: 1 };

#[unsafe(no_mangle)]
pub extern "C" fn ferryx_abi_version() -> FerryxAbiVersion {
    ABI_VERSION
}

#[unsafe(no_mangle)]
pub extern "C" fn ferryx_validate_non_null(ptr: *const u8, len: usize) -> FerryxResult {
    if ptr.is_null() && len > 0 {
        return FerryxResult {
            status: FerryxStatusCode::NullPointer,
            payload_len: 0,
        };
    }
    FerryxResult {
        status: FerryxStatusCode::Ok,
        payload_len: len,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ferryx_validate_buffer_view(view: FerryxBufferView) -> FerryxResult {
    if view.ptr.is_null() && view.len > 0 {
        return FerryxResult {
            status: FerryxStatusCode::NullPointer,
            payload_len: 0,
        };
    }
    if view.item_size == 0 {
        return FerryxResult {
            status: FerryxStatusCode::InternalError,
            payload_len: 0,
        };
    }
    FerryxResult {
        status: FerryxStatusCode::Ok,
        payload_len: view.len,
    }
}

