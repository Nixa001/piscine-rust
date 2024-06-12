#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    // Make RhFactor public
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)] // Derive Ord here as well
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

use std::fmt;
use std::str::FromStr;

impl FromStr for Antigen {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            _ => Err(()),
        }
    }
}

impl FromStr for RhFactor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(()),
        }
    }
}

impl FromStr for BloodType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(());
        }

        let antigen_part = &s[0..s.len() - 1];
        let rh_part = &s[s.len() - 1..];

        let antigen = antigen_part.parse::<Antigen>()?;
        let rh_factor = rh_part.parse::<RhFactor>()?;

        Ok(BloodType { antigen, rh_factor })
    }
}

impl fmt::Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen_str = match self.antigen {
            Antigen::A => "A",
            Antigen::AB => "AB",
            Antigen::B => "B",
            Antigen::O => "O",
        };

        let rh_str = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };

        write!(f, "{}{}", antigen_str, rh_str)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        match (self.antigen, &self.rh_factor) {
            (Antigen::O, RhFactor::Negative) => true,
            (Antigen::O, RhFactor::Positive) => other.rh_factor == RhFactor::Positive,
            (Antigen::A, RhFactor::Negative) => {
                other.antigen == Antigen::A || other.antigen == Antigen::O
            }
            (Antigen::A, RhFactor::Positive) => {
                (other.antigen == Antigen::A || other.antigen == Antigen::O)
                    && other.rh_factor == RhFactor::Positive
            }
            (Antigen::B, RhFactor::Negative) => {
                other.antigen == Antigen::B || other.antigen == Antigen::O
            }
            (Antigen::B, RhFactor::Positive) => {
                (other.antigen == Antigen::B || other.antigen == Antigen::O)
                    && other.rh_factor == RhFactor::Positive
            }
            (Antigen::AB, RhFactor::Negative) => {
                other.antigen == Antigen::A
                    || other.antigen == Antigen::B
                    || other.antigen == Antigen::AB
                    || other.antigen == Antigen::O
            }
            (Antigen::AB, RhFactor::Positive) => other.rh_factor == RhFactor::Positive,
        }
    }

    pub fn donors(&self) -> Vec<Self> {
        let all_blood_types = vec![
            "O-".parse().unwrap(),
            "O+".parse().unwrap(),
            "A-".parse().unwrap(),
            "A+".parse().unwrap(),
            "B-".parse().unwrap(),
            "B+".parse().unwrap(),
            "AB-".parse().unwrap(),
            "AB+".parse().unwrap(),
        ];

        all_blood_types
            .into_iter()
            .filter(|bt| self.can_receive_from(bt))
            .collect()
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let all_blood_types = vec![
            "O-".parse().unwrap(),
            "O+".parse().unwrap(),
            "A-".parse().unwrap(),
            "A+".parse().unwrap(),
            "B-".parse().unwrap(),
            "B+".parse().unwrap(),
            "AB-".parse().unwrap(),
            "AB+".parse().unwrap(),
        ];

        all_blood_types
            .into_iter()
            .filter(|bt| bt.can_receive_from(self))
            .collect()
    }
}

fn main() {
    let blood_type: BloodType = "O+".parse().unwrap();
    println!("{:?}", blood_type);

    println!("recipients of O+ {:?}", blood_type.recipients());
    println!("donors of O+ {:?}", blood_type.donors());
}
