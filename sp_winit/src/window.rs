use std::{time::Duration, sync::{Mutex, Arc}, ops::DerefMut, rc::Rc};
use glam::UVec2;
use winit::{
    event::*,
    event_loop::ControlFlow
};
use crate::clipboard::Clipboard;

pub struct WindowUpdateInput {
    pub time: i64,
    pub size: UVec2,
}

pub struct WindowUpdateResult {
    pub fullscreen: bool,
    pub closing: bool,
    pub pasting: bool,
    pub clipboard_data: Option<String>,
}

pub trait WindowHandler {
    fn copy(&mut self) -> Option<String> {
        None
    }

    fn cut(&mut self) -> Option<String> {
        None
    }

    fn paste(&mut self, _data: String) -> bool {
        false
    }

    fn handle(&mut self, _event: sp_input::WindowEvent) {
    }

    fn update(&mut self, _input: WindowUpdateInput) -> WindowUpdateResult {
        WindowUpdateResult {
            fullscreen: false,
            closing: false,
            pasting: false,
            clipboard_data: None,
        }
    }
}

fn handle_event<H: WindowHandler>(
    time: i64,
    window: &winit::window::Window,
    event: Event<()>,
    sleep_duration: Duration,
    clipboard: &mut Clipboard,
    handler: &mut H,
) -> bool{
    match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            if let Some(event) = super::convert::convert_window_event(&event) {
                // Audio requires user interaction for web
                let interacting =
                    match event {
                        sp_input::WindowEvent::KeyboardInput(_) |
                        sp_input::WindowEvent::MouseWheel(_) |
                        sp_input::WindowEvent::MouseInput(_) |
                        sp_input::WindowEvent::Touch(_) => true,
                        _ => false,
                    };
                if interacting {
                    //audio.enable();
                }
                handler.handle(event);
            }
        }
        Event::DeviceEvent {
            device_id: _,
            event,
        } => {
            if let Some(event) = super::convert::convert_device_event(&event) {
                handler.handle(event);
            }
        }
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            // Render output
            let size = window.inner_size();
            let size = UVec2::new(size.width as u32, size.height as u32);
            let input = WindowUpdateInput { time, size };
            let frame = handler.update(input);

            if frame.fullscreen {
                match window.fullscreen() {
                    None => window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None))),
                    Some(_) => window.set_fullscreen(None),
                }
            }
            
            handler.handle(sp_input::WindowEvent::FullScreenChanged(window.fullscreen().is_some()));

            // Clipboard
            if frame.pasting {
                if let Some(s) = clipboard.copy_from_clipboard() {
                    handler.handle(sp_input::WindowEvent::PasteFromClipboard(s))
                }
            }

            if let Some(s) = frame.clipboard_data {
                clipboard.copy_to_clipboard(s);
            }

            if frame.closing {
                return false
            }

            // Sleep to avoid spinning CPU (do this outside message so
            // it's not included in actor timings)
            // let span = tracing::span!(tracing::Level::DEBUG, "fg_sleep");
            // let _enter = span.enter();

            #[cfg(not(target_arch = "wasm32"))]
            spin_sleep::sleep(sleep_duration);

            // match state.render() {
            //     Ok(_) => {}
            //     // Reconfigure the surface if lost
            //     Err(wgpu::SurfaceError::Lost) => state.resize(state.actor.size),
            //     // The system is out of memory, we should probably quit
            //     Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
            //     // All other errors (Outdated, Timeout) should be resolved by the next frame
            //     Err(e) => eprintln!("{:?}", e),
            // }
        }
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually request it
            window.request_redraw();
        }
        _ => {}
    }
    true
}

pub async fn run<H: 'static + WindowHandler>(
    handler: Arc<Mutex<H>>,
    window: Rc<winit::window::Window>,
    event_loop: winit::event_loop::EventLoop<()>,
    start: instant::Instant,
    //enable_clipboard: bool,
) {
    let sleep_duration = Duration::from_millis(1);
    let mut clipboard = Clipboard::new();

    // Set initial size/scale
    log::debug!("Initializing core");
    let size = UVec2::new(window.inner_size().width, window.inner_size().height);
    {
        let mut handler = handler.lock().unwrap();
        handler.handle(sp_input::WindowEvent::Resized(size));
        handler.handle(sp_input::WindowEvent::ScaleFactorChanged(window.scale_factor() as f32));
        handler.handle(sp_input::WindowEvent::FullScreenChanged(window.fullscreen().is_some()));
    }
    // if enable_clipboard {
    //     super::web::subscribe_clipboard_events(&handler);
    // }

    // Wait until this point to display to avoid white flash on Windows
    window.set_visible(true);

    // Run main loop
    log::debug!("Starting main loop");
    event_loop.run(move |event, _, control_flow| {
        let time = (instant::Instant::now() - start).as_millis() as i64;
        // Handle window event
        let mut handler = handler.lock().unwrap();
        let running = handle_event(
            time,
            &window,
            event,
            sleep_duration,
            &mut clipboard,
            handler.deref_mut(),
        );
        // Close if no longer running. Note we could also handle this as a message.
        if !running {
            // If left in fullscreen, the window won't close on MacOS
            window.set_fullscreen(None);
            *control_flow = ControlFlow::Exit;
        }
    });
}
