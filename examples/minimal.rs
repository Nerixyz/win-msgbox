use win_msgbox::Okay;

fn main() {
    assert_eq!(win_msgbox::show::<Okay>("Hello World"), Ok(Okay));
}
