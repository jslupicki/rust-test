extern crate futures;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate rand;
extern crate thread_control;
extern crate tokio;
extern crate futures_cpupool;

mod start;
mod test;
mod ls;
mod threads;
mod futures_experiments;

use start::start;

fn main() {
    start();
    threads::start();
    threads::start2();
    futures_experiments::start();
}
