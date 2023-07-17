use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};

fn main() {
    windows_nondefault_desktop::assume_active_desktop();

    let event_loop = EventLoop::new();
    let _window = Window::new(&event_loop).unwrap();

    event_loop.run(|event, _, control_flow| {
        control_flow.set_wait();
        match event {
            Event::WindowEvent {
                window_id: _,
                event,
            } => match event {
                WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                }
                _ => {}
            },
            _ => {}
        }
    })
}
