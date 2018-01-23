use test::print;
use log4rs;

pub fn start() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    info!("Witaj Świecie!");

    let letters = vec!["a", "b", "c", "d"];

    debug!("letters: {:?}", letters);

    for l in &letters {
        debug!("{}", l)
    }

    for i in 0..letters.len() {
        debug!("letters[{}] = {}", i, letters[i]);
    }

    info!("Żegnaj Świecie");
    print("Hello");
    print("World");
    print("www");
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        debug!("Unit test:it_works");
        let t = "witaj";
        debug!(">{}<", t)
    }
}
