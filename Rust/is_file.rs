use std::fs;
use std::io;
use std::path::Path;

fn is_file_or_dir(path: &Path) -> Result<String,io::Error> {
    // if path.exists() {  
    //     let metadata = fs::metadata(path)?;  
    //     let file_type = metadata.file_type();  
  
    //     if file_type.is_file() {  
    //         Ok(format!("{} is a file", path.display()))  
    //     } else if file_type.is_dir() {  
    //         Ok(format!("{} is a directory", path.display()))  
    //     } else {  
    //         Err(io::Error::new(io::ErrorKind::Other, format!("{} is not a regular file or directory", path.display())))  
    //     }  
    // } else {  
    //     Err(io::Error::new(io::ErrorKind::NotFound, format!("{} does not exist", path.display())))  
    // } 

    let metadata = fs::metadata(path)?; // 获取元数据  
    let file_type = metadata.file_type();  
  
    match file_type.is_file() {  
        true => Ok(format!("{} is a file", path.display())),  
        false => {  
            if file_type.is_dir() {  
                Ok(format!("{} is a directory", path.display()))  
            } else {  
                Err(io::Error::new(io::ErrorKind::Other, format!("{} is not a file or directory", path.display())))  
            }  
        }  
    }  
}
fn main() {
    let path=Path::new("test_file");
    match is_file_or_dir(path) {
        Ok(message)=>println!("{}",message),
        Err(e)=>println!("Error: {}",e),
    }
}