#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum RhFactor {
	Positive,
	Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};
use std::str::FromStr;
use std::string::String;

impl FromStr for Antigen {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut antigen = String::new();
        for c in s.chars() {
            if c.is_alphabetic() {
                antigen.push(c)
            }
        }
        match antigen.as_str() {
            "A" => Ok(Self::A),
            "AB" => Ok(Self::AB),
            "B" => Ok(Self::B),
            "O" => Ok(Self::O),
            _ => Err("".to_string())
        }
    }
}

impl FromStr for RhFactor {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        if s.contains("-") {
            Ok(Self::Negative)
        } else {
            Ok(Self::Positive)
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, rec: &Self) -> std::cmp::Ordering {
       self.antigen.cmp(&rec.antigen) 
    }
}

impl FromStr for BloodType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let antigen = s.parse::<Antigen>().unwrap();
        let rh_factor = s.parse::<RhFactor>().unwrap();
        Ok(Self {
            antigen,
            rh_factor
        })
    }
}

use std::fmt::{self, Debug, Formatter};

impl Debug for BloodType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self.rh_factor {
            RhFactor::Positive => {
                match self.antigen {
                    Antigen::A => {
                        write!(f, "A+")
                    },
                    Antigen::B => {
                        write!(f, "B+")
                    },
                    Antigen::AB => {
                        write!(f, "AB+")
                    },
                    Antigen::O => {
                        write!(f, "O+")
                    }
                }
            },
            RhFactor::Negative => {
                match self.antigen {
                    Antigen::A => {
                        write!(f, "A-")
                    },
                    Antigen::B => {
                        write!(f, "B-")
                    },
                    Antigen::AB => {
                        write!(f, "AB-")
                    },
                    Antigen::O => {
                        write!(f, "O-")
                    }
                }
            }
        }
    }
}

impl BloodType {
        pub fn can_receive_from(&self, other: &Self) -> bool {
        match self.rh_factor {
            RhFactor::Negative => {
                if self.rh_factor != other.rh_factor {
                    return false
                }
            },
            RhFactor::Positive => {}
        };
        match self.antigen {
            Antigen::A => {other.antigen == Antigen::A || other.antigen == Antigen::O},
            Antigen::B => {other.antigen == Antigen::B || other.antigen == Antigen::O},
            Antigen::O => {other.antigen == Antigen::O},
            Antigen::AB => {true}
        }
    }
    fn all_types() -> Vec<Self> {
        let t = [Antigen::A, Antigen::B, Antigen::AB, Antigen::O];
        let mut res = vec![];
        for el in &t {
            res.push(BloodType{antigen: el.clone(), rh_factor: RhFactor::Negative});
            res.push(BloodType{antigen: el.clone(), rh_factor: RhFactor::Positive})
        }
        res
    }

	pub fn donors(&self) -> Vec<Self> {
        Self::all_types().into_iter().filter(|blood| self.can_receive_from(blood)).collect()
    }
	pub fn recipients(&self) -> Vec<BloodType> {
        Self::all_types().into_iter().filter(|blood| blood.can_receive_from(self)).collect()

    }
}