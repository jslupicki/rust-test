use ls::*;
use test::print;
use log4rs;
use std::path::PathBuf;

pub fn start() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    info!("Witaj Świecie!");

    let letters = vec!["a", "b", "c", "d"];

    info!("letters: {:?}", letters);

    for l in &letters {
        info!("{}", l)
    }

    for i in 0..letters.len() {
        info!("letters[{}] = {}", i, letters[i]);
    }
/*
    info!("LS:");
    match ls() {
        Ok(_) => println!("ls() finished OK"),
        Err(e) => println!("ls() finished with ERROR: {}", e),
    }
*/
    let start_dir = PathBuf::from("resources");

    info!("show_tree:");
    match show_tree(&start_dir) {
        Ok(_) => println!("show_tree() finished OK"),
        Err(e) => println!("show_tree() finished with ERROR: {}", e),
    }

    info!("DirTree:");
    let dir_tree = build_dir_tree(&start_dir);
    println!("{}", dir_tree);

    info!("Żegnaj Świecie");
    print("Hello");
    print("World");
    print("www");
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        info!("Unit test:it_works");
        let t = "witaj";
        info!(">{}<", t)
    }
}
