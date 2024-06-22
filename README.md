# `win-msgbox`

This crate provides a fully featured, ergonomic interface to Windows' [`MessageBox`](https://learn.microsoft.com/ewindows/win32/api/winuser/nf-winuser-messagebox).

All possible options are usable and return values are Rust enums (or structs if only one option is available).

All configuration is done through `MessageBox` and available buttons are configured via `Options`.

`message` and `title` will be converted to UTF-16 when calling `show` on the fly,
if this isn't desired, use the structs and functions exported in the `raw` module. However, note that these are
`unsafe`, as they assume the passed pointers point to valid, null-terminated UTF-16 strings.

## Examples

Show a minimal message box with an **OK** button:

```rust
win_msgbox::show::<Okay>(w!("Hello World"));
```

![Image of the resulting message box](res/minimal.png)

Show a message box with an error icon, and match on the return value:

```rust
use win_msgbox::{w, CancelTryAgainContinue::{self, *}};

let response = win_msgbox::error::<CancelTryAgainContinue>(w!("Couldn't download resource"))
    .title(w!("Download Error"))
    .show()?;

match response {
    Cancel => println!("Cancelling downlaod..."),
    TryAgain => println!("Attempting redownload..."),
    Continue => println!("Skipping resource"),
}
```

![Image of the resulting message box](res/options.png)

For more examples, take a look at the [`examples`](examples) directory.
