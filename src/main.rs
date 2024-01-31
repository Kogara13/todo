use std::env;
use std::fs::{self, File};
use std::path::Path;
use std::fs::OpenOptions;
use std::process;
use std::io::Write;
use std::error::Error;

struct Todo {
    path: String,
    list: Vec<String>
}

//We've build machines that can understand exactly what I want my code to do but somehow it still can't tell that the struct
//isn't used cause I'm not done writing it
impl Todo {
    fn build() -> Todo {
        //Path temporarily hard-coded
        let path= "./src/list.txt".to_string();
        //Will still check that file exists, and create if not    
        match Path::new(&path).exists() {
            true => println!("File found"),
            false => {
                match File::create(&path) {
                    Ok(..) => println!("File created"),
                    Err(err) => {
                        panic!("Failed to create file: {}", err);
                    },
                }
            }
        };
        //Grab current contents of file
        let list: Vec<String> = match fs::read_to_string(&path){
            Ok(contents) => contents
                .split("\n")
                .map(|line| line.to_string())
                .collect(),
            Err(error) => {
                eprintln!("failed to read todo-list file: {}", error);
                Vec::new() //Return empty vector
            }
        };
        Todo { path, list }
    }

    //Append a new line to the file directly. Then the program will rust list to regrab everything
    //from the file, including th new stuff
    fn add(&self, args: &[String]) -> Result<(), Box<dyn Error>> {
        //Check that what the user inputted makes sense
        if args.is_empty() {
            eprintln!("argument missing");
            process::exit(1); 
        }

        //Turn vector of strings into a single string
        let list_item = args.join(" ");

        //Create permissions for the file 
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&self.path)
            .unwrap();
        
        //Convert String to bytes and append to file 
        writeln!(file, "{}", list_item)?;

        Ok(())
        



        
    }


}


fn main() {
    
    //Grab user commands as a vector of strings
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
}








