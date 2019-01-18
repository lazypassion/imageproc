//! Displays an image in a window created by sdl2.

use image::{RgbaImage, imageops::resize};
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
    rect::Rect, surface::Surface
};

/// Displays the provided RGBA image in a new window.
///
/// The minimum window width or height is 150 pixels - input values less than this
/// will be rounded up to the minimum.
pub fn display_image(title: &str, image: &RgbaImage, window_width: u32, window_height: u32) {
    // Enforce minimum window size
    const MIN_WINDOW_DIMENSION: u32 = 150;
    let window_width = window_width.max(MIN_WINDOW_DIMENSION);
    let window_height = window_height.max(MIN_WINDOW_DIMENSION);

    // Initialise sdl2 window
    let sdl = sdl2::init().expect("couldn't create sdl2 context");
    let video_subsystem = sdl.video().expect("couldn't create video subsystem");

    let mut window = video_subsystem
        .window(title, window_width, window_height)
        .position_centered()
        .resizable()
        .build()
        .expect("couldn't create window");

    window
        .set_minimum_size(MIN_WINDOW_DIMENSION, MIN_WINDOW_DIMENSION)
        .expect("invalid minimum size for window");

    let mut canvas = window.into_canvas()
        .build()
        .expect("couldn't create canvas");

    let texture_creator = canvas.texture_creator();

    // Shrinks input image to fit if required and renders to the sdl canvas
    let mut render_image_to_canvas = |image, window_width, window_height| {
        let scaled_image = resize_to_fit(image, window_width, window_height);
        let (image_width, image_height) = scaled_image.dimensions();

        let mut buffer = scaled_image.into_raw();
        const CHANNEL_COUNT: u32 = 4;

        let surface = Surface::from_data(
            &mut buffer,
            image_width,
            image_height,
            image_width * CHANNEL_COUNT,
            PixelFormatEnum::ABGR8888 // sdl2 expects bits from highest to lowest
        ).expect("couldn't create surface");

        let texture = texture_creator
            .create_texture_from_surface(surface)
            .expect("couldn't create texture from surface");

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        let left = ((window_width - image_width) as f32 / 2f32) as i32;
        let top = ((window_height - image_height) as f32 / 2f32) as i32;
        canvas.copy(&texture, None, Rect::new(left, top, image_width, image_height)).unwrap();
        canvas.present();
    };

    render_image_to_canvas(image, window_width, window_height);

    // Create and start event loop to keep window open until Esc
    let mut event_pump = sdl.event_pump().unwrap();
    event_pump.enable_event(sdl2::event::EventType::Window);
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => {
                    render_image_to_canvas(image, w as u32, h as u32);
                }
                _ => {}
            }
        }
    }
}
/// Displays the provided RGBA images in new windows.
///
/// The minimum window width or height is 150 pixels - input values less than this
/// will be rounded up to the minimum.
pub fn display_multiple_images(title: &str, image: Vec<RgbaImage>, window_width: u32, window_height: u32) {
    // Enforce minimum window size
    const MIN_WINDOW_DIMENSION: u32 = 150;
    let window_width = window_width.max(MIN_WINDOW_DIMENSION);
    let window_height = window_height.max(MIN_WINDOW_DIMENSION);

    // Initialise sdl2 window
    let sdl = sdl2::init().expect("couldn't create sdl2 context");
    let video_subsystem = sdl.video().expect("couldn't create video subsystem");

    use sdl2::video::Window;
    use sdl2::render::WindowCanvas;
    use sdl2::video::WindowContext;

    let mut windows: Vec<sdl2::video::Window> = Vec::new();
    for _i in 0..image.len() {
        let mut window = video_subsystem
            .window(title, window_width, window_height)
            .position_centered()
            .resizable()
            .build()
            .expect("couldn't create window");

        window
            .set_minimum_size(MIN_WINDOW_DIMENSION, MIN_WINDOW_DIMENSION)
            .expect("invalid minimum size for window");
        windows.push(window);
    }
    // let mut window = video_subsystem
    //     .window(title, window_width, window_height)
    //     .position_centered()
    //     .resizable()
    //     .build()
    //     .expect("couldn't create window");
    //
    // window
    //     .set_minimum_size(MIN_WINDOW_DIMENSION, MIN_WINDOW_DIMENSION)
    //     .expect("invalid minimum size for window");
    let mut canvases: Vec<WindowCanvas> = Vec::new();
    // for i in 0..windows.len() {
    for window in windows.into_iter() {
        // let mut canvas = windows[i as usize].into_canvas()
        let canvas = window.into_canvas()
            .software()
            .build()
            .expect("couldn't create canvas");
        canvases.push(canvas);
    }


    // let mut canvas = window.into_canvas()
    //     .build()
    //     .expect("couldn't create canvas");

    let mut texture_creators: Vec<sdl2::render::TextureCreator<WindowContext>> = Vec::new();
    for i in 0..canvases.len() {
        let texture_creator = canvases[i as usize].texture_creator();
        texture_creators.push(texture_creator);
    }
    // let texture_creator = canvas.texture_creator();
    use sdl2::render::TextureCreator;
    use sdl2::render::Canvas;

    // Shrinks input image to fit if required and renders to the sdl canvas
    // let mut render_image_to_canvas = |image, window_width, window_height| {
    let mut render_image_to_canvas = |image, window_width, window_height, canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>| {
    // fn render_image_to_canvas(image: &RgbaImage, window_width: u32, window_height: u32, canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>) {
        let scaled_image = resize_to_fit(image, window_width, window_height);
        
        let (image_width, image_height) = scaled_image.dimensions();
        let mut buffer = scaled_image.into_raw();
        const CHANNEL_COUNT: u32 = 4;
        let surface = Surface::from_data(
            &mut buffer,
            image_width,
            image_height,
            image_width * CHANNEL_COUNT,
            PixelFormatEnum::ABGR8888 // sdl2 expects bits from highest to lowest
        ).expect("couldn't create surface");

        let texture = texture_creator
            .create_texture_from_surface(surface)
            .expect("couldn't create texture from surface");

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        let left = ((window_width - image_width) as f32 / 2f32) as i32;
        let top = ((window_height - image_height) as f32 / 2f32) as i32;
        canvas.copy(&texture, None, Rect::new(left, top, image_width, image_height)).unwrap();
        canvas.present();

    };
    // for (canvas, texture_creator) in canvases.iter().zip(texture_creators.iter()) {
    for (i, (mut canvas, texture_creator)) in canvases.into_iter().zip(texture_creators.iter()).enumerate() {
        render_image_to_canvas(&image[i], window_width, window_height, &mut canvas, texture_creator);
        // render_image_to_canvas(&image[i], window_width, window_height, canvases[i], texture_creators[i]); 
        // render_image_to_canvas(image, window_width, window_height, &canvas, &texture_creator);
    }
    
    // Create and start event loop to keep window open until Esc
    let mut event_pump = sdl.event_pump().unwrap();
    event_pump.enable_event(sdl2::event::EventType::Window);
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => {
                    // render_image_to_canvas(image, w as u32, h as u32);
                    // for (i, (mut canvas, texture_creator)) in canvases.iter().zip(texture_creators.iter()).enumerate() {
                    //     render_image_to_canvas(&image[i], w as u32, h as u32, &mut canvas, texture_creator);
                    //     // render_image_to_canvas(&image[i], window_width, window_height, canvases[i], texture_creators[i]); 
                    //     // render_image_to_canvas(image, window_width, window_height, &canvas, &texture_creator);
                    // }
                }
                _ => {}
            }
        }
    }
}

// Scale input image down if required so that it fits within a window of the given dimensions
fn resize_to_fit(image: &RgbaImage, window_width: u32, window_height: u32) -> RgbaImage {
    if image.height() < window_height && image.width() < window_width {
        return image.clone();
    }

    let scale = {
        let width_scale = window_width as f32 / image.width() as f32;
        let height_scale = window_height as f32 / image.height() as f32;
        width_scale.min(height_scale)
    };

    let height = (scale * image.height() as f32) as u32;
    let width = (scale * image.width() as f32) as u32;

    resize(image, width, height, image::FilterType::Triangle)
}
