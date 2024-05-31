
use std::{fs::{create_dir, File, OpenOptions}, io::{self, Error, Write}, path::PathBuf};
use native_dialog::{FileDialog, MessageDialog, MessageType};
enum FileType {
    Pdf,
    Txt
}

fn convert(file_type: FileType) {

    match file_type {
        FileType::Pdf => {
            
            let path = open_file_dialog_box("pdf");

            let bytes = read_file(&path);
            let content = pdf_extract::extract_text_from_mem(&bytes.unwrap()).unwrap();
            
            let _ = write_file(&content);

        },
        FileType::Txt => {
            let _path = open_file_dialog_box("txt");
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

fn open_file_dialog_box(file_type: &str) -> Option<PathBuf> {

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

fn open_path_dialog_box() -> Option<PathBuf> {

    let path = FileDialog::new()
    .set_location("~/Documents/")
    .show_open_single_dir()
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

fn read_file(path: &Option<PathBuf>) -> io::Result<Vec<u8>>{
    match path {
        Some(p) => std::fs::read(p),
        None => Err(io::Error::new(io::ErrorKind::NotFound, "Path not provided")),
    }
}

fn write_file(content: &String) -> Result<(), Error> {

    let path = get_path().unwrap();

    if !path.exists() {
        
        match create_dir(&path) {
            Ok(_) => println!("Directory '{}' created successfully!", &path.display()), 
            Err(e) => println!("Error creating directory '{}': {}", &path.display(), e),
        };
        
        let result = create_file(&path);

        match result {
            Ok(mut file) => {
                if let Err(e) = file.write(content.as_bytes()) {
                    eprintln!("Erro ao escrever no arquivo pdf_to_txt.txt: {}", e);
                } else {
                    println!("Arquivo pdf_to_txt.txt sobrescrito com sucesso!");
                }
            }
            Err(e) => {
                eprintln!("Erro ao abrir o arquivo pdf_to_txt.txt: {}", e);
            }
        }

    }
    
    Ok(())
}

fn create_file(path: &PathBuf) -> Result<File, Error>{

    let file = File::create(path)?;

    Ok(file)
}

fn get_path() -> Option<PathBuf> {
    let choosed_path = open_path_dialog_box();
    let file_location = unwrap_pathbuf_to_string(&choosed_path).unwrap();

    let file_location = format!("{}/Pdf Converter", file_location);

    return Option::from(PathBuf::from(&file_location));
}

fn unwrap_pathbuf_to_string(path: &Option<PathBuf>) -> Result<String, &'static str> {
    match path {
        Some(p) => p
            .to_str()
            .map(|s| s.to_owned())
            .ok_or("Path is None"),
        None => Err("Path is None"),
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