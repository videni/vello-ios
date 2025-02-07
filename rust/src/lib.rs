use std::{ffi::CString, mem::ManuallyDrop, num::NonZeroUsize, ptr, time::Instant};
use scenes::{test_scenes, ImageCache, SceneParams, SimpleText};
use app_surface::{AppSurface, IOSViewObj, SurfaceFrame};
use vello::{kurbo::{Affine as KurboAffine, Vec2}, peniko::Color, AaConfig, Renderer, RendererOptions, Scene};
use vello::wgpu::TextureFormat;
use std::os::raw::c_char;
use std::ffi::CStr;
use tracing_subscriber;

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
pub extern "C" fn App_create(ios_obj: IOSViewObj, assets_dir: *const c_char) -> *mut libc::c_void {
    let assets_dir: &CStr = unsafe { CStr::from_ptr(assets_dir) };
    
    tracing_subscriber::fmt::fmt()
        .pretty()
        .init();

    println!("{}", assets_dir.to_str().unwrap());
    
    std::env::set_var("RUST_BACKTRACE", "full".to_string());
    std::env::set_var("RUST_LOG", "TRACE".to_string());

    println!(
        "AppSurface_create, maximum frames: {}",
        ios_obj.maximum_frames
    );
    // Failed to create renderer: Couldn't find `Rgba8Unorm` or `Bgra8Unorm` texture formats for surface
    
    // Vello not implemented Bgra8UnormSrgb yet，so we hard-coded it as Bgra8Unorm
    let format = TextureFormat::Bgra8Unorm;

    let mut app_surface = AppSurface::new(ios_obj);
    let ctx = &mut app_surface.ctx ;
    ctx.update_config_format(format);

    let obj  = App::new(app_surface, format);
    
    let box_obj = Box::new(obj);

    Box::into_raw(box_obj) as *mut libc::c_void
}

const AA_CONFIGS: [AaConfig; 3] = [AaConfig::Area, AaConfig::Msaa8, AaConfig::Msaa16];

#[no_mangle]
pub extern "C" fn App_render(
    app: *mut App, 
    scene_idx: u32, 
    bounds: Rectangle, 
    scale_factor: f32, 
    affine: Affine,
) {  

    println!("App_render {:?}", bounds);
    dbg!(Vec2::new(bounds.width as f64, bounds.height as f64));

    let app = unsafe { &mut *(app as *mut App) };
    app.app_surface.resize_surface();

    let mut scenes: Vec<scenes::ExampleScene> = test_scenes::test_scenes().scenes;
    
    let scene_ix = scene_idx.rem_euclid(scenes.len() as u32);

    let example_scene = &mut scenes[scene_ix as usize];
    
    let transform = KurboAffine::IDENTITY;

    let title = format!("Vello demo - {}", example_scene.config.name);
    println!("{}", title);

    let mut fragment =  Scene::new();

    let mut simple_text = SimpleText::new();
    let mut images =  ImageCache::new();

    let mut scene_params = SceneParams {
        time: Instant::now().elapsed().as_secs_f64(),
        text: &mut simple_text,
        images: &mut images,
        resolution: Some(Vec2::new(bounds.width as f64, bounds.height as f64)),
        base_color: None,
        interactive: true,
        complexity: 0,
    };
    example_scene
        .function
        .render(&mut fragment, &mut scene_params);

    dbg!("hello", app.format);
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
            &*app.app_surface.device,
            &*app.app_surface.queue,
            &fragment,
            &surface,
            &render_params,
        )
        .expect("failed to render to surface");

    surface.present();
}

#[repr(C)]
pub struct ExampleScene
{
    pub name: *const c_char,
}

#[repr(C)]
pub struct Array {
    ptr: *mut libc::c_void,
    len: usize,
    cap: usize,
}

#[no_mangle]
pub extern "C" fn scenes() -> Array
{
    let scenes: Vec<ExampleScene> = test_scenes::test_scenes()
        .scenes
        .iter()
        .map(|s| ExampleScene{
            name:  CString::new(s.config.name.clone()).unwrap().into_raw() as *mut c_char,
        }).collect::<Vec<ExampleScene>>();

    let mut v = ManuallyDrop::new(scenes);
    let (ptr, len, cap) = (v.as_mut_ptr(), v.len(), v.capacity());
    
    Array {
        ptr: ptr as *mut libc::c_void,
        len,
        cap
    }
}