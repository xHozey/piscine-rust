pub fn check_ms(message: &str) -> Result<&str, &str> {
    if message.is_empty() || message == "stupid" {
      return Err("ERROR: illegal")
    }
  Ok(message)
}