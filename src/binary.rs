use crate::WRITE_TO_BUFFER;
use rand::Rng;
use std::io::Write;

pub fn write_rand_bin_to_buffer(mut num_bytes: usize, mut rng: impl Rng, mut output: impl Write) {
    let mut buf = vec![0u8; WRITE_TO_BUFFER];
    while num_bytes > WRITE_TO_BUFFER {
        rng.fill_bytes(&mut buf);
        output.write_all(&buf).expect("Unable to write to file");
        num_bytes -= WRITE_TO_BUFFER;
    }
    if num_bytes > 0 {
        let mut buf = vec![0u8; num_bytes];
        rng.fill_bytes(&mut buf);
        output.write_all(&buf).expect("Unable to write to file");
    }
    output.flush().expect("Unable to flush file");
}
