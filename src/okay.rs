use super::Options;
use windows_sys::Win32::UI::WindowsAndMessaging::{MB_OK, MESSAGEBOX_RESULT, MESSAGEBOX_STYLE};

/// The message box contains one push button: `OK`.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Okay;

impl From<MESSAGEBOX_RESULT> for Okay {
    fn from(_: MESSAGEBOX_RESULT) -> Self {
        Self
    }
}

impl Options for Okay {
    fn flags() -> MESSAGEBOX_STYLE {
        MB_OK
    }
}
