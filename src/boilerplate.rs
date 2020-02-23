use winit::{
    event,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use futures::executor::{LocalPool, LocalSpawner};
use wgpu;

const DPI_FACTOR : u32 = 10;
const SCREEN_WIDTH : u32 = 64;
const SCREEN_HEIGHT : u32 = 32;

const WINDOW_SIZE : (u32, u32) = (
    SCREEN_WIDTH * DPI_FACTOR,
    SCREEN_HEIGHT * DPI_FACTOR);

pub const COLOR_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8Unorm;
pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;



pub trait Application {
    fn on_key(&mut self, input: event::KeyboardInput) -> bool;
    fn resize(&mut self, device: &wgpu::Device, extent: wgpu::Extent3d) {}
    fn reload(&mut self, device: &wgpu::Device);
    fn update(&mut self, device: &wgpu::Device, delta: f32) -> Vec<wgpu::CommandBuffer>;
    fn draw(&mut self, device: &wgpu::Device) -> wgpu::CommandBuffer;
}

pub struct Harness {
    event_loop: EventLoop<()>,
    window: Window,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    surface: wgpu::Surface,
    swap_chain: wgpu::SwapChain,
    pub extent: wgpu::Extent3d,
    reload_on_focus: bool,
    depth_target: wgpu::TextureView,
}

impl Harness {
    pub fn new(title: &str) -> Self {
        let extent = wgpu::Extent3d {
            width: WINDOW_SIZE.0,
            height: WINDOW_SIZE.1,
            depth: 1,
        };

        let adapter = wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
            },
            wgpu::BackendBit::PRIMARY,
        ).unwrap();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: wgpu::Limits::default(),
        });

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(
                winit::dpi::PhysicalSize::new(extent.width, extent.height),
            )
            .with_resizable(true)
            .build(&event_loop)
            .unwrap();

        let surface = wgpu::Surface::create(&window);
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: COLOR_FORMAT,
            width: extent.width,
            height: extent.height,
            present_mode: wgpu::PresentMode::Vsync,
        };

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);
        let depth_target = device
            .create_texture(&wgpu::TextureDescriptor {
                size: extent,
                array_layer_count: 1,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: DEPTH_FORMAT,
                usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            })
            .create_default_view();

        Harness {
            event_loop,
            window,
            device,
            queue,
            surface,
            swap_chain,
            extent,
            reload_on_focus: true,
            depth_target
        }
    }

    pub fn main_loop<A: 'static + Application>(self, mut app: A) {
        use std::time;

        let mut last_time = time::Instant::now();
        let mut task_pool = LocalPool::new();
        let mut needs_reload = false;
        let Harness {
            event_loop,
            device,
            queue,
            surface,
            mut swap_chain,
            mut extent,
            reload_on_focus,
            mut depth_target,
            ..
        } = self;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            task_pool.run_until_stalled();

            match event {
                event::Event::WindowEvent {
                    event: event::WindowEvent::Resized(size),
                    ..
                } => {
                    extent = wgpu::Extent3d {
                        width: size.width,
                        height: size.height,
                        depth: 1,
                    };
                    let sc_desc = wgpu::SwapChainDescriptor {
                        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                        format: COLOR_FORMAT,
                        width: size.width,
                        height: size.height,
                        present_mode: wgpu::PresentMode::Vsync,
                    };
                    swap_chain = device.create_swap_chain(&surface, &sc_desc);
                    depth_target = device
                        .create_texture(&wgpu::TextureDescriptor {
                            size: extent,
                            array_layer_count: 1,
                            mip_level_count: 1,
                            sample_count: 1,
                            dimension: wgpu::TextureDimension::D2,
                            format: DEPTH_FORMAT,
                            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                        })
                        .create_default_view();
                    app.resize(&device, extent);
                }
                event::Event::WindowEvent { event, .. } => match event {
                    event::WindowEvent::Focused(false) => {
                        needs_reload = reload_on_focus;
                    }
                    event::WindowEvent::Focused(true) if needs_reload => {
                        app.reload(&device);
                        needs_reload = false;
                    }
                    event::WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    event::WindowEvent::KeyboardInput { input, .. } => {
                        if !app.on_key(input) {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                    _ => {}
                },
                event::Event::MainEventsCleared => {
                }
                _ => (),
            }
        });
    }
}