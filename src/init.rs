
use sdl2::image;

#[cfg(not(target_os="linux"))]
pub const CAP_BACKEND: i32 = opencv::videoio::CAP_ANY;
#[cfg(target_os="linux")]
pub const CAP_BACKEND: i32 = opencv::videoio::CAP_V4L2;

pub struct Context {
    pub sdl_context: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub image_context: sdl2::image::Sdl2ImageContext,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
}

impl Context {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let image_context = sdl2::image::init(image::InitFlag::JPG)?;

        let canvas = video_subsystem.window("BirdView Client", 800, 600)
            .position_centered()
            .build()?
            .into_canvas()
            .build()?;
        
        let texture_creator = canvas.texture_creator();


        Ok(Self {
            sdl_context,
            video_subsystem,
            image_context,
            canvas,
            texture_creator
        })
    }
}