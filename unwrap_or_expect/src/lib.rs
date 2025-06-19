pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => {
            match server {
                Ok(url) => url.to_string(),
                Err(_) => panic!()
            }
        },
        Security::Message => {
            match server {
                Ok(url) => url.to_string(),
                Err(_) => panic!("ERROR: program stops")
            }
        },
        Security::Warning => {
            match server {
                Ok(url) => url.to_string(),
                Err(_) => "WARNING: check the server".to_string()
            }
        },
        Security::NotFound => {
            match server {
                Ok(url) => url.to_string(),
                Err(err) => format!("Not found: {}", err)
            }
        },
        Security::UnexpectedUrl => {
            match server {
                Ok(url) => panic!("{}", url),
                Err(err) => err.to_string()
            }
        },
    }
}