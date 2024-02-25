use win_msgbox::{w, Okay, Result};

fn main() -> Result<()> {
    win_msgbox::information::<Okay>(w!("Ferris landed on Mars."))
        .title(w!("Landing Module"))
        .show()?;
    Ok(())
}
