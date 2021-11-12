use std::f64::consts::PI;
use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "samplegen")]
/// Generate samples from periodic functions
struct Config {
    /// Bit depth
    #[structopt(short, long)]
    depth: u8,

    /// Number of samples to generate
    #[structopt(short, long)]
    samples: u32,

    /// Number of columns to use in the output
    #[structopt(short, long, default_value = "1")]
    cols: u8,

    /// Enables hexadecimal output
    #[structopt(short, long)]
    hex: bool,

    /// Verbose mode
    #[structopt(short, long)]
    verbose: bool,
}

fn main() {
    let config = Config::from_args();
    if config.verbose {
        println!("{:#?}", config);
    }

    // Check the bounds of the bit depth
    if config.depth == 0 {
        println!("Specified bit depth cannot be zero");
        exit(1);
    } else if config.depth > 32 {
        println!(
            "Specified bit depth of {} exceeds supported maximum of 32",
            config.depth
        );
        exit(2);
    }

    // Check the bounds of the number of columns
    if config.cols == 0 {
        println!("Column count cannot be zero");
        exit(3);
    }

    // Calculate the maximum sample value
    let max = (2_u64.pow(config.depth as u32) - 1) as f64;
    if config.verbose {
        println!("Max sample value: {}", max);
    }

    let mut samples: Vec<u32> = Vec::new();

    // Perform calculation for each sample point
    for i in 0..config.samples {
        // Get the current position as a fraction of the full period
        let x = i as f64 / config.samples as f64;

        // Calculate the sine
        let sin = (x * 2_f64 * PI).sin();

        // Rescale the calculated value
        let sample = (((sin + 1_f64) / 2_f64) * max).round() as u32;

        samples.push(sample);
    }

    // Pick the formatting function
    let print = if config.hex {
        match config.depth {
            0..=4 => |s: u32| print!("{:#03X}, ", s),
            5..=8 => |s: u32| print!("{:#04X}, ", s),
            9..=12 => |s: u32| print!("{:#05X}, ", s),
            13..=16 => |s: u32| print!("{:#06X}, ", s),
            17..=20 => |s: u32| print!("{:#07X}, ", s),
            21..=24 => |s: u32| print!("{:#08X}, ", s),
            25..=28 => |s: u32| print!("{:#09X}, ", s),
            _ => |s: u32| print!("{:#010X}, ", s),
        }
    } else {
        |s: u32| print!("{}, ", s)
    };

    // Generate output
    for (i, s) in samples.iter().enumerate() {
        print(*s);

        // Move to next line if we're at the end of the row
        if (i as u32 + 1) % (config.cols as u32) == 0 {
            println!();
        }
    }
    println!();
}
