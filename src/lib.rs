//! This crate provides a fully featured, ergonomic interface to Windows' [`MessageBox`](https://learn.microsoft.com/ewindows/win32/api/winuser/nf-winuser-messagebox).
//!
//! All possible options are usable and return values are Rust enums (or structs if only one option is available).
//!
//! All configuration is done through [MessageBox] and available buttons are configured via [Options].
//!
//! `message` and `title` will be converted to UTF-16 when calling [show][MessageBox::show] on the fly.
//! If this isn't desired, use the structs and functions exported in the [raw] module. However, note that these are
//! `unsafe`, as they assume the passed pointers point to valid, null-terminated UTF-16 strings.
//!
//! ## Examples
//!
//! Show a minimal message box with an **OK** button:
//!
//! ```no_run
//! use win_msgbox::Okay;
//! win_msgbox::show::<Okay>("Hello World");
//! ```
//!
//! Show a message box with an error icon, and match on the return value:
//!
//! ```no_run
//! use win_msgbox::CancelTryAgainContinue::{self, *};
//!
//! # fn main() -> win_msgbox::Result<()> {
//! let response = win_msgbox::error::<CancelTryAgainContinue>("Couldn't download resource")
//!     .title("Download Error")
//!     .show()?;
//!
//! match response {
//!     Cancel => println!("Cancelling downlaod..."),
//!     TryAgain => println!("Attempting redownload..."),
//!     Continue => println!("Skipping resource"),
//! }
//! #    Ok(())
//! # }
//! ```
#![deny(missing_docs)]
#![deny(clippy::cargo)]
use std::marker::PhantomData;
use windows_sys::Win32::{
    Foundation::{GetLastError, HWND},
    UI::WindowsAndMessaging::{
        MessageBoxW, MB_APPLMODAL, MB_DEFAULT_DESKTOP_ONLY, MB_DEFBUTTON1, MB_DEFBUTTON2,
        MB_DEFBUTTON3, MB_DEFBUTTON4, MB_HELP, MB_ICONASTERISK, MB_ICONERROR, MB_ICONEXCLAMATION,
        MB_ICONHAND, MB_ICONINFORMATION, MB_ICONQUESTION, MB_ICONSTOP, MB_ICONWARNING, MB_RIGHT,
        MB_RTLREADING, MB_SERVICE_NOTIFICATION, MB_SETFOREGROUND, MB_SYSTEMMODAL, MB_TASKMODAL,
        MB_TOPMOST, MESSAGEBOX_RESULT, MESSAGEBOX_STYLE,
    },
};

mod abort_retry_ignore;
mod cancel_try_again_continue;
mod okay;
mod okay_cancel;
pub mod raw;
mod retry_cancel;
mod yes_no;
mod yes_no_cancel;

pub use abort_retry_ignore::*;
pub use cancel_try_again_continue::*;
pub use okay::*;
pub use okay_cancel::*;
pub use retry_cancel::*;
pub use yes_no::*;
pub use yes_no_cancel::*;

/// Raw error returned by [GetLastError](https://learn.microsoft.com/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror).
pub type Error = windows_sys::Win32::Foundation::WIN32_ERROR;

/// Convenience wrapper type for a `Result<T, win_msgbox::Error>`.
pub type Result<T> = core::result::Result<T, Error>;

/// This trait is implemented for all possible options.
///
/// Available are:
///
/// - [**Abort**, **Retry**, and **Ignore**](AbortRetryIgnore)
/// - [**Cancel**, **Try Again**, and **Continue**](CancelTryAgainContinue)
/// - [**OK**](Okay)
/// - [**OK**, and **Cancel**](OkayCancel)
/// - [**Retry**, and **Cancel**](RetryCancel)
/// - [**Yes**, and **No**](YesNo)
/// - [**Yes**, **No**, and **Cancel**](YesNoCancel)
pub trait Options: From<MESSAGEBOX_RESULT> {
    /// The flags this option requires to be shown.
    fn flags() -> MESSAGEBOX_STYLE;
}

/// The icon to be displayed in a message box.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
#[repr(u32)] // = MESSAGEBOX_STYLE
pub enum Icon {
    /// An exclamation-point icon appears in the message box.
    Exclamation,
    /// An exclamation-point icon appears in the message box.
    Warning,
    /// An icon consisting of a lowercase letter i in a circle appears in the message box.
    Information,
    /// An icon consisting of a lowercase letter i in a circle appears in the message box.
    Asterisk,
    /// A question-mark icon appears in the message box.
    /// The question-mark message icon is no longer recommended
    /// because it does not clearly represent a specific type of message
    /// and because the phrasing of a message as a question could apply to any message type.
    /// In addition, users can confuse the message symbol question mark with Help information.
    /// Therefore, do not use this question mark message symbol in your message boxes.
    /// The system continues to support its inclusion only for backward compatibility.
    Question,
    /// A stop-sign icon appears in the message box.
    Stop,
    /// A stop-sign icon appears in the message box.
    Error,
    /// A stop-sign icon appears in the message box.
    Hand,
}

