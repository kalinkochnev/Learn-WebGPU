mod state;
mod pipelines;
mod vertex;
mod linalg;

use crate::state::State;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(window).await;
    event_loop.run(move |event, _, control_flow| {
        let app_window = state.window.id().clone();
        let same_window = | window_id | window_id == app_window;
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if same_window(window_id) && !state.input(event) => {
                match event {
                    WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                                ..
                        } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => state.resize(*physical_size),
                    // new_inner_size is &&mut
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => state.resize(**new_inner_size),
                    _ => {}
                }
            },
            Event::RedrawRequested(window_id) if same_window(window_id) => {
                state.update();

                match state.render() {
                    Ok(_) => {}
                    // Reconfigure if the surface is lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,

                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            },
            Event::MainEventsCleared => {
                state.window.request_redraw();
            },
            _ => {}
        }
    });
}

fn main() {
    pollster::block_on(run())
}
