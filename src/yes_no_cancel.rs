use super::Options;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    IDNO, IDYES, MB_YESNOCANCEL, MESSAGEBOX_RESULT, MESSAGEBOX_STYLE,
};

/// The message box contains three push buttons: **Yes**, **No**, and **Cancel**.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum YesNoCancel {
    /// The **Yes** button was selected.
    Yes,
    /// The **No** button was selected.
    No,
    /// The **Cancel** button was selected.
    Cancel,
}

impl From<MESSAGEBOX_RESULT> for YesNoCancel {
    fn from(value: MESSAGEBOX_RESULT) -> Self {
        match value {
            IDYES => Self::Yes,
            IDNO => Self::No,
            _ => Self::Cancel,
        }
    }
}

impl Options for YesNoCancel {
    fn flags() -> MESSAGEBOX_STYLE {
        MB_YESNOCANCEL
    }
}
