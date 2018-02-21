#[macro_use]
extern crate log;
extern crate log4rs;
extern crate rand;
extern crate thread_control;

mod start;
mod test;
mod ls;
mod threads;

use start::start;

fn main() {
    start();
    threads::start();
    threads::start2();
}
