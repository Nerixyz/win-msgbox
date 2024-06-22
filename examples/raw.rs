use win_msgbox::{
    raw::{self, w},
    CancelTryAgainContinue::{self, *},
    Result,
};

fn main() -> Result<()> {
    // Safety: `w!` creates null-terminated UTF-16 strings at compile time, thus the pointed-to value never changes.
    let response = unsafe {
        raw::error::<CancelTryAgainContinue>(w!("Couldn't download resource"))
            .title(w!("Download Error"))
            .show()?
    };

    match response {
        Cancel => println!("Cancelling downlaod..."),
        TryAgain => println!("Attempting redownload..."),
        Continue => println!("Skipping resource"),
    }

    Ok(())
}
