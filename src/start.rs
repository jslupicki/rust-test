use test::print;

pub fn start() {
    println!("Witaj Świecie!");

    let letters = vec!["a", "b", "c", "d"];

    println!("letters: {:?}", letters);

    for l in &letters {
        println!("{}", l)
    }

    for i in 0..letters.len() {
        println!("letters[{}] = {}", i, letters[i]);
    }

    println!("Żegnaj Świecie");
    print("Hello");
    print("World");
    print("www");
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        println!("Unit test:it_works");
        let t = "witaj";
        println!(">{}<", t)
    }
}
