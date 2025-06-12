use std::io;

fn main() {
    let mut retries = 1;
    let riddle =  "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let answer = "The letter e";

    loop {
        println!("{}", riddle);
        let mut buffer = String::new(); 
        let _ = io::stdin().read_line(&mut buffer);
        if buffer.trim() == answer {
            println!("Number of trials: {}", retries);
            break
        } else {
            retries += 1
        }
    }
}