use clap::{App, Arg};

pub struct Args {
    pub infile: String,
    pub outfile: String,
    pub blur: bool,
    pub blur_amount: f32,
    pub fractal: bool,
}

impl Args {
    pub fn parse() -> Self {
        let matches = App::new("Mirage Image Processor")
            .version("0.1")
            .about("Provides image processing capabilities")
            .arg(
                Arg::with_name("INPUT_FILE")
                    .required(true)
                    .help("Input file to process")
                    .index(1),
            )
            .arg(
                Arg::with_name("OUTPUT_FILE")
                    .required(true)
                    .help("Output file for results")
                    .index(2),
            )
            .arg(
                Arg::with_name("blur")
                    .short('b')
                    .long("blur")
                    .value_name("AMOUNT")
                    .help("blurs image by x amount")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("fractal")
                    .short('f')
                    .long("fractal")
                    .help("blurs image by x amount")
                    .takes_value(false),
            )
            .get_matches();

        let infile = matches
            .value_of("INPUT_FILE")
            .unwrap_or_default()
            .to_string();
        let outfile = matches
            .value_of("OUTPUT_FILE")
            .unwrap_or_default()
            .to_string();
        let blur = matches.is_present("blur");
        let blur_amount: f32 = matches
            .value_of("blur")
            .unwrap_or_default()
            .parse()
            .unwrap();
        let fractal = matches.is_present("fractal");

        Self {
            infile,
            outfile,
            blur,
            blur_amount,
            fractal,
        }
    }
}
