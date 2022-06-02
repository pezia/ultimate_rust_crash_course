use clap::{Arg, Command};
use image::{DynamicImage, ImageBuffer, Rgb};

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
const ERROR_OUTFILE: &'static str = "Failed writing OUTFILE.";
const ERROR_INFILE: &'static str = "Failed to open INFILE.";

fn main() {
   /* let matches = Command::new(env!("CARGO_CRATE_NAME"))
        .about("Mirage Image Processor")
        .subcommand(
            Command::new("generate")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("INFILE")
                        .short('i')
                        .long("infile")
                        .help("input image file path")
                        .takes_value(true)
                        .use_value_delimiter(false)
                ).arg(
                    Arg::new("OUTFILE")
                        .short('o')
                        .long("outfile")
                        .help("output image file path")
                        .takes_value(true)
                        .default_missing_value("out.png")
                        .use_value_delimiter(false)
                )
        )
        .get_matches();

    let mut subcommand = matches.subcommand();

    println!("{:?}", subcommand);*/

    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }
    let subcommand = args.remove(0);
    match subcommand.as_str() {
        // EXAMPLE FOR CONVERSION OPERATIONS
        "blur" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let sigma: f32 = args.remove(0).parse().expect("Cannot parse SIGMA");

            let mut img = image::open(infile).expect(ERROR_INFILE);

            blur(&mut img, sigma).save(outfile).expect("Failed writing OUTFILE.");
        }
        "brighten" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let brightness: i32 = args.remove(0).parse().expect("Cannot parse BRIGHTNESS");

            let mut img = image::open(infile).expect(ERROR_INFILE);

            brighten(&mut img, brightness).save(outfile).expect(ERROR_OUTFILE);
        }

        // **OPTION**
        // Crop -- see the crop() function below
        "crop" => {
            if args.len() != 6 {
                print_usage_and_exit();
            }

            let infile = args.remove(0);
            let outfile = args.remove(0);
            let x: u32 = args.remove(0).parse().expect("Cannot parse X");
            let y: u32 = args.remove(0).parse().expect("Cannot parse Y");
            let width: u32 = args.remove(0).parse().expect("Cannot parse WIDTH");
            let height: u32 = args.remove(0).parse().expect("Cannot parse HEIGHT");

            let mut img = image::open(infile).expect(ERROR_INFILE);

            crop(&mut img, x, y, width, height).save(outfile).expect(ERROR_OUTFILE);
        }

        // **OPTION**
        // Rotate -- see the rotate() function below
        "rotate" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }

            let infile = args.remove(0);
            let outfile = args.remove(0);
            let degrees: i32 = args.remove(0).parse().expect("Cannot parse DEGREES");

            let mut img = image::open(infile).expect(ERROR_INFILE);

            rotate(&mut img, degrees).save(outfile).expect(ERROR_OUTFILE);
        }

        // **OPTION**
        // Invert -- see the invert() function below
        "invert" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }

            let infile = args.remove(0);
            let outfile = args.remove(0);

            let mut img = image::open(infile).expect(ERROR_INFILE);

            invert(&mut img).save(outfile).expect(ERROR_OUTFILE);
        }

        // **OPTION**
        // Grayscale -- see the grayscale() function below
        "grayscale" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }

            let infile = args.remove(0);
            let outfile = args.remove(0);

            let mut img = image::open(infile).expect(ERROR_INFILE);

            grayscale(&mut img).save(outfile).expect(ERROR_OUTFILE);
        }

        // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
        "fractal" => {
            if args.len() != 1 {
                print_usage_and_exit();
            }
            let outfile = args.remove(0);
            fractal().save(outfile).expect(ERROR_OUTFILE);
        }
        "generate" => {
            if args.len() != 1 {
                print_usage_and_exit();
            }
            let outfile = args.remove(0);
            generate().save(outfile).expect(ERROR_OUTFILE);
        }

        // For everything else...
        _ => {
            print_usage_and_exit();
        }
    }
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");

    println!("blur INFILE OUTFILE SIGMA");
    println!("brighten INFILE OUTFILE BRIGHTNESS");
    println!("crop INFILE OUTFILE X Y WIDTH HEIGHT");
    println!("fractal OUTFILE");
    println!("generate OUTFILE");
    println!("grayscale INFILE OUTFILE");
    println!("invert INFILE OUTFILE");
    println!("rotate INFILE OUTFILE DEGREES");

    std::process::exit(-1);
}

fn blur(img: &mut DynamicImage, sigma: f32) -> DynamicImage {
    img.blur(sigma)
}

fn brighten(img: &mut DynamicImage, brightness: i32) -> DynamicImage {
    img.brighten(brightness)
}

fn crop(img: &mut DynamicImage, x: u32, y: u32, width: u32, height: u32) -> DynamicImage {
    img.crop(x, y, width, height)
}

fn rotate(img: &mut DynamicImage, degrees: i32) -> DynamicImage {
    match degrees {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => (*img).clone(),
    }
}

fn invert(img: &mut DynamicImage) -> DynamicImage {
    img.invert();
    (*img).clone()
}

fn grayscale(img: &mut DynamicImage) -> DynamicImage {
    img.grayscale()
}

fn generate() -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    // Create an ImageBuffer -- see fractal() for an example
    let width = 1024;
    let height = 768;
    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([255, 0, 0]);
    }

    imgbuf
    // Iterate over the coordinates and pixels of the image -- see fractal() for an example

    // Set the image to some solid color. -- see fractal() for an example

    // Challenge: parse some color data from the command-line, pass it through
    // to this function to use for the solid color.

    // Challenge 2: Generate something more interesting!

    // See blur() for an example of how to save the image
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal() -> ImageBuffer<Rgb<u8>, Vec<u8>> {
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

    imgbuf
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
