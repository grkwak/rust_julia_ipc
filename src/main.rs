use std::io::{Read, Write};
use std::fs::OpenOptions;

fn main() -> std::io::Result<()> {
    println!("RUST: Begin main");
    let outgoing_pipe_path = "r2j"; // replace with your pipe path
    let incoming_pipe_path = "j2r"; // replace with your pipe path

    // Open the named pipe for reading and writing
    let mut outgoing_file = OpenOptions::new()
    .read(false)
    .write(true)
    .open(outgoing_pipe_path)?;

    // println!("huh");

    // Open the named pipe for reading and writing
    let mut incoming_file = OpenOptions::new()
    .read(true)
    .write(false)
    .open(incoming_pipe_path)?;

    /*
     In Unix-like systems, opening a named pipe for reading will block until another process opens the pipe for writing, and vice versa
    */

    loop {
        println!("RUST: Waiting for command...");
        let mut buffer = [0]; // assuming the command is at most 12 bytes long
        incoming_file.read(&mut buffer)?; //the read method in Rust is a blocking operation. If there's no data available to read, the process will block until data becomes available

        // let line = String::from_utf8_lossy(&buffer);
        println!("RUST: Received command: {:?}", buffer);

        match buffer {
            [49] => { // 49 is the ASCII code for '1'
                println!("RUST: Writing get_bytes_1 to pipe");
                let result = get_bytes_1();
                outgoing_file.write_all(&result)?;
            }
            [50] => { // 50 is the ASCII code for '2'
                println!("RUST: Writing get_bytes_2 to pipe");
                let result = get_bytes_2();
                outgoing_file.write_all(&result)?;
            }
            _ => {
                // handle unknown function
            }
        }
    }

}

fn get_bytes_1() -> Vec<u8> {
    vec![2, 3, 4, 5, 6]
}

fn get_bytes_2() -> Vec<u8> {
    vec![7, 8, 9, 10, 11]
}