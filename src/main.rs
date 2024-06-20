use std::io::{Read, Write};
use std::fs::OpenOptions;

fn main() -> std::io::Result<()> {
    println!("RUST: Begin main\n\n");
    let outgoing_pipe_path = "r2j";
    let incoming_pipe_path = "j2r";

    // Open the named pipe for reading and writing
    let mut outgoing_file = OpenOptions::new()
    .read(false)
    .write(true)
    .open(outgoing_pipe_path)?;

    // Open the named pipe for reading and writing
    let mut incoming_file = OpenOptions::new()
    .read(true)
    .write(false)
    .open(incoming_pipe_path)?;

    /*
     In Unix-like systems, opening a named pipe for reading will block until another process opens the pipe for writing, and vice versa
    */

    let mut count = 0;
    loop {
        println!("\nFunc call #{:?}", count);
        count += 1;
        println!("RUST: Waiting for command...");
        let mut buffer = [0; 8]; // the command is 8 bytes long
        // Print buffer
        // println!("RUST: Buffer: {:?}", buffer);
        incoming_file.read_exact(&mut buffer)?; //the read method in Rust is a blocking operation. If there's no data available to read, the process will block until data becomes available
        let num = f64::from_le_bytes(buffer); // convert the bytes to a float
        println!("RUST: Received command: {:?}", num);
        println!("RUST: Writing rust_function result to pipe");
                        let result = rust_func_1(num);
                        outgoing_file.write_all(&result[..])?; //creates a slice that includes all elements of result
    }

}

fn rust_func_1(num: f64) -> [u8; 8] {
    let val = num * 10.0;
    let bytes = val.to_le_bytes(); // convert the float to little-endian bytes
    bytes
}

// fn rust_func_2(num: f64) -> [u8; 8] {
//     let num = 1.8f64;
//     let bytes = num.to_le_bytes(); // convert the float to little-endian bytes
//     bytes
// }