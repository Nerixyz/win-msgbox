use std::fmt::Write;

fn main() {
    std::panic::set_hook(Box::new(|info| {
        let mut buf = String::from("Panic occurred: ");

        if let Some(msg) = info.payload_as_str() {
            buf.push_str(msg);
        } else {
            buf.push_str("(no message)");
        }
        if let Some(loc) = info.location() {
            let _ = write!(&mut buf, " at {loc}");
        }
        let _ = win_msgbox::error::<win_msgbox::Okay>(&buf).show();
    }));

    panic!("Something went wrong");
}
