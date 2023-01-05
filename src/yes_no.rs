use super::Options;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    IDYES, MB_YESNO, MESSAGEBOX_RESULT, MESSAGEBOX_STYLE,
};

/// The message box contains two push buttons: **Yes** and **No**.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum YesNo {
    /// The **Yes** button was selected.
    Yes,
    /// The **No** button was selected.
    No,
}

impl From<MESSAGEBOX_RESULT> for YesNo {
    fn from(value: MESSAGEBOX_RESULT) -> Self {
        match value {
            IDYES => Self::Yes,
            _ => Self::No,
        }
    }
}

impl Options for YesNo {
    fn flags() -> MESSAGEBOX_STYLE {
        MB_YESNO
    }
}
