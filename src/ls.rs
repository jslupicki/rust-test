use std::io::Result;
use std::fs::{read_dir};
use std::path::PathBuf;

pub fn ls() -> Result<()> {
    println!("ls() here!");
    read_dir("/")?.map(|e| e.unwrap()).for_each(|e| {
        println!(
            "{} -> {}",
            e.file_name().into_string().unwrap(),
            e.file_type().unwrap().is_dir()
        )
    });

    Ok(())
}
pub fn show_tree(path: PathBuf) -> Result<()> {
    priv_show_tree(String::from(" "), path)
}

fn priv_show_tree(prefix: String, path: PathBuf) -> Result<()> {
    println!("{}+-{}", prefix, path.file_name().unwrap().to_string_lossy());
    let prefix = prefix + "| ";
    for e in read_dir(path)? {
        let e = e?;
        let file_name = e.file_name().into_string().unwrap();
        if e.file_type().unwrap().is_dir() {
            priv_show_tree(prefix.clone(), e.path())?;
        } else {
            println!("{}  {}", prefix, file_name)
        }
    }
    Ok(())
}
