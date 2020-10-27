use std::fs;
use std::io::{ErrorKind};

fn main() {
    let result = copy();

    match result {
        Ok(file_size_copied) => file_size_copied,
        Err(error) => {
            if error.kind() == ErrorKind::NotFound {
                println!("File not found !!!!")
            }
            if error.kind() == ErrorKind::PermissionDenied {
                println!("PremissionDenied check user permissions on the files which need copying");
            }
            if error.kind() == ErrorKind::AlreadyExists {
                println!("Error File AlreadyExists " );
            }
        } 
    }
    
}

fn copy() -> std::io::Result<()> {
    let file_size_copied = match fs::copy("foo.txt", "bar.txt") { 
        Err(e) => {println!("{:?}",e);return Err(e)},
        Ok(f) => f,
    };
    Ok(())
}
