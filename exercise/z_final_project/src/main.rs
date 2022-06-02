use clap::{Arg, arg, Command};
use image::{DynamicImage, ImageBuffer, Rgb};

const ERROR_OUTFILE: &'static str = "Failed writing OUTFILE.";
const ERROR_INFILE: &'static str = "Failed to open INFILE.";

fn main() {
    let cmd = Command::new(env!("CARGO_CRATE_NAME"))
        .about("Mirage Image Processor")
        .arg_required_else_help(true)
        .allow_hyphen_values(true)
        .arg(
            Arg::new("infile")
                .help("input image file path")
                .takes_value(true)
                .required(true)
                .use_value_delimiter(false)
        )
        .arg(
            Arg::new("outfile")
                .help("output image file path")
                .takes_value(true)
                .required(true)
                .default_missing_value("out.png")
                .use_value_delimiter(false)
        )
        .arg(
            arg!(<OPERATIONS> ... "Operations")
        );

    let matches = cmd.get_matches();

    let infile = matches.value_of("infile").expect("infile is required");
    let mut img = image::open(infile).expect(ERROR_INFILE);

    let outfile = matches.value_of("outfile").expect("outfile is required");

    let mut args: Vec<&str> = matches.values_of("OPERATIONS").unwrap().collect();

    loop {
        if args.is_empty() {
            break;
        }

        let subcommand = args.remove(0);

        match subcommand {
            // EXAMPLE FOR CONVERSION OPERATIONS
            "blur" => {
                let sigma: f32 = args.remove(0).parse().expect("Cannot parse SIGMA");

                img = blur(&mut img, sigma);
            }
            "brighten" => {
                let brightness: i32 = args.remove(0).parse().expect("Cannot parse BRIGHTNESS");

                img = brighten(&mut img, brightness);
            }
            "crop" => {
                let x: u32 = args.remove(0).parse().expect("Cannot parse X");
                let y: u32 = args.remove(0).parse().expect("Cannot parse Y");
                let width: u32 = args.remove(0).parse().expect("Cannot parse WIDTH");
                let height: u32 = args.remove(0).parse().expect("Cannot parse HEIGHT");

                img = crop(&mut img, x, y, width, height);
            }
            "rotate" => {
                let degrees: i32 = args.remove(0).parse().expect("Cannot parse DEGREES");

                img = rotate(&mut img, degrees);
            }
            "invert" => {
                img = invert(&mut img);
            }
            "grayscale" => {
                img = grayscale(&mut img);
            }
            "fractal" => {
                img = fractal();
            }
            "generate" => {
                img = generate();
            }

            // For everything else...
            _ => {
                print_usage_and_exit();
            }
        }

        img.save(outfile).expect(ERROR_OUTFILE);
    }
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");

    println!("INFILE OUTFILE <operation> <parameters>");
    println!("blur SIGMA");
    println!("brighten BRIGHTNESS");
    println!("crop X Y WIDTH HEIGHT");
    println!("fractal OUTFILE");
    println!("generate OUTFILE");
    println!("grayscale");
    println!("invert");
    println!("rotate DEGREES");

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

fn generate() -> DynamicImage {
    let width = 1024;
    let height = 768;
    let mut imgbuf = ImageBuffer::new(width, height);

    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = Rgb([255, 0, 0]);
    }

    image::load_from_memory(imgbuf.as_ref()).unwrap()
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal() -> DynamicImage {
    let width = 800;
    let height = 800;

    let mut imgbuf = ImageBuffer::new(width, height);

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
        *pixel = Rgb([red, green, blue]);
    }

    image::load_from_memory(imgbuf.as_ref()).unwrap()
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
