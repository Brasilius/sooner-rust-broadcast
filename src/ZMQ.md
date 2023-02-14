You
---

write a basic zmq subscriber program in rust

ChatGPT
-------

### A Basic ZMQ Subscriber Program in Rust

This program will create a subscriber socket and connect to the given port address. Messages are printed out when available.

```rust
use zmq;

fn main() {
  let context = zmq::Context::new();
  let mut subscriber = context.socket(zmq::SUB).unwrap();

  subscriber.connect("tcp://127.0.0.1:5555").unwrap();
  subscriber.subscribe("").unwrap(); // subscibe to all topics

  loop {
    match subscriber.recv_string(0) {
      Ok(message) => {
        println!("Received Message: {}", message);
      }
      Err(e) => break, // Break out of the loop if there is an error
    }
  }
}
```