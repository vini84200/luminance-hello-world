use glfw::{Action, Context as _, Key, WindowEvent};
use luminance::context::GraphicsContext;
use luminance::pipeline::PipelineState;
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};
use std::process::exit;
use std::time::Instant;

fn main() {
    let dim = WindowDim::Windowed {
        width: 960,
        height: 540,
    };
    let surface =  GlfwSurface::new_gl33("Hello World!", WindowOpt::default().set_dim(dim));

    match surface {
        Ok(surface) => {
            println!("graphics surface created");
            main_loop(surface)
        }

        Err(e) => {
            println!("cannot create graphics surface: \n {}", e);
            exit(1);
        }
    }
}

fn main_loop(mut surface: GlfwSurface) {
    let start_t = Instant::now();
    let back_buffer = surface.back_buffer().unwrap();

    'app: loop {
        surface.window.glfw.poll_events();
        for (_, event) in surface.events_rx.try_iter() {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Press, _) => break 'app,
                _ => ()
            }
        }

        // Rendering Code
        let t = start_t.elapsed().as_millis() as f32 * 1e-3;
        let color = [t.cos(), t.sin(), 0.5, 1.];

        let render = surface.new_pipeline_gate().pipeline(
            &back_buffer,
            &PipelineState::default().set_clear_color(color),
            |_, _| Ok(()),
        ).assume();

        if render.is_ok() {

            surface.window.swap_buffers();
        } else {
            break 'app;
        }
    }
}
