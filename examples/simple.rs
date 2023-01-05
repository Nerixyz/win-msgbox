use win_msgbox::Okay;
use windows_sys::{w, Win32::Foundation::WIN32_ERROR};

fn main() -> Result<(), WIN32_ERROR> {
    win_msgbox::information::<Okay>(w!("Ferris landed on Mars."))
        .title(w!("Landing Module"))
        .show()?;
    Ok(())
}
