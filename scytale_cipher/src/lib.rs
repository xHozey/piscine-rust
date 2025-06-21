pub fn scytale_cipher(message: String, i: u32) -> String {
    let mut scytal = vec![vec![' '; (message.len() as f32/i as f32).ceil() as usize]; i as usize];
    for (idx, c) in message.char_indices() {
        let row = idx % i as usize;
        let col = idx / i as usize;
        scytal[row][col] = c; 
    }
    let mut res = String::new();
    for r in scytal {
        for c in r {
            res.push(c)
        }
    }
    res.trim().to_string()
}