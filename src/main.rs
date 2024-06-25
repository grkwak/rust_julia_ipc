use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    println!("RUST: Begin main\n\n");

    // let mut str = "init"; //TODO: only have 1 incoming pipe and in that incoming pipe, say which Rust function to call AND the necessary params
    wait_for_julia_call("init")?;
    wait_for_julia_call("step")?; // won't actually know how many times Rust will be called
    wait_for_julia_call("close")?;

    println!("\n\nRUST: End main");

    Ok(())
}

// func_name: { init, step, close }
fn wait_for_julia_call(func_name: &str) -> std::io::Result<f64> {
    println!("\n\nRUST: Begin wait_for_julia_call");

    let outgoing_pipe_path = String::from("r2j_") + func_name;
    let incoming_pipe_path = String::from("j2r_") + func_name;

    // Open the named pipe for reading
    let incoming_file = OpenOptions::new()
        .read(true)
        .write(false)
        .open(incoming_pipe_path);

    // Check if opening the incoming file was successful
    let mut incoming_file = match incoming_file {
        Ok(file) => file,
        Err(e) => return Err(e), // Return the error if opening the file failed
    };

    let mut buffer = [0; 8]; // The command is 8 bytes long
    incoming_file.read_exact(&mut buffer)?; // Use `?` to propagate errors
    let num = f64::from_le_bytes(buffer); // Convert the bytes to a float
    println!("RUST: Received number: {}", num);

    let mut result = [0; 8]; // The result is 8 bytes long
    if func_name == "init" {
        result = rust_init(num);
    } else if func_name == "step" {
        result = rust_step(num);
    } else if func_name == "close" {
        result = rust_close(num);
    } else {
        panic!("Invalid function name");
    }

    // Open the named pipe for writing
    let outgoing_file = OpenOptions::new()
        .read(false)
        .write(true)
        .open(outgoing_pipe_path);

    // Check if opening the outgoing file was successful
    let mut outgoing_file = match outgoing_file {
        Ok(file) => file,
        Err(e) => return Err(e), // Return the error if opening the file failed
    };

    outgoing_file.write_all(&result[..])?;

    Ok(num) // Return the number

    //Rust will close pipes for me automatically
}

fn rust_init(num: f64) -> [u8; 8] {
    let val = num * 10.0;
    let bytes = val.to_le_bytes(); // convert the float to little-endian bytes
    sleep(Duration::from_secs(5));
    bytes
}

fn rust_step(num: f64) -> [u8; 8] {
    let val = num * 100.0;
    let bytes = val.to_le_bytes(); // convert the float to little-endian bytes
    sleep(Duration::from_secs(5));
    bytes
}

fn rust_close(num: f64) -> [u8; 8] {
    let val = num * 1000.0;
    let bytes = val.to_le_bytes(); // convert the float to little-endian bytes
    sleep(Duration::from_secs(5));
    bytes
}
