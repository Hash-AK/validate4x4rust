use std::io::{self, BufRead};
fn main() {
    // Acquire a handle to the standard input
    let stdin = io::stdin();
    
    // Use .lock() for better performance and to gain access to BufRead methods
    for line in stdin.lock().lines() {
        match line {
            Ok(content) => {
                println!("Processed: {}", content);
            }
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                break;
            }
        }
    }
}
