use std::sync::{Arc, Mutex};
use crate::window::WindowHandler;

#[cfg(target_arch = "wasm32")]
pub fn subscribe_clipboard_events<H: WindowHandler + 'static>(core: &Arc<Mutex<H>>) {
    //use winit::platform::web::WindowExtWebSys;
    use wasm_bindgen::JsCast;
    let win = web_sys::window().expect("Could not get window");

    // https://w3c.github.io/clipboard-apis/#override-copy
    {
        let core = core.clone();
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |e: web_sys::ClipboardEvent| {
            if let Some(s) = core.lock().unwrap().copy() {
                log::info!("Copying:\n{}", s);
                e.clipboard_data().unwrap().set_data("text/plain", &s).unwrap();
            }
            e.prevent_default();
        }) as Box<dyn FnMut(_)>);
        win
            .add_event_listener_with_callback("copy", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    // https://w3c.github.io/clipboard-apis/#override-paste
    {
        let core = core.clone();
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |e: web_sys::ClipboardEvent| {
            log::info!("Pasting: {:?}", e);
            let s = e.clipboard_data().unwrap().get_data("text/plain").unwrap();
            core.lock().unwrap().paste(s);
            e.prevent_default();
        }) as Box<dyn FnMut(_)>);
        win
            .add_event_listener_with_callback("paste", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    // https://w3c.github.io/clipboard-apis/#override-cut
    {
        let core = core.clone();
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |e: web_sys::ClipboardEvent| {
            if let Some(s) = core.lock().unwrap().cut() {
                log::info!("Cutting:\n{}", s);
                e.clipboard_data().unwrap().set_data("text/plain", &s).unwrap();
            }
            e.prevent_default();
        }) as Box<dyn FnMut(_)>);
        win
            .add_event_listener_with_callback("cut", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn subscribe_clipboard_events<H: WindowHandler + 'static>(_core: &Arc<Mutex<H>>) {
}