use std::{io, path::{self, PathBuf}};
use native_dialog::{FileDialog, MessageDialog, MessageType};

enum FileType {
    Pdf,
    Txt
}

fn convert(file_type: FileType) {

    match file_type {
        FileType::Pdf => {
            let path = open_dialog_box("pdf");
            todo!();
        },
        FileType::Txt => {
            let path = open_dialog_box("txt");
            todo!();
        }
    }

}

fn open_message_dialog(path: &PathBuf) -> bool {
    return MessageDialog::new()
    .set_type(MessageType::Info)
    .set_title("Would you like to choose this file?")
    .set_text(&format!("{:#?}", path))
    .show_confirm()
    .unwrap();
}

fn open_dialog_box(file_type: &str) -> Option<PathBuf> {

    let path = FileDialog::new()
    .set_location("~/Downloads")
    .add_filter("File type", &[file_type])
    .show_open_single_file()
    .unwrap();

    let path = match path {
        Some(path) => path,
        None => panic!("Error"),
    };

    let yes = open_message_dialog(&path);


    if yes {
        return Some(path);
    }
    else {
        return None;
    }
        
}


fn main() {

    println!("Welcome!");

    loop {
        
        println!("Select the type of file you want to convert: ");
        println!("1 - PDF");
        println!("2 - TXT");
        println!("0 - EXIT");
        let mut input = String::new();
    
        io::stdin().read_line(&mut input).expect("Fail to read input");

        let input: i32 = match input.trim().parse::<i32>() {
            Ok(numb) => {numb},
            Err(_) => {
                println!("Invalid input!");
                continue;
            }
        };

        match input {
            1 => convert(FileType::Pdf),
            2 => convert(FileType::Txt),
            0 => break,
            _ => panic!("Input error")
        };

    }

}