use win_msgbox::{
    w,
    CancelTryAgainContinue::{self, *},
    Result,
};

fn main() -> Result<()> {
    let response = win_msgbox::error::<CancelTryAgainContinue>(w!("Couldn't download resource"))
        .title(w!("Download Error"))
        .show()?;

    match response {
        Cancel => println!("Cancelling downlaod..."),
        TryAgain => println!("Attempting redownload..."),
        Continue => println!("Skipping resource"),
    }

    Ok(())
}
