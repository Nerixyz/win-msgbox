use win_msgbox::{Okay, Result};

fn main() -> Result<()> {
    win_msgbox::information::<Okay>("Ferris landed on Mars.")
        .title("Landing Module")
        .show()?;
    Ok(())
}
