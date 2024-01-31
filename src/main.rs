use std::env;
use std::fs::{self, File};
use std::path::Path;
use std::fs::OpenOptions;




fn main() {
    
    //Check that file exists and create one if it doesn't
    let file_path = "./src/list.txt";
    match Path::new(&file_path).exists() {
        true => println!("File found"),
        false => {
            match File::create(&file_path) {
                Ok(..) => println!("File created"),
                Err(err) => {
                    panic!("Failed to create file: {}", err);
                },
            }
        }
    };
    

    let todo_list: Vec<String> = match fs::read_to_string(&file_path){
        Ok(contents) => contents
            .split("\n")
            .map(|line| line.to_string())
            .collect(),
        Err(error) => {
            eprintln!("failed to read todo-list file: {}", error);
            Vec::new() //Return empty vector
        }
    };

    let args: Vec<String> = env::args().collect();
    
    println!("{:?}", args);
    for i in &todo_list{
        println!("{i}");
    }
}


//Take command line arguments and append them to file
fn add(&file_path: String, &args: &[String]) {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true) //May not need this
        .open(&file_path);
}






