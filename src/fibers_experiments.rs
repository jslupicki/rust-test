use fibers::{Executor, Spawn, ThreadPoolExecutor};
use futures::{Future, finished};

pub fn start() {
    println!("*********************************");
    println!("fibers_experiments module start()");
    println!("*********************************");

    let input_number = 10;

    let mut executor = ThreadPoolExecutor::new().unwrap();
    let future = fibonacci(input_number, &executor.handle());
    let monitor = executor.spawn_monitor(future);
    let answer = executor.run_fiber(monitor).unwrap().unwrap();
    println!("fibonacci({}) = {}", input_number, answer);
}

fn fibonacci<H: Spawn>(n: usize, handle: &H) -> Box<Future<Item = usize, Error = ()> + Send> {
    if n < 2 {
        Box::new(finished(n))
    } else {
        let f0 = handle.spawn_monitor(fibonacci(n - 1, handle));
        let f1 = handle.spawn_monitor(fibonacci(n - 2, handle));
        Box::new(f0.join(f1).map(|(a0, a1)| a0 + a1).map_err(|_| ()))
    }
}