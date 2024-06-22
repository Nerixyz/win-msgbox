use win_msgbox::{
    CancelTryAgainContinue::{self, *},
    Result,
};

fn main() -> Result<()> {
    let response = win_msgbox::error::<CancelTryAgainContinue>("Couldn't download resource")
        .title("Download Error")
        .show()?;

    match response {
        Cancel => println!("Cancelling downlaod..."),
        TryAgain => println!("Attempting redownload..."),
        Continue => println!("Skipping resource"),
    }

    Ok(())
}
