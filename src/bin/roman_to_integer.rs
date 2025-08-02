use std::str::FromStr;

enum Numerals {
    I = 1,
    V = 5,
    X = 10,
    L = 50,
    C = 100,
    D = 500,
    M = 1000,
}

#[derive(Debug)]
pub struct ParseNumeralsError;

impl std::fmt::Display for ParseNumeralsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid numeral")
    }
}

impl std::error::Error for ParseNumeralsError {}

enum Subtractions {
    IV = 4,
    IX = 9,
    XL = 40,
    XC = 90,
    CD = 400,
    CM = 900,
}

#[derive(Debug)]
pub struct ParseSubtractionError;

impl std::fmt::Display for ParseSubtractionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid subtraction numeral")
    }
}

impl std::error::Error for ParseSubtractionError {}

impl ToString for Numerals {
    fn to_string(&self) -> String {
        match self {
            Numerals::I => String::from("I"),
            Numerals::V => String::from("V"),
            Numerals::X => String::from("X"),
            Numerals::L => String::from("L"),
            Numerals::C => String::from("C"),
            Numerals::D => String::from("D"),
            Numerals::M => String::from("M"),
        }
    }
}
impl ToString for Subtractions {
    fn to_string(&self) -> String {
        match self {
            Subtractions::IV => String::from("IV"),
            Subtractions::IX => String::from("IX"),
            Subtractions::XL => String::from("XL"),
            Subtractions::XC => String::from("XC"),
            Subtractions::CD => String::from("CD"),
            Subtractions::CM => String::from("CM"),
        }
    }
}

impl FromStr for Subtractions {
    type Err = ParseSubtractionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IV" => Ok(Subtractions::IV),
            "IX" => Ok(Subtractions::IX),
            "XL" => Ok(Subtractions::XL),
            "XC" => Ok(Subtractions::XC),
            "CD" => Ok(Subtractions::CD),
            "CM" => Ok(Subtractions::CM),
            _ => Err(ParseSubtractionError),
        }
    }
}
impl FromStr for Numerals {
    type Err = ParseNumeralsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "I" => Ok(Numerals::I),
            "V" => Ok(Numerals::V),
            "X" => Ok(Numerals::X),
            "L" => Ok(Numerals::L),
            "C" => Ok(Numerals::C),
            "D" => Ok(Numerals::D),
            "M" => Ok(Numerals::M),
            _ => Err(ParseNumeralsError),
        }
    }
}

pub fn roman_to_int(s: &str) -> i32 {
    let mut sum: i32 = 0;
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if i + 1 < chars.len() {
            let pair = format!("{}{}", chars[i], chars[i + 1]);
            if let Ok(sub) = Subtractions::from_str(&pair) {
                sum += sub as u16 as i32;
                i += 2;
                continue;
            }
        }

        // Otherwise, single numeral
        let single = chars[i].to_string();
        if let Ok(num) = Numerals::from_str(&single) {
            sum += num as u16 as i32;
        } else {
            // Invalid character, bail out with 0
            return 0;
        }
        i += 1;
    }

    sum
}

fn main() {
    assert_eq!(roman_to_int("III"), 3);
    assert_eq!(roman_to_int("IV"), 4);
    assert_eq!(roman_to_int("IX"), 9);
    assert_eq!(roman_to_int("LVIII"), 58); // L=50, V=5, III=3
    assert_eq!(roman_to_int("MCMXCIV"), 1994);
}
