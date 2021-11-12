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

    // Calculate the maximum sample value
    let max = (2_u64.pow(config.depth as u32) - 1) as f64;
    if config.verbose {
        println!("Max sample value: {}", max);
    }

    // Perform calculation for each sample point
    for i in 0..config.samples {
        let x = i as f64 / config.samples as f64;
        let sin = (x * 2_f64 * PI).sin();
        let sample = (((sin + 1_f64) / 2_f64) * max).round() as u32;
        if config.hex {
            println!("{:#010x}", sample)
        } else {
            println!("{}", sample);
        }
    }
}
