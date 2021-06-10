use std::io::{Write, LineWriter};

pub trait WriteReceiver: Write {
    fn write_with_separator(&mut self, contents: &str, print_separator: bool) {
        let num_bytes_written = match self.write(contents.as_bytes()) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to write to output: {}", e);
                0
            }
        };
        if num_bytes_written > 0 && print_separator {
            self.write_separator(" ");
        }
    }

    fn write_separator(&mut self, separator: &str) {
        match self.write(separator.as_bytes()) {
            Ok(_) => (),
            Err(e) => eprintln!("Failed to write to output: {}", e),
        }
    }
}

impl<W: Write> WriteReceiver for LineWriter<W> {}