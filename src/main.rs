#[macro_use]
extern crate log;
extern crate log4rs;

mod start;
mod test;

use start::start;

fn main() {
    start();
}
