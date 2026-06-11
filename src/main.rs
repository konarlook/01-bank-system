use parslib::convert;
use parslib::error::ConvertError;
use parslib::format::Format;
use std::path::Path;
use std::{env, error, fs};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Использование");
        eprintln!("     converter input.csv outpub.txt");
        eprintln!("     converter input.txt output.bin");
        eprintln!("     converter input.bin output.csv");
        return Ok(());
    }
    let inp: Format = Path::new(&args[1])
        .extension()
        .and_then(|e| e.to_str())
        .ok_or(ConvertError::UnknownFormat)?
        .parse()?;

    let out: Format = Path::new(&args[2])
        .extension()
        .and_then(|e| e.to_str())
        .ok_or(ConvertError::UnknownFormat)?
        .parse()?;

    let mut reader = fs::File::open(args[1].clone())?;
    let mut writer = fs::File::create(args[2].clone())?;

    convert(inp, out, &mut reader, &mut writer)?;

    Ok(())
}
