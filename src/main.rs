use std::env;
use futures::executor::block_on;
use plan9sys::syscall::nsec;

pub struct Timeval {
    sec: i32,
    usec: i32,
}

pub fn nsec_to_timeval(n: isize) -> Timeval {
    let nsec = n + 999; // round up to microsecond

    let tv: Timeval = Timeval {
        usec: (nsec % 1_000_000_000 / 1_000) as i32,
        sec: (nsec / 1_000_000_000) as i32,
    };
    tv
}

pub fn get_time_of_day() -> Timeval {
    let nsec = nsec();
    nsec_to_timeval(nsec)
}

async fn hello_world() {
    println!("hello, world!");
}

pub fn main() {
    let a = Box::new(4711);
    println!("Hello World {}", a);

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let tv = get_time_of_day();
    println!("{:?}", tv.usec);
    println!("{:?}", tv.sec);

    let future = hello_world(); // Nothing is printed
    block_on(future); // `future` is run and "hello, world!" is printed
    
    println!("done")
}
