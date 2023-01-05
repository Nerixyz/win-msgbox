use super::Options;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    IDABORT, IDRETRY, MB_ABORTRETRYIGNORE, MESSAGEBOX_RESULT, MESSAGEBOX_STYLE,
};

/// The message box contains three push buttons: **Abort**, **Retry**, and **Ignore**.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum AbortRetryIgnore {
    /// The **Abort** button was selected.
    Abort,
    /// The **Retry** button was selected.
    Retry,
    /// The **Ignore** button was selected.
    Ignore,
}

impl From<MESSAGEBOX_RESULT> for AbortRetryIgnore {
    fn from(value: MESSAGEBOX_RESULT) -> Self {
        match value {
            IDABORT => Self::Abort,
            IDRETRY => Self::Retry,
            _ => Self::Ignore,
        }
    }
}

impl Options for AbortRetryIgnore {
    fn flags() -> MESSAGEBOX_STYLE {
        MB_ABORTRETRYIGNORE
    }
}
