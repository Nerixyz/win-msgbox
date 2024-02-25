use win_msgbox::{w, Okay};

fn main() {
    assert_eq!(win_msgbox::show::<Okay>(w!("Hello World")), Ok(Okay));
}
