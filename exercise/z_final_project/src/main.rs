// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

use clap::Parser;
use image::{DynamicImage, ImageError};
use mirage::args::Args;

fn main() -> Result<(), ImageError> {
    let args = Args::parse();

    println!("Using input file: {}", args.infile);
    println!("Using output file: {}", args.outfile);

    if args.generate || args.fractal {  // These operations do not require an input file
        if args.fractal {
            fractal(&args.outfile);
        }

        if args.generate {
            generate(&args.outfile);
        }
    } else {
        let mut img = image::open(args.infile.clone()).expect("Failed to open INFILE.");

        if args.blur != 0.0 {
            img = img.blur(args.blur);
        }

        if args.brighten != 0 {
            img = img.brighten(args.brighten);
        }

        if args.invert {
            img.invert();
        }

        if args.grayscale {
            img = img.grayscale();
        }

        if args.rotation != 0.0 {
            img = rotate(img.clone(), args.rotation);
        }

        if args.width != 0 && args.height != 0 {
            img = img.crop(args.x_offset, args.y_offset, args.width, args.height);
        } else if args.width != 0 || args.height != 0 {
            println!("Crop requires a width and height");
        }
        img.save(args.outfile).expect("Failed writing OUTFILE.");
    }

    Ok(())
}

fn rotate(img: DynamicImage, rotation: f32) -> DynamicImage {
    if rotation <= 90.0 {
        img.rotate90()
    } else if rotation <= 180.0 {
        img.rotate180()
    } else {
        img.rotate270()
    }
}

fn generate(outfile: &String) {
    // Create an ImageBuffer -- see fractal() for an example
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    // Iterate over the coordinates and pixels of the image -- see fractal() for an example
    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        // let red = (0.3 * x as f32) as u8;
        // let green = (0.3 * x as f32) as u8;
        // let blue = (0.3 * x as f32) as u8;

        // Set the image to some solid color. -- see fractal() for an example
        let red: u8 = 50;
        let blue: u8 = 50;
        let green: u8 = 0;

        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: &String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}
