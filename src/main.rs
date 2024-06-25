use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    println!("RUST: Begin main\n\n");

    // let mut str = "init"; //TODO: only have 1 incoming pipe and in that incoming pipe, say which Rust function to call AND the necessary params
    handle_init_or_close("init")?;
    handle_step()?; // won't actually know how many times Rust will be called
    handle_init_or_close("close")?;

    println!("\n\nRUST: End main");

    Ok(())
}

// func_name: { init, step, close }
fn handle_init_or_close(func_name: &str) -> std::io::Result<()> {
    println!("\n\nRUST: Begin wait_for_julia_call");

    let outgoing_pipe_path = String::from("r2j_") + func_name;
    let incoming_pipe_path = String::from("j2r_") + func_name;

    // Open the named pipe for reading
    let incoming_file = OpenOptions::new().read(true).open(incoming_pipe_path);

    // Check if opening the incoming file was successful
    let mut incoming_file = match incoming_file {
        Ok(file) => file,
        Err(e) => return Err(e), // Return the error if opening the file failed
    };

    let mut buffer = [0; 8]; // The command is 8 bytes long
    incoming_file.read_exact(&mut buffer)?; // Use `?` to propagate errors
    println!("RUST: Received number: {}", f64::from_le_bytes(buffer));

    let mut result = [0; 8]; // The result is 8 bytes long
    if func_name == "init" {
        result = rust_init(buffer);
    } else if func_name == "close" {
        result = rust_close(buffer);
    } else {
        panic!("Invalid function name");
    }

    // Open the named pipe for writing
    let outgoing_file = OpenOptions::new().write(true).open(outgoing_pipe_path);

    // Check if opening the outgoing file was successful
    let mut outgoing_file = match outgoing_file {
        Ok(file) => file,
        Err(e) => return Err(e), // Return the error if opening the file failed
    };

    outgoing_file.write_all(&result[..])?;

    Ok(()) // Return the number

    //Rust will close pipes for me automatically
}

fn handle_step() -> io::Result<()> {
    let incoming_pipe_path = "j2r_step";
    let outgoing_pipe_path = "r2j_step";

    let mut incoming_pipe = OpenOptions::new().read(true).open(incoming_pipe_path)?;
    let mut outgoing_pipe = OpenOptions::new().write(true).open(outgoing_pipe_path)?;

    let mut buffer = [0; 8]; // Assuming we're working with single-byte commands for simplicity

    loop {
        println!("\n\nRUST: Begin handle_step");

        match incoming_pipe.read_exact(&mut buffer) {
            Ok(_) => {
                let num = f64::from_le_bytes(buffer);
                println!("RUST: Received number: {}", num);
                if num.is_nan() {
                    break;
                }
                let result = rust_step(buffer);
                outgoing_pipe.write_all(&result[..])?;
            }
            Err(e) => return Err(e),
        }
    }

    // Pipes will be automatically closed when `incoming_pipe` and `outgoing_pipe` go out of scope
    Ok(())
}

fn rust_init(bytes: [u8; 8]) -> [u8; 8] {
    let num = f64::from_le_bytes(bytes); // Convert the bytes to a float
    let val = num * 10.0;
    sleep(Duration::from_secs(5));
    let bytes = val.to_le_bytes(); // convert the float to little-endian bytes

    bytes
}

fn rust_step(bytes: [u8; 8]) -> [u8; 8] {
    let num = f64::from_le_bytes(bytes); // Convert the bytes to a float
    let val = num * 100.0;
    sleep(Duration::from_secs(5));
    let bytes = val.to_le_bytes(); // convert the float to little-endian bytes

    bytes
}

fn rust_close(bytes: [u8; 8]) -> [u8; 8] {
    let num = f64::from_le_bytes(bytes); // Convert the bytes to a float
    let val = num * 1000.0;
    sleep(Duration::from_secs(5));
    let bytes = val.to_le_bytes(); // convert the float to little-endian bytes

    bytes
}
