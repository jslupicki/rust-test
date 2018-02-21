use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;
use rand::random;
use thread_control;
use std::time::Duration;

pub fn start() {
    println!("**********************");
    println!("Threads module start()");
    println!("**********************");

    let threads_params = [('a', 50), ('b', 50), ('c', 50)];
    let mut threads = HashMap::new();

    let (sender_from_channel, receiver_from_channel): (Sender<String>, Receiver<String>) =
        mpsc::channel();
    for &(name, percentage) in threads_params.into_iter() {
        let (sender_to_channel, receiver_to_channel): (Sender<String>, Receiver<String>) =
            mpsc::channel();
        let (flag, control) = thread_control::make_pair();
        let sender = sender_from_channel.clone();
        let thread = thread::spawn(move || {
            info!("Thread '{}' started with threshold {}", name, percentage);
            while flag.alive() {
                match receiver_to_channel.recv() {
                    Ok(msg) => {
                        debug!("Thread '{}' receive '{}'", name, msg);
                        let response = match msg.as_str() {
                            "ping" => "pong",
                            _ => "ping",
                        };
                        info!("Thread '{}' responds with '{}'", name, response);
                        sender.send(response.to_string()).unwrap();
                        let r = ((random::<u8>() as f64) / 256.0 * 100.0) as i32;
                        debug!(
                            "Thread '{}' drew {} with threshold {}.",
                            name, r, percentage
                        );
                        if r > percentage {
                            info!(
                                "Thread '{}' drew {} which is more then {} so it end.",
                                name, r, percentage
                            );
                            break;
                        }
                    }
                    Err(e) => {
                        error!("{:?}", e);
                        break;
                    }
                }
            }
        });
        &threads.insert(name, (thread, control, sender_to_channel));
    }

    let &&(_, _, ref sender_to_channel) = &threads.get(&threads_params[0].0).unwrap();
    sender_to_channel.send("ping".to_string()).unwrap();

    loop {
        let mut all_done = true;
        for (name, &(_, ref control, ref sender_to_channel)) in &threads {
            debug!("Thread {} is alive {}", name, !control.is_done());
            all_done &= control.is_done();
            match receiver_from_channel.try_recv() {
                Ok(msg) => {
                    debug!("From thread '{}' receive '{}'", name, msg);
                    let mut sended = false;
                    for (t, &(_, ref control, ref sender_to_channel)) in &threads {
                        if !control.is_done() && t != name {
                            info!("To thread '{}' sending '{}'", t, msg);
                            sender_to_channel.send(msg.clone()).unwrap();
                            sended = true;
                            break;
                        }
                    }
                    if !sended & !control.is_done() {
                        info!("Thread '{}' is only alive so I send back '{}'", name, msg);
                        sender_to_channel.send(msg.clone()).unwrap();
                    }
                }
                _ => debug!("Receive NOTHING"),
            }
        }
        debug!("All is done = {}", all_done);
        if all_done {
            break;
        }
        thread::sleep(Duration::from_secs(1));
    }
}

pub fn start2() {
    println!("**********************");
    println!("Threads module start2()");
    println!("**********************");

    let threads_params = [("A", 80), ("B", 80)];
    let mut threads = vec![];

    let (main_sender, main_receiver): (Sender<(String, String)>, Receiver<(String, String)>) =
        mpsc::channel();
    for &(name, percentage) in threads_params.into_iter() {
        let (sender, receiver): (Sender<(String, String)>, Receiver<(String, String)>) =
            mpsc::channel();
        let main_sender = main_sender.clone();
        let thread = thread::Builder::new()
            .name(name.to_string())
            .spawn(move || loop {
                let (msg, sender_name) = receiver.recv().unwrap();
                println!("'{}' got '{}' from '{}'", name, msg, sender_name);
                if test(percentage) {
                    println!("'{}' ending", name);
                    break;
                }
                main_sender
                    .send((msg + "," + name, name.to_string()))
                    .unwrap();
            });
        threads.push((name, thread, sender));
    }

    threads[0]
        .2
        .send(("Start".to_string(), "main".to_string()))
        .unwrap();
    thread::sleep(Duration::from_secs(1));

    while let Ok((msg, sender_name)) = main_receiver.try_recv() {
        for &(ref name, _, ref sender) in &threads {
            if name.to_string() != sender_name {
                match sender.send((msg.clone(), sender_name.clone())) {
                    Ok(()) => break,
                    _ => continue,
                }
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
}

fn test(percentage: i32) -> bool {
    ((random::<u8>() as f64) / 256.0 * 100.0) as i32 > percentage
}
