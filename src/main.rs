use zmq::Context;

fn main() {
    println!("Hello, world!");

    let context = Context::new();
    let subscriber = context.socket(zmq::SUB).unwrap();
    subscriber.connect("tcp://localhost:5555").unwrap();
    subscriber.set_subscribe(b"topic").unwrap();

    loop {
        let message = subscriber.recv_string(0).unwrap().unwrap();
        println!("Received: {}", message);
    }
    
}

