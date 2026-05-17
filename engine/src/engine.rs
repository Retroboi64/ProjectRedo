use windowed::{ControlFlow, Event, Window, WindowConfig};

use crate::input::Input;
use crate::renderer::Renderer;

pub struct Engine {
    window: Window,
    pub renderer: Renderer,
    pub input: Input,
}

impl Engine {
    pub fn init() -> Self {
        let config = WindowConfig::new("ProjectRedo1")
            .fullscreen(true)
            .resizable(true);

        let window = Window::new(config).unwrap();
        let mut renderer = Renderer::new();

        unsafe {
            window.create_gl_context();
            gl::load_with(|s| window.get_proc_address(s) as *const _);
            window.test_gl();
        }

        renderer.init();

        Self {
            window,
            renderer,
            input: Input::new(),
        }
    }

    pub fn run(self, mut update: impl FnMut(&Input) -> bool) {
        let Engine {
            mut window,
            mut input,
            mut renderer,
            ..
        } = self;

        window
            .run(move |event, win| {
                match event {
                    Event::CloseRequested => {
                        return ControlFlow::Exit;
                    }

                    Event::KeyDown(key) => {
                        input.process_key_down(key);
                    }
                    Event::KeyUp(key) => {
                        input.process_key_up(key);
                    }

                    Event::RedrawRequested => {
                        if !update(&input) {
                            return ControlFlow::Exit;
                        }

                        renderer.run();

                        unsafe { win.swap_buffers() };

                        input.flush();
                    }

                    _ => {}
                }

                ControlFlow::Continue
            })
            .unwrap();
    }
}
