use super::Options;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    IDCONTINUE, IDTRYAGAIN, MB_CANCELTRYCONTINUE, MESSAGEBOX_RESULT, MESSAGEBOX_STYLE,
};

/// The message box contains three push buttons: **Cancel**, **Try Again**, **Continue**.
/// Use this message box type instead of [AbortRetryIgnore](crate::AbortRetryIgnore).
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum CancelTryAgainContinue {
    /// The **Cancel** button was selected.
    Cancel,
    /// The **Try Again** button was selected.
    TryAgain,
    /// The **Continue** button was selected.
    Continue,
}

impl From<MESSAGEBOX_RESULT> for CancelTryAgainContinue {
    fn from(value: MESSAGEBOX_RESULT) -> Self {
        match value {
            IDTRYAGAIN => Self::TryAgain,
            IDCONTINUE => Self::Continue,
            _ => Self::Cancel,
        }
    }
}

impl Options for CancelTryAgainContinue {
    fn flags() -> MESSAGEBOX_STYLE {
        MB_CANCELTRYCONTINUE
    }
}
