use win_msgbox::Okay;
use windows_sys::w;

fn main() {
    assert_eq!(win_msgbox::show::<Okay>(w!("Hello World")), Ok(Okay));
}
