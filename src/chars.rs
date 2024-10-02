use rand::{seq::SliceRandom, Rng};
use std::io::Write;

pub fn write_rand_char_to_buffer(
    num_chars: usize,
    mut rng: impl Rng,
    alphabet: String,
    line_length: Option<usize>,
    mut output: impl Write,
) {
    for i in 0..num_chars {
        let _ = output
            .write(&[*alphabet.as_bytes().choose(&mut rng).unwrap()])
            .unwrap();
        if let Some(length) = line_length {
            if i % length == length - 1 {
                let _ = output.write(b"\n").unwrap();
            }
        }
    }
    if let Some(length) = line_length {
        if num_chars % length != 0 {
            let _ = output.write(b"\n").unwrap();
        }
    }
    output.flush().expect("Unable to flush file");
}
