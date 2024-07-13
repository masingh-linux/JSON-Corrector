use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let (input_path, output_path) = match args.as_slice() {
        [_, input, output] => (input, output),
        _ => {
            eprintln!("Usage: json_corrector <input_file> <output_file>");
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid arguments"));
        }
    };

    let input_file = File::open(input_path)?;
    let output_file = File::create(output_path)?;

    let mut reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);

    let mut buffer = vec![0; 4096]; // Buffer to read chunks
    let mut remainder = Vec::new();  // To handle partial reads

    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        let mut chunk = &buffer[..bytes_read];
        if let Some((last_idx, _)) = chunk.iter().enumerate().rfind(|&(_, &c)| c == b';') {
            remainder.extend_from_slice(&chunk[last_idx + 1..]);
            chunk = &chunk[..last_idx + 1];
        } else {
            remainder.extend_from_slice(chunk);
            continue;
        }

        let corrected_chunk = String::from_utf8(remainder.clone()).unwrap().replace(';', ":");
        writer.write_all(corrected_chunk.as_bytes())?;
        remainder.clear();
    }

    writer.flush()?;
    Ok(())
}

