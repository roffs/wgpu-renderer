mod app;
mod camera;
mod entity;
mod gpu_context;
mod layouts;
mod light;
mod material;
mod render_pass;
mod render_world;
mod resources;
mod scene;
mod surface_context;
mod texture;
mod transform;
mod window_context;

use app::App;
use gpu_context::GpuContext;
use surface_context::SurfaceContext;
use window_context::WindowContext;
use winit::event::{Event, WindowEvent};

fn main() {
    let window_loop = WindowContext::new();
    let mut surface = SurfaceContext::new();
    let context = GpuContext::new(&surface);
    surface.init(&context, window_loop.window.clone());

    let mut app = App::new(&context, &surface);

    window_loop
        .event_loop
        .run(move |event, elwt| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => elwt.exit(),
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                let output = surface.get().unwrap().get_current_texture().unwrap();
                let view = output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                app.render(&context.device, &context.queue, &view);

                output.present();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                let width = size.width;
                let height = size.height;

                if width > 0 && height > 0 {
                    surface.configure(&context.device, width, height);
                    app.resize(&context.device, width, height);
                }
            }
            Event::AboutToWait => window_loop.window.request_redraw(),
            event => app.update(event, elwt),
        })
        .unwrap();
}
