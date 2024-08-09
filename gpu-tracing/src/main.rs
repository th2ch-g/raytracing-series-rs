mod algebra;
mod render;
mod camera;

use anyhow::{Context, Result};
use winit::{
    event::{Event, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use crate::{algebra::Vec3, camera::Camera};

const WIDTH: u32 = 1600;
const HEIGHT: u32 = 1200;

#[pollster::main]
async fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    let window_size = winit::dpi::PhysicalSize::new(WIDTH, HEIGHT);
    let window = WindowBuilder::new()
        .with_inner_size(window_size)
        .with_resizable(false)
        .with_title("GPU Path Tracer".to_string())
        .build(&event_loop)?;

    // device, queue, surface
    use wgpu::TextureFormat::{Bgra8Unorm, Rgba8Unorm};
    let instance = wgpu::Instance::default();
    let surface = instance.create_surface(&window)?;
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
    })
    .await
    .context("failed to find a compatible adapter")?;

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .context("failed to connect to the GPU")?;

    let caps = surface.get_capabilities(&adapter);
    let format = caps
        .formats
        .into_iter()
        .find(|it| matches!(it, Rgba8Unorm | Bgra8Unorm))
        .context("could not find preferred texture format (Rgba8Unorm or Bgra8Unorm)")?;
    let size = window.inner_size();
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::AutoVsync,
        alpha_mode: caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 3,
    };

    surface.configure(&device, &config);
    //

    let mut renderer = render::PathTracer::new(device, queue, WIDTH, HEIGHT);
    let mut camera = Camera::look_at(
        Vec3::new(0., 0.75, 1.),
        Vec3::new(0., -0.5, -1.),
        Vec3::new(0., 1., 0.),
    );

    event_loop.run(|event, control_handle| {
        control_handle.set_control_flow(ControlFlow::Poll);
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => control_handle.exit(),
                WindowEvent::MouseWheel { delta, .. } => {
                    let delta = match delta {
                        MouseScrollDelta::PixelDelta(delta) => 0.001 * delta.y as f32,
                        MouseScrollDelta::LineDelta(_, y) => y * 0.1,
                    };
                    camera.zoom(delta);
                    renderer.reset_samples();
                }
                WindowEvent::RedrawRequested => {
                    let frame = surface
                        .get_current_texture()
                        .expect("failed to get current texture");

                    let render_target = frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());

                    renderer.render_frame(&camera, &render_target);

                    frame.present();
                    window.request_redraw();
                }
                _ => (),
            },
            _ => (),
        }
    })?;
    Ok(())
}
