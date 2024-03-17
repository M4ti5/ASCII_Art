use colored::*;
use image::{imageops::FilterType, GenericImageView, Pixel};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (image_path, new_width): (String, u32);

    // Check for "demo" argument or use default behavior
    if args.len() == 2 && args[1] == "demo" {
        image_path = "mimic.webp".to_string(); // Default demo image path
        new_width = 125; // Default demo width
    } else if args.len() >= 3 {
        image_path = args[1].clone();
        new_width = args[2].parse().expect("Failed to parse new_width");
    } else {
        eprintln!("Usage: {} <image_path> <width> | demo", args[0]);
        std::process::exit(1);
    }
    match image_to_ascii_color(&image_path, new_width) {
        Ok(ascii_art) => println!("{}", ascii_art),
        Err(e) => eprintln!("Error converting image to ASCII: {}", e),
    }
}

fn image_to_ascii_color(image_path: &str, new_width: u32) -> Result<String, Box<dyn std::error::Error>> {
    // Open the image and get its dimensions
    let img: image::DynamicImage = image::open(image_path)?;
    let (width, height) = img.dimensions();
    
    // Adjust for the aspect ratio of characters in terminal
    let aspect_ratio: f32 = 0.5;
    let new_height: u32 = ((height as f32 / width as f32) * new_width as f32 * aspect_ratio) as u32;
    
    // Resize the image to new dimensions
    let resized_image: image::DynamicImage = img.resize_exact(new_width, new_height, FilterType::Nearest);

    let ascii_chars: [char; 4] = ['█', '▓', '▒', '░'];
    let mut ascii_image: String = String::new();
    ascii_image.reserve(new_width as usize * new_height as usize); // Pre-allocate space for efficiency

    for y in 0..new_height {
        for x in 0..new_width {
            let pixel: image::Rgba<u8> = resized_image.get_pixel(x, y).to_rgba();
            let grayscale: f32 = (0.299 * (pixel[0] as f32) + 0.587 * (pixel[1] as f32) + 0.114 * (pixel[2] as f32)) / 255.0;
            let ascii_char: char = ascii_chars[(grayscale * ((ascii_chars.len() - 1) as f32)) as usize];
            
            // Convert the character to a colored string, then immediately to a String for appending
            let color_string: String = format!("{}", ascii_char).truecolor(pixel[0], pixel[1], pixel[2]).to_string();
            ascii_image += &color_string;
        }
        ascii_image.push('\n'); // Append a newline at the end of each row
    }

    Ok(ascii_image)
}
