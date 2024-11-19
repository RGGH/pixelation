use image::GenericImageView;
use image::{open, Rgba, RgbaImage};
use image::imageops::FilterType;
use minifb::{Key, Window, WindowOptions};

const PIXEL_SIZE:u32=5; //  bigger number more pixelated

fn main() {
    let img = open("assets/ex.png").unwrap();
    let (width, height) = img.dimensions();

    // Resize to create pixelated effect
    let resized = img.resize(width / PIXEL_SIZE, height / PIXEL_SIZE, FilterType::Nearest);
    let upscaled = resized.resize(width, height, FilterType::Nearest); // Upscale to original size

    let (width, height) = upscaled.dimensions();

    // Create a buffer to hold the pixels
    let mut buffer: Vec<u32> = vec![0; (width * height) as usize];

    for (x, y, pixel) in upscaled.pixels() {
        let rgba = pixel.0;
        let color = (rgba[0] as u32) << 16
            | (rgba[1] as u32) << 8
            | (rgba[2] as u32)
            | (rgba[3] as u32) << 24;
        buffer[(y * width + x) as usize] = color;
    }

    // Create an RgbaImage to save the result
    let mut output_image: RgbaImage = RgbaImage::new(width, height);

    // Fill the output image from the buffer
    for (x, y, pixel) in output_image.enumerate_pixels_mut() {
        let color = buffer[(y * width + x) as usize];
        let r = ((color >> 16) & 0xFF) as u8;
        let g = ((color >> 8) & 0xFF) as u8;
        let b = (color & 0xFF) as u8;
        let a = ((color >> 24) & 0xFF) as u8;
        *pixel = Rgba([r, g, b, a]);
    }

    // Save the pixelated image to a file
    output_image.save("pixelated_image.png").unwrap();

    // Create the window for display
    let mut window = Window::new(
        "Pixelated Image",
        width as usize,
        height as usize,
        WindowOptions::default(),
    )
    .unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, width as usize, height as usize)
            .unwrap();
    }
}

