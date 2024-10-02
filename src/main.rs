use binary::write_rand_bin_to_buffer;
use chars::write_rand_char_to_buffer;
use clap::{arg, Parser, Subcommand};
use rand::{rngs::StdRng, SeedableRng};
use std::{
    fs::File,
    io::{stdout, BufWriter, IsTerminal},
};

mod binary;
mod chars;

const WRITE_TO_BUFFER: usize = 1024 * 1024;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long, default_value = None)]
    output: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Binary {
        #[arg(short, long, default_value_t = 1024 * 1024 * 1024)]
        num_bytes: usize,
    },
    Char {
        #[arg(short, long, default_value_t = 1024 * 1024 * 1024)]
        num_chars: usize,
        #[arg(
            short,
            long,
            default_value = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        )]
        alphabet: String,

        #[arg(short, long, default_value = None)]
        line_length: Option<usize>,
    },
}

fn main() {
    let args = Args::parse();
    let rng = StdRng::from_entropy();

    match args.command {
        Commands::Binary { num_bytes } => {
            if !stdout().is_terminal() {
                let output = BufWriter::with_capacity(WRITE_TO_BUFFER, stdout());
                write_rand_bin_to_buffer(num_bytes, rng, output);
            } else if let Some(file_name) = args.output {
                let file = File::create(file_name).expect("Unable to create file");
                let output = BufWriter::with_capacity(WRITE_TO_BUFFER, file);
                write_rand_bin_to_buffer(num_bytes, rng, output);
            } else {
                let output = BufWriter::with_capacity(WRITE_TO_BUFFER, stdout());
                write_rand_bin_to_buffer(num_bytes, rng, output);
            };
        }
        Commands::Char {
            num_chars,
            alphabet,
            line_length,
        } => {
            if !stdout().is_terminal() {
                let output = BufWriter::with_capacity(WRITE_TO_BUFFER, stdout());
                write_rand_char_to_buffer(num_chars, rng, alphabet, line_length, output);
            } else if let Some(file_name) = args.output {
                let file = File::create(file_name).expect("Unable to create file");
                let output = BufWriter::with_capacity(WRITE_TO_BUFFER, file);
                write_rand_char_to_buffer(num_chars, rng, alphabet, line_length, output);
            } else {
                let output = BufWriter::with_capacity(WRITE_TO_BUFFER, stdout());
                write_rand_char_to_buffer(num_chars, rng, alphabet, line_length, output);
            };
        }
    }
}
