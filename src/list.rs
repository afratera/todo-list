use fancy::*;

pub struct List {
    pub name: String,
    pub file_path: String,
    pub list_element: Vec<String>,
}

impl List {
    pub fn new(fd: String, content: String) -> List {
        List{
            name: {
                let fd_split: Vec<&str> = fd.split('/').collect();
                let mut result: String = fd_split.last().expect("Name Retrieval Failed!").to_string();
                if result == "config" {
                    result = "ToDo".to_string();
                }
                result
                },
            file_path: fd,
            list_element: Self::parse_contents(content),
        }
    }

    fn parse_contents(content: String) -> Vec<String> {
        let mut vector: Vec<String> = Vec::new();

        let split_content: Vec<&str> = content.split('\n').collect();
        
        for line in split_content{
            if line != "" {
                vector.push(line.to_string());
            }
        }
        
        return vector;
    }

    pub fn print(&self) {

        printcoln!("[bold|underline|red]{}:", self.name);
        for i in 0..self.list_element.len() {
            printcoln!(" {}: {}", i+1, self.list_element[i].to_string());
        }

    
    }

}