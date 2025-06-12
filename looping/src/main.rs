use std::io;

fn main() {
    let mut retries = 0;
    let riddle =  "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let answer = "The letter e";

    loop {
        println!("{}", riddle);
        let mut buffer = String::new(); 
        io::stdin().read_line(&mut buffer);
        if buffer.trim() == answer {
            println!("{}", retries);
            break
        } else {
            retries += 1
        }
    }
}