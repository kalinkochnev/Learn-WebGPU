mod state;

use crate::state::State;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder},
};

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(window).await;
    event_loop.run(move |event, _, control_flow| { 
        let same_window = | window_id | window_id == state.window.id();
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if same_window(window_id) => {
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
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size } => {
                        // new_inner_size is &&mut
                        state.resize(**new_inner_size)
                    }
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
            }
            _ => {}
        }
    });
}

fn main() {
    pollster::block_on(run())
}
