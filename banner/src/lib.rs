use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(long_hand: &str, desc: &str) -> Self {
        Self {
            short_hand: format!("-{}", long_hand[0..1].to_string()),
            long_hand: format!("-{}", long_hand),
            desc: desc.to_string()
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.long_hand, func);
        self.flags.insert(flag.short_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        match  self.flags.get(input).unwrap()(argv[0], argv[1]) {
            Ok(res) => Ok(res),
            Err(err) => Err(err.to_string())
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    match a.parse::<f64>() {
        Ok(c) => {
            match b.parse::<f64>() {
                Ok(d) => {
                    Ok((c/d).to_string())
                },
                Err(err) => Err(err)
            }
        },
        Err(err) => Err(err)
    }

}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
     match a.parse::<f64>() {
        Ok(c) => {
            match b.parse::<f64>() {
                Ok(d) => {
                    Ok((c%d).to_string())
                },
                Err(err) => Err(err)
            }
        },
        Err(err) => Err(err)
    }
}
