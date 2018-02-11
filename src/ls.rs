use std::io::Result;
use std::fs::read_dir;
use std::path::PathBuf;

#[derive(Debug)]
pub struct DirTree {
    path: PathBuf,
    children: Vec<DirTree>,
    files: Vec<PathBuf>,
}

#[allow(dead_code)]
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

pub fn show_tree(path: &PathBuf) -> Result<()> {
    priv_show_tree(String::from(" "), path)
}

fn priv_show_tree(prefix: String, path: &PathBuf) -> Result<()> {
    println!(
        "{}+-{}",
        prefix,
        path.file_name().unwrap().to_string_lossy()
    );
    let prefix = prefix + "| ";
    for e in read_dir(path)? {
        let e = e?;
        let file_name = e.file_name().into_string().unwrap();
        if e.file_type().unwrap().is_dir() {
            priv_show_tree(prefix.clone(), &e.path())?;
        } else {
            println!("{}  {}", prefix, file_name)
        }
    }
    Ok(())
}

pub fn build_dir_tree(path: &PathBuf) -> DirTree {
    let mut dir = DirTree {
        path: path.clone(),
        children: vec![],
        files: vec![],
    };
    for e in read_dir(path).unwrap() {
        let e = e.unwrap();
        if e.file_type().unwrap().is_dir() {
            dir.children.push(build_dir_tree(&e.path()));
        } else {
            dir.files.push(e.path());
        }
    }
    dir
}

fn display(dir_tree: &DirTree, f: &mut fmt::Formatter, prefix: &String) -> fmt::Result {
    writeln!(
        f,
        "{}+-{}",
        prefix,
        dir_tree.path.file_name().unwrap().to_string_lossy()
    )?;
    let prefix = prefix.clone() + "| ";
    for dir in &dir_tree.children {
        display(dir, f, &prefix)?;
    }
    for file in &dir_tree.files {
        let file_name = file.file_name().unwrap().to_string_lossy();
        writeln!(f, "{}  {}", prefix, file_name)?;
    }
    Ok(())
}

use std::fmt;
impl fmt::Display for DirTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        display(self, f, &String::from(""))
    }
}
