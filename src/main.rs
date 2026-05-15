use clap::{ArgGroup, Parser};
// use fsdr_blocks::type_converters::TypeConvertersBuilder;
use futuresdr::blocks::{Apply, FileSink, FileSource};
use futuresdr::futuredsp::num_traits::ToPrimitive;
use futuresdr::prelude::*;

use std::path::PathBuf;
use std::process;

pub mod iq_types;
use iq_types::IQTypes::{self, *};

#[derive(Parser, Debug)]
#[command(author, version, about, group(ArgGroup::new("mutual_exclusion_meta_override").args(["sigmf", "input_type"]).required(true)),long_about = None)]
struct Cli {
    /// Path to input iq file
    #[arg(value_name = "FILE")]
    iq_input: PathBuf,

    /// Path to output iq file
    #[arg(short('o'), long("output"), value_name = "FILE")]
    iq_output: PathBuf,

    /// Type IQ data should be converted to
    #[arg(short('t'), long, value_name = "TYPE")]
    output_type: IQTypes,

    /// override Type of input IQ data
    /// (normally automatically fetched from sigmf-meta)
    #[arg(short('i'), long, value_name = "TYPE")]
    input_type: Option<IQTypes>,

    /// Path to sigmf-meta file
    #[arg(short, long, value_name = "FILE")]
    sigmf: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    //ToDo: Output to stdout, if no output file given

    // print!("{:?}", args);

    let mut fg = Flowgraph::new();
    // let n = TypeConvertersBuilder::scale_convert::<i16, f32>().build();

    match (args.input_type.unwrap(), args.output_type) {
        // convert cf32 -> ci16
        (cf32le, ci16le) => {
            let src = FileSource::<Complex<f32>>::new(args.iq_input, false);
            let conv = Apply::new(|i: &Complex<f32>| {
                Complex::<i16>::new(
                    (i.re * i16::MAX as f32) as i16,
                    (i.im * i16::MAX as f32) as i16,
                )
            });
            let snk = FileSink::<Complex<i16>>::new(args.iq_output);
            connect!(fg, src > conv > snk);
        }

        // convert ci16 -> cf32
        (ci16le, cf32le) => {
            let src = FileSource::<Complex<i16>>::new(args.iq_input, false);
            let conv = Apply::<_, _, _>::new(move |i: &Complex<i16>| {
                Complex32::new(
                    i.re.to_f32().unwrap() / (i16::MAX as f32),
                    i.im.to_f32().unwrap() / (i16::MAX as f32),
                )
            });
            let snk = FileSink::<Complex<f32>>::new(args.iq_output);
            connect!(fg, src > conv > snk);
        }

        // convert cu8 -> cf32
        (cu8, cf32le) => {
            let src = FileSource::<Complex<u8>>::new(args.iq_input, false);
            let conv = Apply::<_, _, _>::new(move |i: &Complex<u8>| {
                Complex32::new(
                    i.re.to_f32().unwrap() / (u8::MAX as f32),
                    i.im.to_f32().unwrap() / (u8::MAX as f32),
                )
            });
            let snk = FileSink::<Complex<f32>>::new(args.iq_output);
            connect!(fg, src > conv > snk);
        }

        // convert cf32 -> cu8
        (cf32le, cu8) => {
            let src = FileSource::<Complex<f32>>::new(args.iq_input, false);
            let conv = Apply::new(|i: &Complex<f32>| {
                Complex::<u8>::new(
                    ((i.re + 1.0) * i8::MAX as f32) as u8,
                    ((i.im + 1.0) * i8::MAX as f32) as u8,
                )
            });
            let snk = FileSink::<Complex<u8>>::new(args.iq_output);
            connect!(fg, src > conv > snk);
        }

        // convert cf32 -> ci8
        (cf32le, ci8) => {
            let src = FileSource::<Complex<f32>>::new(args.iq_input, false);
            let conv = Apply::new(|i: &Complex<f32>| {
                Complex::<i8>::new((i.re * i8::MAX as f32) as i8, (i.im * i8::MAX as f32) as i8)
            });
            let snk = FileSink::<Complex<i8>>::new(args.iq_output);
            connect!(fg, src > conv > snk);
        }

        (a, b) => {
            eprintln!("Conversion from {a} to {b} is not supported (yet)!");
            process::exit(1);
        }
    };

    Runtime::new().run(fg)?;

    Ok(())
}
