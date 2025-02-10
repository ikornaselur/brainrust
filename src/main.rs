use brainrust;
use clap::Parser;
use std::path::PathBuf;

/// BrainRust - A BrainFuck interpreter, written in Rust (everyone loves to hear that)
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file
    input: PathBuf,
}

fn main() -> brainrust::Result<()> {
    let args = Args::parse();

    if !args.input.exists() {
        eprintln!("Input file does not exist: {}", args.input.display());
        std::process::exit(1);
    }

    if !args.input.is_file() {
        eprintln!("Input file is not a file: {}", args.input.display());
        std::process::exit(2);
    }

    if !args
        .input
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .eq("bf")
    {
        eprintln!(
            "Input file is not a BrainFuck file: {}",
            args.input.display()
        );
        std::process::exit(3);
    }

    let contents = std::fs::read_to_string(&args.input)?;

    let program = brainrust::parse_input(&contents)?;

    let mut vm = brainrust::VM::new();

    for instruction in program {
        vm.run_instruction(&instruction);
    }

    Ok(())
}
