use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;
use rand::random;
use thread_control;
use std::time::Duration;

pub fn start() {
    let threads_params = [('a', 50), ('b', 50), ('c', 50)];
    let mut threads = HashMap::new();

    println!("Threads module started!");

    for &(name, percentage) in threads_params.into_iter() {
        let (sender_to_channel, receiver_to_channel): (Sender<String>, Receiver<String>) =
            mpsc::channel();
        let (sender_from_channel, receiver_from_channel): (
            Sender<String>,
            Receiver<String>,
        ) = mpsc::channel();
        let (flag, control) = thread_control::make_pair();
        let thread = thread::spawn(move || {
            info!("Thread '{}' started with threshold {}", name, percentage);
            while flag.alive() {
                match receiver_to_channel.recv() {
                    Ok(msg) => {
                        info!("Thread '{}' receive '{}'", name, msg);
                        let response = match msg.as_str() {
                            "ping" => "pong",
                            _ => "ping",
                        };
                        info!("Thread '{}' responds with '{}'", name, response);
                        sender_from_channel.send(response.to_string()).unwrap();
                        let r = ((random::<u8>() as f64) / 256.0 * 100.0) as i32;
                        info!(
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
        &threads.insert(
            name,
            (thread, control, sender_to_channel, receiver_from_channel),
        );
    }

    let &&(_, _, ref sender_to_channel, _) = &threads.get(&threads_params[0].0).unwrap();
    sender_to_channel.send("ping".to_string()).unwrap();

    loop {
        let mut all_done = true;
        for (name, &(_ , ref control, ref sender_to_channel , ref receiver_from_channel)) in &threads {
            info!("Thread {} is alive {}", name, !control.is_done());
            all_done &= control.is_done();
            match receiver_from_channel.try_recv() {
                Ok(msg) => {
                    info!("From thread '{}' receive '{}'", name, msg);
                    let mut sended = false;
                    for (t, &(_ , ref control , ref sender_to_channel, _ )) in &threads {
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
                },
                _ => info!("From thread '{}' receive NOTHING", name),
            }
        }
        info!("All is done = {}", all_done);
        if all_done {
            break;
        }
        thread::sleep(Duration::from_secs(1));
    }
}
