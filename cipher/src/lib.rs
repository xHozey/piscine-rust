#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut cipher_origin = String::new();
    for c in original.chars() {
        if c.is_alphabetic() {
            if c.is_lowercase() {
                cipher_origin.push((((26 - ((c as u8) - 96))) + 97) as char);
            } else {
                cipher_origin.push((((26 - ((c as u8) - 64))) + 65) as char);

            }
        } else {
            cipher_origin.push(c)
        }
    }
    if cipher_origin == ciphered {
        Ok(())
    } else {
        Err(CipherError{
            expected: cipher_origin
        })
    }
}