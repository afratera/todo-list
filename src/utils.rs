use std::fs;
use std::path::Path;
use std::io;
use std::io::Write;

use crate::List;

pub enum UserChoices {
    Help,
    List,
    ListCat,
    ListAll,
    MarkDone,
    Remove,
    RemoveCat,
    ChangeColor,
    Quit,
}

pub fn print_all_lists(lists: &Vec<List>) {
    for list in lists {
        List::print(list);
    }
}

pub fn print_list_names(lists: &Vec<List>){
    panic!("Not yet implemented");
}

pub fn read_config(directory_path: &String, config: &mut Box<List>) {
    let path_config: String = directory_path.to_owned() + "/config";
    let config_file_contents: String;
    
    if !Path::new(&directory_path).exists()
    {
        println!("Todo-List Data Directory doesn't exits. Creating now!");
        let directory_creation_result = fs::create_dir(&directory_path);

        let _ = match directory_creation_result {
            Ok(file) => file,
            Err(error) => panic!("Problem creating Directory {:?}", error),
        };
    }

    if !Path::new(&path_config).exists()
    {
        println!("Lists File doesn't exist. Creating now!");
        let config_creation_result = fs::File::create(&path_config);
        
        let _ = match config_creation_result {
            Ok(file) => file,
            Err(error) => panic!("Problem creating File {:?}", error),
        };
    }

    config_file_contents = fs::read_to_string(&path_config).expect("Failed to read from Config File!");

    *config = Box::new(List::new(path_config, config_file_contents));

}

pub fn open_lists(directory_path: &String, config: &Box<List>, lists: &mut Vec<List>){
    
    for i in 0..config.list_element.len(){
        let file_contents: String;
        let file_path = directory_path.to_string() + "/" + &config.list_element[i].to_string();
    
        if !Path::new(&file_path).exists()
        {
            println!("File doesn't exist. Creating now!");
            let file_creation_result = fs::File::create(&file_path);
            
            let _ = match file_creation_result {
                Ok(file) => file,
                Err(error) => panic!("Problem creating File {:?}", error),
            };
        }
    
        file_contents = fs::read_to_string(&file_path).expect("Failed to read from File!");
    
        lists.push(List::new(file_path, file_contents));
    }

}

pub fn get_user_choice() -> UserChoices{
    let mut input = String::new();
    read_input(&mut input);
    
    let inputstr: &str = &input;


    match inputstr {
        "q" => UserChoices::Quit,
        "h" => UserChoices::Help,
        "lc" => UserChoices::ListCat,
        "la" => UserChoices::ListAll,
        _ => UserChoices::Help,
    }
}

pub fn help(){
    println!(" h   - Show this list");
    println!(" l   - Show specific Categorie");
    println!(" lc  - List Categories");
    println!(" la  - List all ToDo's");
    println!(" d   - Mark ToDo as done");
    println!(" rm  - Remove ToDo");
    println!(" rmc - Remove Categorie");
    println!(" c   - Change Color of Categorie");
    println!(" q   - Quit");
}

fn read_input(input: &mut String) {

    print!("> ");
    let _ = io::stdout().flush();

    io::stdin().read_line(input).expect("Failed to read Input");

    input.pop();
}