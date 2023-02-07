#[cfg(not(target_arch = "wasm32"))]
mod inner {
    use clipboard::{ClipboardContext, ClipboardProvider};

    pub struct Clipboard {
        ctx: ClipboardContext,
    }

    impl Clipboard {
        pub fn new() -> Self {
            Self {
                ctx: ClipboardProvider::new().unwrap(),
            }
        }

        pub fn copy_to_clipboard(&mut self, s: String) {
            self.ctx.set_contents(s).unwrap();
        }

        pub fn copy_from_clipboard(&mut self) -> Option<String> {
            match self.ctx.get_contents() {
                Ok(s) => Some(s),
                Err(err) => {
                    log::warn!("Could not copy from clipboard: {}", err);
                    None
                },
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod inner {
    pub struct Clipboard {
        content: Option<String>,
    }

    impl Clipboard {
        pub fn new() -> Self {
            Self {
                content: None,
            }
        }

        pub fn copy_to_clipboard(&mut self, s: String) {
            self.content = Some(s);
        }

        pub fn copy_from_clipboard(&mut self) -> Option<String> {
            self.content.clone()
        }
    }
}

pub use inner::*;