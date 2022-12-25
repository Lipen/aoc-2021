use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::{error, fmt};

use anyhow::{anyhow, Error, Result};
use once_cell_regex::regex;
use regex::Regex;

use crate::ValidationError::*;
use crate::ValidationResult::*;

#[derive(Debug, Default, PartialEq)]
struct Passport {
    /// Birth Year
    byr: Option<String>,
    /// Issue Year
    iyr: Option<String>,
    /// Expiration Year
    eyr: Option<String>,
    /// Height
    hgt: Option<String>,
    /// Hair Color
    hcl: Option<String>,
    /// Eye Color
    ecl: Option<String>,
    /// Passport ID
    pid: Option<String>,
    /// Country ID (optional for validation)
    cid: Option<String>,
}

#[derive(Debug, PartialEq)]
enum ValidationResult {
    Valid,
    Invalid(ValidationError),
}

#[derive(Debug, PartialEq)]
enum ValidationError {
    FieldIsNone(&'static str),
    FieldDoesNotMatchRegex(&'static str),
    FieldIsNotInRange(&'static str),
    HeightCmIsNotInRange,
    HeightInIsNotInRange,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for ValidationError {}

impl Passport {
    fn is_valid_simple(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
        // Note: self.cid is optional for validation
    }

    fn validate_complex(&self) -> ValidationResult {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        if let Some(byr) = &self.byr {
            if let Some(caps) = regex!(r"^(\d{4})$").captures(&byr) {
                let n = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
                if !(1920..=2002).contains(&n) {
                    return Invalid(FieldIsNotInRange("byr"));
                }
            } else {
                return Invalid(FieldDoesNotMatchRegex("byr"));
            }
        } else {
            return Invalid(FieldIsNone("byr"));
        }

        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        if let Some(iyr) = &self.iyr {
            if let Some(caps) = regex!(r"^(\d{4})$").captures(&iyr) {
                let n = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
                if !(2010..=2020).contains(&n) {
                    return Invalid(FieldIsNotInRange("iyr"));
                }
            } else {
                return Invalid(FieldDoesNotMatchRegex("iyr"));
            }
        } else {
            return Invalid(FieldIsNone("iyr"));
        }

        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        if let Some(eyr) = &self.eyr {
            if let Some(caps) = regex!(r"^(\d{4})$").captures(&eyr) {
                let n = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
                if !(2020..=2030).contains(&n) {
                    return Invalid(FieldIsNotInRange("eyr"));
                }
            } else {
                return Invalid(FieldDoesNotMatchRegex("eyr"));
            }
        } else {
            return Invalid(FieldIsNone("eyr"));
        }

        // hgt (Height) - a number followed by either cm or in:
        //  If cm, the number must be at least 150 and at most 193.
        //  If in, the number must be at least 59 and at most 76
        if let Some(hgt) = &self.hgt {
            if let Some(caps) = regex!(r"^(\d+)(cm|in)$").captures(&hgt) {
                let n = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let unit = caps.get(2).unwrap().as_str();
                match unit {
                    "cm" => {
                        if !(150..=193).contains(&n) {
                            return Invalid(HeightCmIsNotInRange);
                        }
                    }
                    "in" => {
                        if !(59..=76).contains(&n) {
                            return Invalid(HeightInIsNotInRange);
                        }
                    }
                    _ => {}
                }
            } else {
                return Invalid(FieldDoesNotMatchRegex("hgt"));
            }
        } else {
            return Invalid(FieldIsNone("hgt"));
        }

        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        if let Some(hcl) = &self.hcl {
            if !regex!(r"^#[0-9a-f]{6}$").is_match(&hcl) {
                return Invalid(FieldDoesNotMatchRegex("hcl"));
            }
        } else {
            return Invalid(FieldIsNone("hcl"));
        }

        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        if let Some(ecl) = &self.ecl {
            if !regex!(r"^(amb|blu|brn|gry|grn|hzl|oth)$").is_match(&ecl) {
                return Invalid(FieldDoesNotMatchRegex("ecl"));
            }
        } else {
            return Invalid(FieldIsNone("ecl"));
        }

        // pid (Passport ID) - a nine-digit number, including leading zeroes
        if let Some(pid) = &self.pid {
            if !regex!(r"^(\d{9})$").is_match(&pid) {
                return Invalid(FieldDoesNotMatchRegex("pid"));
            }
        } else {
            return Invalid(FieldIsNone("pid"));
        }

        // cid (Country ID) - ignored, missing or not.

        // If passed all validation checks, then the Passport is valid
        Valid
    }

    fn is_valid_complex(&self) -> bool {
        self.validate_complex() == Valid
    }
}

fn main() -> Result<()> {
    let path = "data/input.txt";
    let reader = BufReader::new(File::open(path)?);

    let mut passports = Vec::new();
    let mut temp: Option<Passport> = None;

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            if let Some(passport) = temp {
                passports.push(passport);
                temp = None
            }
        } else {
            let passport = temp.get_or_insert_with(|| Default::default());
            for item in line.split_whitespace() {
                let (key, value) = item
                    .split_once(':')
                    .ok_or(anyhow!("Bad key-value pair `{}`", item))?;
                match key {
                    "byr" => passport.byr = Some(value.to_string()),
                    "iyr" => passport.iyr = Some(value.to_string()),
                    "eyr" => passport.eyr = Some(value.to_string()),
                    "hgt" => passport.hgt = Some(value.to_string()),
                    "hcl" => passport.hcl = Some(value.to_string()),
                    "ecl" => passport.ecl = Some(value.to_string()),
                    "pid" => passport.pid = Some(value.to_string()),
                    "cid" => passport.cid = Some(value.to_string()),
                    _ => return Err(anyhow!("Bad key `{}`", key)),
                }
            }
        }
    }

    // Push last passport
    if let Some(passport) = temp {
        passports.push(passport);
        temp = None
    }

    // First passport (invalid)
    assert_eq!(
        Some(&Passport {
            byr: Some("1943".to_string()),
            iyr: Some("2013".to_string()),
            eyr: Some("2030".to_string()),
            hgt: Some("151cm".to_string()),
            hcl: Some("#ceb3a1".to_string()),
            ecl: Some("grn".to_string()),
            pid: None,
            cid: None,
        }),
        passports.first()
    );
    assert!(!passports.first().unwrap().is_valid_simple());
    // Last passport (valid)
    assert_eq!(
        Some(&Passport {
            byr: Some("2001".to_string()),
            iyr: Some("2014".to_string()),
            eyr: Some("2025".to_string()),
            hgt: Some("161cm".to_string()),
            hcl: Some("#4784a2".to_string()),
            ecl: Some("amb".to_string()),
            pid: Some("955262336".to_string()),
            cid: None,
        }),
        passports.last()
    );
    assert!(passports.last().unwrap().is_valid_simple());

    // Simple validation
    let valid_simple = passports.iter().filter(|&x| x.is_valid_simple()).count();
    println!(
        "Number of valid passports (simple validation): {} of {}",
        valid_simple,
        passports.len()
    );

    // Complex validation
    let valid_complex = passports.iter().filter(|&x| x.is_valid_complex()).count();
    println!(
        "Number of valid passports (complex validation): {} of {}",
        valid_complex,
        passports.len()
    );

    Ok(())
}