impl Icon {
    fn style(self) -> MESSAGEBOX_STYLE {
        match self {
            Icon::Exclamation => MB_ICONEXCLAMATION,
            Icon::Warning => MB_ICONWARNING,
            Icon::Information => MB_ICONINFORMATION,
            Icon::Asterisk => MB_ICONASTERISK,
            Icon::Question => MB_ICONQUESTION,
            Icon::Stop => MB_ICONSTOP,
            Icon::Error => MB_ICONERROR,
            Icon::Hand => MB_ICONHAND,
        }
    }
}

/// Specifies the modality of the dialog box.
#[derive(Debug, Default, Eq, PartialEq, Clone, Copy, Hash)]
#[repr(u32)] // = MESSAGEBOX_STYLE
pub enum Modal {
    /// The user must respond to the message box before continuing work in the window identified by the [`hwnd`](MessageBox::hwnd).
    /// However, the user can move to the windows of other threads and work in those windows.
    /// Depending on the hierarchy of windows in the application,
    /// the user may be able to move to other windows within the thread.
    /// All child windows of the parent of the message box are automatically disabled,
    /// but pop-up windows are not.
    #[default]
    Application = MB_APPLMODAL,
    /// Same as [`Application`](Self::Application) except that the message box has the `WS_EX_TOPMOST` style.
    /// Use system-modal message boxes to notify the user of serious,
    /// potentially damaging errors that require immediate attention (for example, running out of memory).
    /// This flag has no effect on the user's ability to interact with windows other than those associated with [`hwnd`](MessageBox::hwnd).
    System = MB_SYSTEMMODAL,
    /// Same as [`Application`](Self::Application) except that all the top-level windows belonging to the current thread are disabled
    /// if the [`hwnd`](MessageBox::hwnd) parameter is `0`. Use this flag when the calling application
    /// or library does not have a window handle available but still needs to prevent input to other windows in the calling thread
    /// without suspending other threads.
    Task = MB_TASKMODAL,
}

/// Specifies the default button of the dialog box.
///
/// The meaning of the nth button is determined by the type ([Options]).
#[derive(Debug, Default, Eq, PartialEq, Clone, Copy, Hash)]
#[repr(u32)] // = MESSAGEBOX_STYLE
pub enum DefaultButton {
    /// The first button is the default button.
    #[default]
    DefaultButton1 = MB_DEFBUTTON1,
    /// The second button is the default button.
    DefaultButton2 = MB_DEFBUTTON2,
    /// The third button is the default button.
    DefaultButton3 = MB_DEFBUTTON3,
    /// The fourth button is the default button.
    DefaultButton4 = MB_DEFBUTTON4,
}

/// A builder for a modal dialog box that contains a system icon,
/// a set of buttons, and a brief application-specific message, such as status or error information.
///
/// The type of the message box is specified by `T` (See [Options] for available options).
pub struct MessageBox<'a, T> {
    /// The icon of this message box.
    icon: Icon,
    /// The text inside the message box.
    text: &'a str,
    /// The title of the message box (default is None).
    title: Option<&'a str>,
    /// The owner window of the message box (default is `0` - no owner)
    hwnd: HWND,
    /// Flags for the creation of this message box.
    flags: MESSAGEBOX_STYLE,
    /// The response options of message box.
    _response: PhantomData<T>,
}

impl<T> std::fmt::Debug for MessageBox<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MessageBox")
            .field("title", &self.title)
            .field("text", &self.text)
            .field("icon", &self.icon)
            .field("hwnd", &self.hwnd)
            .finish()
    }
}

macro_rules! ctors {
    ($($name:ident => $icon:ident),*) => {
        impl <'a, T> MessageBox<'a, T> {
            $(
            #[doc = concat!("Creates a new message box where its icon is set to [", stringify!($icon), "](Icon::", stringify!($icon),").")]
            pub fn $name(text: &'a str) -> Self {
                Self::new(text).icon(Icon::$icon)
            }
            )*
        }
        $(
        #[doc = concat!("Creates a new message box where its icon is set to [", stringify!($icon), "](Icon::", stringify!($icon),").")]
        pub fn $name<T>(text: &str) -> MessageBox<'_, T> {
            MessageBox::<T>::$name(text)
        })*
    };
}

impl<'a, T> MessageBox<'a, T> {
    /// Creates a new message box with a specified `text` to be displayed.
    /// If the string consists of more than one line,
    /// you can separate the lines using a carriage return and/or linefeed character between each line.
    pub fn new(text: &'a str) -> Self {
        Self {
            icon: Icon::Information,
            text,
            title: None,
            hwnd: std::ptr::null_mut(),
            flags: 0,
            _response: PhantomData,
        }
    }

