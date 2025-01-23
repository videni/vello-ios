use std::{num::NonZeroUsize, time::Instant};

use scenes::{test_scenes, ImageCache, SceneParams, SimpleText};
use app_surface::{AppSurface, IOSViewObj, SurfaceFrame};
use vello::{kurbo::{Affine as KurboAffine, Vec2}, peniko::Color, AaConfig, Renderer, RendererOptions, Scene};
use vello::wgpu::TextureFormat;

pub struct App
{
    app_surface: AppSurface,
    format: TextureFormat,
}

impl App {
    pub fn new(
        app_surface: AppSurface,
        format: TextureFormat,
    ) -> Self {
        let ctx = &app_surface.ctx;
        
        Self {
            app_surface,
            format,
        }
    }
}


#[repr(C)]
pub struct Affine
{
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    tx: f32,
    ty: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rectangle<T = f32> {
    /// X coordinate of the top-left corner.
    pub x: T,

    /// Y coordinate of the top-left corner.
    pub y: T,

    /// Width of the rectangle.
    pub width: T,

    /// Height of the rectangle.
    pub height: T,
}

#[no_mangle]
pub extern "C" fn App_create(ios_obj: IOSViewObj) -> *mut libc::c_void {
    println!(
        "AppSurface_create, maximum frames: {}",
        ios_obj.maximum_frames
    );
    
    // vello not implemented Bgra8UnormSrgb yet，so we hard-coded it as Bgra8Unorm
    let format = TextureFormat::Bgra8Unorm;

    let obj  = App::new(AppSurface::new(ios_obj), format);
    
    let box_obj = Box::new(obj);

    Box::into_raw(box_obj) as *mut libc::c_void
}

const AA_CONFIGS: [AaConfig; 3] = [AaConfig::Area, AaConfig::Msaa8, AaConfig::Msaa16];

#[no_mangle]
pub extern "C" fn App_render(pdf: *mut App, 
    scene_idx: u32, 
    bounds: Rectangle, 
    scale_factor: f32, 
    affine: Affine,
    page_no: u32,
    clip_bounds: Rectangle<u32>
) {
    let app = unsafe { &mut *(pdf as *mut App) };

    let mut scenes = test_scenes::test_scenes().scenes;
    
    let scene_ix = scene_idx.rem_euclid(scenes.len() as u32);

    let example_scene = &mut scenes[scene_ix as usize];
    
    let transform = KurboAffine::IDENTITY;

    let title = format!("Vello demo - {}", example_scene.config.name);

    let mut fragment =  Scene::new();

    let mut simple_text = SimpleText::new();
    let mut images =  ImageCache::new();

    let mut scene_params = SceneParams {
        time: Instant::now().elapsed().as_secs_f64(),
        text: &mut simple_text,
        images: &mut images,
        resolution: None,
        base_color: None,
        interactive: true,
        complexity: 0,
    };
    example_scene
        .function
        .render(&mut fragment, &mut scene_params);

    let mut renderer = Renderer::new(
        &app.app_surface.device,
        RendererOptions {
            surface_format: Some(app.format),
            use_cpu: false,
            antialiasing_support: AA_CONFIGS.iter().copied().collect(),
            num_init_threads: None,
        },
    )
    .map_err(|e| {
        // Pretty-print any renderer creation error using Display formatting before unwrapping.
        anyhow::format_err!("{e}")
    })
    .expect("Failed to create renderer");

    let (surface, _texture_view) = app.app_surface.get_current_frame_view(Some(app.format));

    let antialiasing_method = AA_CONFIGS[0 as usize];

    let render_params = vello::RenderParams {
        base_color: Color::BLACK,
        width: bounds.width as u32,
        height: bounds.height as u32,
        antialiasing_method,
    };
    renderer
        .render_to_surface(
            &app.app_surface.device,
            &app.app_surface.queue,
            &fragment,
            &surface,
            &render_params,
        )
        .expect("failed to render to surface");
    surface.present();
}
