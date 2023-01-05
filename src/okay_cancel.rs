use super::Options;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    IDOK, MB_OKCANCEL, MESSAGEBOX_RESULT, MESSAGEBOX_STYLE,
};

/// The message box contains two push buttons: **OK** and **Cancel**.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum OkayCancel {
    /// The **OK** button was selected.
    Okay,
    /// The **Cancel** button was selected.
    Cancel,
}

impl From<MESSAGEBOX_RESULT> for OkayCancel {
    fn from(value: MESSAGEBOX_RESULT) -> Self {
        match value {
            IDOK => Self::Okay,
            _ => Self::Cancel,
        }
    }
}

impl Options for OkayCancel {
    fn flags() -> MESSAGEBOX_STYLE {
        MB_OKCANCEL
    }
}
