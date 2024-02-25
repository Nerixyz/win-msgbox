use win_msgbox::{w, Okay, Result};

fn main() -> Result<()> {
    win_msgbox::information::<Okay>(w!(
        "This is some longer paragraph to demonstrate how\nthe text is right justified."
    ))
    .right()
    .title(w!("Cool Demo App"))
    .show()?;
    Ok(())
}
