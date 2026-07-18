use std::io::{self, BufRead};
fn main() {

//  v = read();
//  if(validate(v)) {
//    println!("ok");
//  } else {
//    println!("not ok");
//  }
//
//
//}
//
//fn read()
//{
    // declare v
    // Acquire a handle to the standard input
    let stdin = io::stdin();
    
    // Use .lock() for better performance and to gain access to BufRead methods
    for line in stdin.lock().lines() {
        match line {
            Ok(content) => {
                println!("Processed: {}", content);
		//v = cnvToVec(content);
            }
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                break;
            }
        }
    }
//    return v;
}
