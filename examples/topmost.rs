use win_msgbox::{Okay, Result};

fn main() -> Result<()> {
    win_msgbox::information::<Okay>(
        "This is some longer paragraph to demonstrate how\nthe text is right justified.",
    )
    .right()
    .title("Cool Demo App")
    .show()?;
    Ok(())
}
