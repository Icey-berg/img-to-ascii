use image::GenericImageView; // https://github.com/image-rs/image
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let img_path = &args[1];
    let compress_factor: u32 = *&args[2].parse().unwrap(); // If compress factor is 10, print an ascii character every 10 pixels

    println!("Converting IMG at: {}", img_path);

    let img = image::open(img_path).unwrap();

    let brightness_characters = " $B:. ";
    let dimensions = img.dimensions();

    for (x, y, color) in img.pixels() {
        if x % compress_factor == 0 && y % compress_factor == 0 {
            let brightness: f32 = get_brightness(color);

            // Get the ascii character to print judging from the brightness variable
            let mut icon: String = String::from("");
            for (index, character) in brightness_characters.char_indices().rev() {
                // turns the index into brightness. The higher the index, the lower the brightness
                let char_brightness = 1 as f32 / index as f32;

                icon = character.to_string();

                if brightness <= char_brightness {
                    break;
                }
            }

            print!("{}", icon);

            // check if the current pixel is at the end of the image (x axis) and start a new line if so
            if dimensions.0 == x || (x + compress_factor) > dimensions.0 {
                print!("\n")
            }
        }
    }
}

// By adding all the RGB values and by dividing them by the 765 (255 * 3, max brightness) we can get a normalised brightness value
fn get_brightness(pixel: image::Rgba<u8>) -> f32 {
    (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / 765 as f32
}
