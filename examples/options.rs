use win_msgbox::CancelTryAgainContinue;
use windows_sys::{w, Win32::Foundation::WIN32_ERROR};

fn main() -> Result<(), WIN32_ERROR> {
    let response = win_msgbox::error::<CancelTryAgainContinue>(w!("Couldn't download resource"))
        .title(w!("Download Error"))
        .show()?;

    use CancelTryAgainContinue::*;
    match response {
        Cancel => println!("Cancelling downlaod..."),
        TryAgain => println!("Attempting redownload..."),
        Continue => println!("Skipping resource"),
    }

    Ok(())
}
