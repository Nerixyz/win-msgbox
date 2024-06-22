//! This module provides the raw functionality to avoid having to convert UTF-8 to UTF-16 when showing a message box.
//! Otherwise, it's identical to the main exports.
//!
//! ## Safety
//!
//! Note that these functions are `unsafe`, as they assume the strings passed as `message` and `title`
//! are and will remain valid UTF-16 and null terminated until [show][MessageBox::show] is called.
//! This is especially important when passing strings created at runtime.
//!
//! To create a wide string statically, use the `w!` macro re-exported by this module from
//! [`windows_sys`](https://docs.rs/windows-sys/latest/windows_sys/macro.w.html).
//!
//! ## Examples
//!
//! Show a minimal message box with an **OK** button:
//!
//! ```no_run
//! use win_msgbox::{raw::{show, w}, Okay};
//! unsafe { show::<Okay>(w!("Hello World")); }
//! ```
//!
//! Show a message box with an error icon, and match on the return value:
//!
//! ```no_run
//! use win_msgbox::{raw::{error, w}, CancelTryAgainContinue::{self, *}};
//!
//! # fn main() -> win_msgbox::Result<()> {
//! // Safety: `w!` creates null-terminated UTF-16 strings at compile time,
//! //         thus the pointed-to value never changes.
//! let response = unsafe {
//!     error::<CancelTryAgainContinue>(w!("Couldn't download resource"))
//!         .title(w!("Download Error"))
//!         .show()?
//! };
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
use windows_sys::{
    core::PCWSTR,
    Win32::{
        Foundation::{GetLastError, HWND},
        UI::WindowsAndMessaging::{
            MessageBoxW, MB_DEFAULT_DESKTOP_ONLY, MB_HELP, MB_RIGHT, MB_RTLREADING,
            MB_SERVICE_NOTIFICATION, MB_SETFOREGROUND, MB_TOPMOST, MESSAGEBOX_STYLE,
        },
    },
};

use crate::{DefaultButton, Icon, Modal, Options, Result};

pub use windows_sys::w;

/// A builder for a modal dialog box that contains a system icon,
/// a set of buttons, and a brief application-specific message, such as status or error information.
///
/// The type of the message box is specified by `T` (See [Options] for available options).
pub struct MessageBox<T> {
    /// The icon of this message box.
    icon: Icon,
    /// The text inside the message box.
    text: PCWSTR,
    /// The title of the message box (default is null).
    title: PCWSTR,
    /// The owner window of the message box (default is `0` - no owner)
    hwnd: HWND,
    /// Flags for the creation of this message box.
    flags: MESSAGEBOX_STYLE,
    /// The response options of message box.
    _response: PhantomData<T>,
}

impl<T> std::fmt::Debug for MessageBox<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MessageBox")
            .field("icon", &self.icon)
            .field("hwnd", &self.hwnd)
            .finish()
    }
}

macro_rules! ctors {
    ($($name:ident => $icon:ident),*) => {
        impl <T> MessageBox<T> {
            $(
            #[doc = concat!("Creates a new message box where its icon is set to [", stringify!($icon), "](Icon::", stringify!($icon),").")]
            pub fn $name(text: impl Into<PCWSTR>) -> Self {
                Self::new(text).icon(Icon::$icon)
            }
            )*
        }
        $(
        #[doc = concat!("Creates a new message box where its icon is set to [", stringify!($icon), "](Icon::", stringify!($icon),").")]
        pub fn $name<T>(text: impl Into<PCWSTR>) -> MessageBox<T> {
            MessageBox::<T>::$name(text)
        })*
    };
}

impl<T> MessageBox<T> {
    /// Creates a new message box with a specified `text` to be displayed.
    /// If the string consists of more than one line,
    /// you can separate the lines using a carriage return and/or linefeed character between each line.
    pub fn new(text: impl Into<PCWSTR>) -> Self {
        Self {
            icon: Icon::Information,
            text: text.into(),
            title: std::ptr::null(),
            hwnd: 0,
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
    pub fn title(mut self, title: impl Into<PCWSTR>) -> Self {
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

impl<T: Options> MessageBox<T> {
    /// Shows the message box, returning the option the user clicked on.
    ///
    /// If a message box has a **Cancel** button, the function returns the `Cancel` value
    /// if either the ESC key is pressed or the **Cancel** button is selected.
    ///
    /// If the message box has no **Cancel** button, pressing ESC will no effect -
    /// unless an **Ok** button is present.
    ///
    /// If an **Ok** button is displayed and the user presses ESC, the return value will be `Ok`.
    ///
    /// ### Safety
    ///
    /// [`text`][Self::new] and [`title`][Self::title] (if set) must point to a valid 16 bit, null terminated string.
    pub unsafe fn show(self) -> Result<T> {
        let return_code = MessageBoxW(
            self.hwnd,
            self.text,
            self.title,
            T::flags() | self.icon.style() | self.flags,
        );
        match return_code {
            0 => Err(GetLastError()),
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
///
/// ### Safety
///
/// `text` must point to a valid 16 bit, null terminated string.
pub unsafe fn show<T: Options>(text: impl Into<PCWSTR>) -> Result<T> {
    MessageBox::new(text).show()
}
