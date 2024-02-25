use win_msgbox::Okay;
use windows_sys::{w, Win32::Foundation::WIN32_ERROR};

fn main() -> Result<(), WIN32_ERROR> {
    win_msgbox::information::<Okay>(w!(
        "This is some longer paragraph to demonstrate how\nthe text is right justified."
    ))
    .right()
    .title(w!("Cool Demo App"))
    .show()?;
    Ok(())
}
