// use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use mio::{Events, Interest, Poll, Token};
// use mio::unix::SourceFd;
// use std::os::unix::io::AsRawFd;

fn main() {
    let pipe = OpenOptions::new().read(true).write(true).open("Users/grace.kwak/Desktop/rust_julia_ipc/").unwrap();
    let reader = BufReader::new(&pipe);

    let mut poll = Poll::new().unwrap();

    // let mut poll = Poll::new();//.unwrap();
    // let mut events = Events::with_capacity(1);

    // // Register our `pipe` with the `Poll` instance.
    // poll.registry().register(&pipe, Token(0), Interest::READABLE).unwrap(); //maybe &mut pipe

    // loop { // This is an infinite loop that keeps the program running until it's manually stopped
    //     poll.poll(&mut events, None).unwrap(); // This line blocks the program and waits for events. When an event occurs, it will return and we can handle the event

    //     for event in events.iter() { // there should only be 1 event
    //         match event.token() {
    //             Token(0) => { // If the token is 0, it means the event came from the source registered with token 0 (in this case, the pipe).
    //                 for line in reader.lines() { //there should only be 1 line
    //                     let line = line.unwrap();
    //                     match line.as_str() {
    //                         "get_bytes_1" => {
    //                             let result = get_bytes_1();
    //                             let slice: &[u8] = &result;
    //                             pipe.write_all(slice).unwrap();
    //                             // writeln!(pipe, "{}", result).unwrap();
    //                         }
    //                         "get_bytes_2" => {
    //                             let result = get_bytes_2();
    //                             let slice: &[u8] = &result;
    //                             pipe.write_all(slice).unwrap();
    //                             // writeln!(pipe, "{}", result).unwrap();
    //                         }
    //                         _ => {
    //                             // writeln!(pipe, "Unknown function: {}", line).unwrap();
    //                         }
    //                     }
    //                 }
    //             }
    //             _ => unreachable!(),
    //         }
    //     }
    // }
}

/*
If you want to return a byte slice that isn't static (i.e., it doesn't live for the entire duration of the program), you'll need to return a Vec<u8> instead, or use lifetimes to ensure the data the returned slice references lives long enough. Here's an example with Vec<u8>:
*/
fn get_bytes_1() -> Vec<u8> {
    vec![1, 2, 3]
}

fn get_bytes_2() -> Vec<u8> {
    vec![4, 5, 6]
}