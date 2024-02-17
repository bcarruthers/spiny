use std::sync::Arc;

use glam::UVec2;
use winit::{event_loop::EventLoop, window::{WindowBuilder, Window}};

pub struct Startup {
    pub window: Arc<Window>,
    pub event_loop: EventLoop<()>,
    pub is_web: bool,
}

impl Startup {
    pub fn new(title: &str, size: UVec2) -> Self {
        log::debug!("Creating window");

        // Create window and graphics context
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new()
            .with_title(title)
            // Note winit takes control of web canvas size
            .with_inner_size(winit::dpi::LogicalSize {
                width: size.x as i32,
                height: size.y as i32,
            })
            .with_visible(false)
            .build(&event_loop)
            .expect("Could not create window");
        let window = Arc::new(window);
        log::debug!("Scale factor: {}", window.scale_factor());

        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowExtWebSys;
            use wasm_bindgen::JsCast;

            let win = web_sys::window().expect("Could not get window");
            let body = win.document()
                .and_then(|doc| doc.body())
                .expect("Could not find body element").clone();

            // Winit wants complete control of the canvas to work properly, so we need
            // to resize through it whenever body size changes
            let body_size = {
                let body = body.clone();
                move || {
                    winit::dpi::LogicalSize::new(body.client_width(), body.client_height())
                }
            };

            window.set_inner_size(body_size());
        
            // On wasm, append the canvas to the document body
            let canvas = web_sys::Element::from(window.canvas());
            // https://stackoverflow.com/questions/23886278/disable-middle-mouse-click
            canvas.set_attribute("onmousedown", "if (e.which === 2){return false;}")
                .expect("Could not set attribute");
            canvas.set_attribute("oncontextmenu", "return false;")
                .expect("Could not set attribute");
            body.append_child(&canvas).ok();

            // https://github.com/a-b-street/abstreet/blob/master/widgetry/src/backend_glow_wasm.rs
            // https://github.com/a-b-street/abstreet/pull/388/files
            // Resize canvas when window changes
            {
                let window = window.clone();
                let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |e: web_sys::Event| {
                    log::debug!("Resized window: {:?}", e);
                    let size = body_size();
                    window.set_inner_size(size)
                }) as Box<dyn FnMut(_)>);
                win
                    .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
                    .unwrap();
                closure.forget();
            }
        }

        #[cfg(target_arch = "wasm32")]
        let is_web = true;
        #[cfg(not(target_arch = "wasm32"))]
        let is_web = false;
        Self {
            window,
            event_loop,
            is_web,
        }
    }
}
