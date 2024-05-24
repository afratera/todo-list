use crate::list::List;
use crate::utils::UserChoices;

mod list;
mod utils;


const DIRECTORY_PATH: &str = "/home/afratera/.todo_list";               // TODO: Universell Einsetzbar machen (Remove afratera stuff)




fn main() {

    let mut lists: Vec<List> = Vec::new();

    let mut config: Box<List> = Box::new(List::new(String::new(), String::new()));

    utils::read_config(&DIRECTORY_PATH.to_string(), &mut config);

    utils::open_lists(&DIRECTORY_PATH.to_string(), &config, &mut lists);

    //List::print(&config);

    //utils::print_all_lists(&lists);

    utils::help();

    loop {
        match utils::get_user_choice() {
            UserChoices::Help => utils::help(),
            UserChoices::List => panic!("Not Implemented"),
            UserChoices::ListCat => utils::print_list_names(&lists),
            UserChoices::ListAll => utils::print_all_lists(&lists),
            UserChoices::MarkDone => panic!("Not Implemented"),
            UserChoices::Remove => panic!("Not Implemented"),
            UserChoices::RemoveCat => panic!("Not Implemented"),
            UserChoices::ChangeColor => panic!("Not Implemented"),
            UserChoices::Quit => break,
        }
    }

}
