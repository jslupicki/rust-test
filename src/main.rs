#[macro_use]
extern crate log;
extern crate log4rs;

mod start;
mod test;
mod ls;

use start::start;

fn main() {
    start();
}
