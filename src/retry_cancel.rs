use super::Options;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    IDRETRY, MB_RETRYCANCEL, MESSAGEBOX_RESULT, MESSAGEBOX_STYLE,
};

/// The message box contains two push buttons: **Retry** and **Cancel**.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum RetryCancel {
    /// The **Retry** button was selected.
    Retry,
    /// The **Cancel** button was selected.
    Cancel,
}

impl From<MESSAGEBOX_RESULT> for RetryCancel {
    fn from(value: MESSAGEBOX_RESULT) -> Self {
        match value {
            IDRETRY => Self::Retry,
            _ => Self::Cancel,
        }
    }
}

impl Options for RetryCancel {
    fn flags() -> MESSAGEBOX_STYLE {
        MB_RETRYCANCEL
    }
}
