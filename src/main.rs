use clap::{CommandFactory, Parser, Subcommand, ValueEnum, error::ErrorKind};
use color_print::{ceprintln, cprintln};
use formats::{DataFormat, PixelFormat};
use gvrtex::{TextureDecoder, TextureEncoder};
use std::{ops::Not, path::PathBuf, process::ExitCode};

mod formats;

#[derive(Parser)]
#[command(name = "gvrtex")]
#[command(version, about = "Encodes and decodes images in the GVR texture format", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encodes the given image file into an appropriate GVR texture file.
    Encode {
        /// Input image file to operate on. Can be any standardized image format (.png, .jpg, etc).
        input: PathBuf,

        /// Path to where to save the encoded GVR file to.
        output: PathBuf,

        /// The format the image data should be encoded in.
        #[arg(short, long, value_enum, default_value_t = DataFormat::Dxt1)]
        data_format: DataFormat,

        /// The format to use for the color data of the color palette, when using either `index4`
        /// or `index8` data format. This option is ignored in other cases.
        #[arg(short, long, value_enum, default_value_t = PixelFormat::Rgb5a3)]
        pixel_format: PixelFormat,

        /// Encode this texture with mipmaps. Only supported on `dxt1`, `rgb565` and `rgb5a3`.
        #[arg(short, long)]
        mipmaps: bool,

        /// The magic string to use in the header of the encoded GVR file.
        #[arg(short = 'i', long, value_enum, default_value_t = HeaderId::Gcix)]
        header: HeaderId,

        /// The global index to use in the header of the encoded GVR file.
        #[arg(short, long, default_value_t = 0)]
        global_index: u32,
    },

    /// Decodes the given GVR texture file into an image file.
    Decode {
        /// Input GVR texture file to operate on.
        input: PathBuf,

        /// Path to where to save the output image file to.
        /// Make sure to include the file extension for the file, so that it can save the image in
        /// the given format. Only image formats that support transparency will work.
        output: PathBuf,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum HeaderId {
    /// Sets the GVR texture's header string to "GCIX".
    Gcix,
    /// Sets the GVR texture's header string to "GBIX".
    Gbix,
}

impl std::fmt::Display for HeaderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gcix => write!(f, "GCIX"),
            Self::Gbix => write!(f, "GBIX"),
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Encode {
            input,
            output,
            data_format,
            pixel_format,
            mipmaps,
            header,
            global_index,
        } => {
            // mipmap validation
            if *mipmaps
                && matches!(
                    data_format,
                    DataFormat::Dxt1 | DataFormat::Rgb565 | DataFormat::Rgb5a3
                )
                .not()
            {
                let possible_value = data_format.to_possible_value().unwrap();
                let name = possible_value.get_name();
                let mut cmd = Cli::command();
                cmd.error(
                    ErrorKind::ArgumentConflict,
                    format!("Can't use mipmaps on the `{name}` data format."),
                )
                .exit()
            }

            // encode the texture
            let encoder_result: Result<TextureEncoder, gvrtex::error::TextureEncodeError>;
            match data_format {
                DataFormat::Index4 | DataFormat::Index8 => {
                    if let HeaderId::Gcix = header {
                        encoder_result = TextureEncoder::new_gcix_palettized(
                            (*pixel_format).into(),
                            (*data_format).into(),
                        );
                    } else {
                        encoder_result = TextureEncoder::new_gbix_palettized(
                            (*pixel_format).into(),
                            (*data_format).into(),
                        );
                    }
                }

                _ => {
                    if let HeaderId::Gcix = header {
                        encoder_result = TextureEncoder::new_gcix((*data_format).into());
                    } else {
                        encoder_result = TextureEncoder::new_gbix((*data_format).into());
                    }
                }
            }

            if let Err(e) = encoder_result {
                ceprintln!("<r!>error:</> while initializing:");
                eprintln!("  {e}");
                return ExitCode::FAILURE;
            }

            let mut encoder = encoder_result.unwrap();
            if *mipmaps {
                // guaranteed to not fail
                encoder = encoder.with_mipmaps().unwrap();
            }
            if *global_index > 0 {
                encoder = encoder.with_global_index(*global_index);
            }

            let encoded = match encoder.encode(input.to_str().expect("Couldn't parse input path."))
            {
                Ok(val) => val,
                Err(e) => {
                    ceprintln!("<r!>error:</> while encoding texture:");
                    eprintln!("  {e}");
                    return ExitCode::FAILURE;
                }
            };

            if let Err(e) = std::fs::write(
                output.to_str().expect("Couldn't parse output path."),
                &encoded,
            ) {
                ceprintln!("<r!>error:</> while writing output file:");
                eprintln!("  {e}");
                return ExitCode::FAILURE;
            }

            cprintln!("<g!>success:</> saved encoded texture to:");
            println!("  {}", output.display());

            println!();

            cprintln!("<c!>info:</>");
            println!("  Header: {header}");
            println!("  Data format: {data_format}");
            if let DataFormat::Index4 | DataFormat::Index8 = data_format {
                println!("  Pixel format: {pixel_format}");
            }
            println!("  Mipmaps: {mipmaps}");
            println!("  Global index: {global_index}");
        }

        Commands::Decode { input, output } => {
            let mut decoder =
                TextureDecoder::new(input.to_str().expect("Couldn't parse input path."));
            if let Err(e) = decoder {
                ceprintln!("<r!>error:</> while opening input file:");
                eprintln!("  {e}");
                return ExitCode::FAILURE;
            }

            if let Err(e) = decoder.as_mut().unwrap().decode() {
                ceprintln!("<r!>error:</> while decoding input file:");
                eprintln!("  {e}");
                return ExitCode::FAILURE;
            }

            if let Err(e) = decoder
                .unwrap()
                .save(output.to_str().expect("Couldn't parse the output path."))
            {
                ceprintln!("<r!>error:</> while saving output image:");
                eprintln!("  {e}");
                return ExitCode::FAILURE;
            }

            cprintln!("<g!>success:</> saved decoded image to:");
            println!("  {}", output.display());
        }
    }

    ExitCode::SUCCESS
}
