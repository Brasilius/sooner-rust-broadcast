use std::io::stdin;
use zmq::Socket;
use zmq::{self, Context};
fn main() {
    println!("welcome to the video streamer! ");
    loop {
        println!("Do  you wish to continue running the program? (y/n:): ");
        let mut user_input = String::new();
        stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        if user_input.trim().to_lowercase() == "y" {
            //place more shitty code here
            video_stream();
        } else if user_input.trim().to_lowercase() == "n" {
            // statement
            break;
        }
    }
}
fn video_stream() {
    let context = Context::new();
    let subscriber = context
        .socket(zmq::SUB)
        .unwrap();
    subscriber
        .connect("tcp://Localhost:5555")
        .unwrap();
    subscriber
        .set_subscribe("".as_bytes())
        .unwrap();
    loop {
        let msg = subscriber
            .recv_string(0)
            .unwrap()
            .expect("failed to recieve message");
        println!("Received {}", msg);
    }
}
