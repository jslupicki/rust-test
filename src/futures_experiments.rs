use futures::future::{lazy, ok};
use futures::Future;
use rand::random;
use std::thread;
use std::time::Duration;
use futures_cpupool::CpuPool;

pub fn start() {
    println!("**********************************");
    println!("futures_experiments module start()");
    println!("**********************************");

    let mut futures = vec![];

    for i in 1..10 {
        futures.push(lazy(move || {
            let result = format!("Future {}", i).to_string();
            thread::sleep(Duration::from_millis(random::<u8>() as u64));
            println!("{}", result);
            ok::<u32,u32>(1)
        }));
    }

    let thread_pool = CpuPool::new_num_cpus();   
    let mut running_futures = vec![];

    for f in futures {
        running_futures.push(thread_pool.spawn(f));
    }

   for f in running_futures {
       f.wait().unwrap();
   }
}