    /// The [Icon] to be displayed in this message box.
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = icon;
        self
    }

    /// The dialog box title. If this parameter is **null**, the default title is **Error**.
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title.into();
        self
    }

    /// A handle to the owner window of the message box to be created.
    /// If this parameter is `0`, the message box has no owner window (default).
    pub fn hwnd(mut self, hwnd: HWND) -> Self {
        self.hwnd = hwnd;
        self
    }

    /// Set the modality of the dialog box. See [Modal] for options.
    pub fn modal(mut self, modal: Modal) -> Self {
        self.flags |= modal as u32;
        self
    }

    /// Set the default button of the dialog box. See [DefaultButton] for options.
    pub fn default_button(mut self, btn: DefaultButton) -> Self {
        self.flags |= btn as u32;
        self
    }

    /// Same as desktop of the interactive window station. For more information, see [Window Stations](https://learn.microsoft.com/windows/desktop/winstation/window-stations).
    /// If the current input desktop is not the default desktop,
    /// [show](Self::show) does not return until the user switches to the default desktop.
    pub fn default_desktop_only(mut self) -> Self {
        self.flags |= MB_DEFAULT_DESKTOP_ONLY;
        self
    }

    /// The text is right-justified.
    pub fn right(mut self) -> Self {
        self.flags |= MB_RIGHT;
        self
    }

    /// Displays message and caption text using right-to-left reading order on Hebrew and Arabic systems.
    pub fn rtl_reading(mut self) -> Self {
        self.flags |= MB_RTLREADING;
        self
    }

    /// The message box becomes the foreground window.
    /// Internally, the system calls the [SetForegroundWindow](https://learn.microsoft.com/windows/desktop/api/winuser/nf-winuser-setforegroundwindow) function for the message box.
    pub fn set_foreground(mut self) -> Self {
        self.flags |= MB_SETFOREGROUND;
        self
    }

    /// The message box is created with the `WS_EX_TOPMOST` window style.
    pub fn topmost(mut self) -> Self {
        self.flags |= MB_TOPMOST;
        self
    }

    /// The caller is a service notifying the user of an event.
    /// The function displays a message box on the current active desktop,
    /// even if there is no user logged on to the computer.
    ///
    /// Terminal Services: If the calling thread has an impersonation token,
    /// the function directs the message box to the session specified in the impersonation token.
    ///
    /// If this is called, [`hwnd`](Self::hwnd) must not be called - it must remain `0`.
    /// his is so that the message box can appear on a desktop other than the desktop corresponding to the `hwnd`.
    ///
    /// For information on security considerations in regard to using this flag, see [Interactive Services](https://learn.microsoft.com/windows/desktop/Services/interactive-services).
    /// In particular, be aware that this flag can produce interactive content on a locked desktop
    /// and should therefore be used for only a very limited set of scenarios, such as resource exhaustion.
    pub fn service_notification(mut self) -> Self {
        self.flags |= MB_SERVICE_NOTIFICATION;
        self
    }

    /// Adds a Help button to the message box.
    /// When the user clicks the Help button or presses F1,
    /// the system sends a [WM_HELP](https://learn.microsoft.com/windows/desktop/shell/wm-help) message to the owner.
    pub fn with_help(mut self) -> Self {
        self.flags |= MB_HELP;
        self
    }
}

impl<T: Options> MessageBox<'_, T> {
    /// Shows the message box, returning the option the user clicked on.
    ///
    /// If a message box has a **Cancel** button, the function returns the `Cancel` value
    /// if either the ESC key is pressed or the **Cancel** button is selected.
    ///
    /// If the message box has no **Cancel** button, pressing ESC will no effect -
    /// unless an **Ok** button is present.
    ///
    /// If an **Ok** button is displayed and the user presses ESC, the return value will be `Ok`.
    pub fn show(self) -> Result<T> {
        let text: Vec<_> = self.text.encode_utf16().chain(std::iter::once(0)).collect();
        let title = match self.title {
            Some(t) => t.encode_utf16().chain(std::iter::once(0)).collect(),
            None => Vec::new(),
        };

        let return_code = unsafe {
            MessageBoxW(
                self.hwnd,
                text.as_ptr(),
                if title.is_empty() {
                    std::ptr::null()
                } else {
                    title.as_ptr()
                },
                T::flags() | self.icon.style() | self.flags,
            )
        };
        match return_code {
            0 => Err(unsafe { GetLastError() }),
            x => Ok(T::from(x)),
        }
    }
}

ctors! {
    exclamation => Exclamation,
    warning => Warning,
    information => Information,
    asterisk => Asterisk,
    question => Question,
    stop => Stop,
    error => Error,
    hand => Hand
}

/// Shows a message box with a specified `text` to be displayed.
///
/// For more options see [MessageBox].
pub fn show<T: Options>(text: &str) -> Result<T> {
    MessageBox::new(text).show()
}
