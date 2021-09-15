
use birdview::Context;
use birdview::{Color, ColorData};

use sdl2::{
    pixels,
    event::Event,
    keyboard::Keycode,
    image::ImageRWops,
};

use opencv as cv;
use cv::{
    core::Vector,
    prelude::*,
    imgcodecs,
    videoio::VideoCapture,
};



fn main() -> Result<(), Box<dyn std::error::Error>> {

    let Context { // Convenience for initializing sdl2
        sdl_context,
        video_subsystem: _video_subsystem,
        image_context: _image_context,
        mut canvas,
        texture_creator
    } = Context::new()?;

    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    
    let mut cap = VideoCapture::new(0, birdview::CAP_BACKEND)?;

    let mut frame = Mat::default();
    let mut proc_frame = Mat::default();

    // Stuff for encoding to bmp
    let mut framebuffer = Vector::new();
    let format_params = Vector::new();



    let col_data = ColorData::new(Color::red(), Color::green(), Color::blue());


    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => (),
            }
        }

        // Rest of the stuff

        canvas.clear();

        if !cap.read(&mut frame)? { break 'running }

        birdview::process_image(&frame, &col_data, 50, &mut proc_frame);

        // Ugly, horrendous, scary stuff I have to do because I don't want to handle internal image data
        // Encode to bmp, store in buffer, read from buffer into texture
        imgcodecs::imencode(".bmp", &proc_frame, &mut framebuffer, &format_params)?;

        let rw_ops = sdl2::rwops::RWops::from_bytes(framebuffer.as_slice())?;
        let texture = rw_ops.load_bmp()?.as_texture(&texture_creator)?;


        // Copy texture to screen and display it
        canvas.copy(&texture, None, None)?;

        canvas.present();
        //thread::sleep(Duration::from_millis(1000 / 60));
    }

    Ok(())
}
