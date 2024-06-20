use std::os::unix::net::UnixStream;
use std::io::{BufRead, BufReader, Write};

fn main() -> std::io::Result<()> {
    let pipe_path = "pipe_name"; // replace with your pipe path

    // Open the named pipe for reading and writing
    let stream = UnixStream::connect(pipe_path)?;

    // Create a BufReader for the stream
    let mut reader = BufReader::new(&stream);

    // Write to the pipe
    writeln!(stream, "get_bytes1")?;
    stream.flush()?;

    // Read from the pipe
    let mut line = String::new();
    reader.read_line(&mut line)?;
    println!("Received get_bytes_1: {}", line.trim());

    // Write to the pipe
    writeln!(stream, "get_bytes2")?;
    stream.flush()?;

    // Read from the pipe
    line.clear();
    reader.read_line(&mut line)?;
    println!("Received get_bytes_2: {}", line.trim());

    Ok(())
}