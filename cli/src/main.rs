use std::{fs::{OpenOptions, self, File},  error::Error, path::Path, io::{BufReader, self}};
use std::io::prelude::*;
use directories::{ ProjectDirs};
use common::{self, Metric};

fn main() {
    let lines = read_record().unwrap().unwrap();

    for line in lines {
        println!("{}", line.unwrap());
    }
    
}

fn read_record() -> Result<Option<io::Lines<BufReader<File>>>, Box<dyn Error>> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "konsv", "batterymeter") {
        let path = proj_dirs.config_dir();
        let str_path = Box::new(path.to_str().unwrap());

        let ppath = Path::new(*str_path);
        if Path::exists(ppath) {

            let file = File::open(ppath).unwrap();
            return Ok(Some(io::BufReader::new(file).lines()));
        }  
        return Ok(None);
    }

    Ok(None)
}





