use adventofcode2020::regex;

#[derive(Default)]
struct Passport<'a> {
    byr: Option<&'a str>,
    iyr: Option<&'a str>,
    eyr: Option<&'a str>,
    hgt: Option<&'a str>,
    hcl: Option<&'a str>,
    ecl: Option<&'a str>,
    pid: Option<&'a str>,
    cid: Option<&'a str>,
}

fn parse_passwords(input: &str) -> Vec<Passport> {
    let mut passports = Vec::new();
    passports.push(Passport::default());
    for line in input.lines() {
        if line.is_empty() {
            passports.push(Passport::default());
            continue;
        }

        for field in line.split(' ') {
            let passport = passports.last_mut().unwrap();
            let mut fields = field.split(':');
            let key = fields.next().unwrap();
            let value = fields.next().unwrap();
            match key {
                "byr" => passport.byr = Some(value),
                "iyr" => passport.iyr = Some(value),
                "eyr" => passport.eyr = Some(value),
                "hgt" => passport.hgt = Some(value),
                "hcl" => passport.hcl = Some(value),
                "ecl" => passport.ecl = Some(value),
                "pid" => passport.pid = Some(value),
                "cid" => passport.cid = Some(value),
                entry => panic!("unknown entry: {}", entry),
            }
        }
    }
    passports
}

fn is_valid_date_range(str: &str, min: usize, max: usize) -> bool {
    if let Some(captures) = regex!("^(\\d{4})$").captures(str) {
        if let Ok(year) = captures[0].parse::<usize>() {
            if year >= min && year <= max {
                return true;
            }
        };
    }
    false
}

impl<'a> Passport<'a> {
    fn required_fields_present(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn required_fields_valid(&self) -> bool {
        if !self.required_fields_present() {
            return false;
        }

        if !self
            .byr
            .map(|s| is_valid_date_range(s, 1920, 2002))
            .unwrap_or(false)
        {
            return false;
        }
        if !self
            .iyr
            .map(|s| is_valid_date_range(s, 2010, 2020))
            .unwrap_or(false)
        {
            return false;
        }
        if !self
            .eyr
            .map(|s| is_valid_date_range(s, 2020, 2030))
            .unwrap_or(false)
        {
            return false;
        }

        if let Some(captures) = regex!("^(\\d+)(cm|in)$").captures(self.hgt.unwrap()) {
            if let (Ok(value), unit) = (captures[1].parse::<usize>(), &captures[2]) {
                if !match unit {
                    "cm" => value >= 150 && value <= 193,
                    "in" => value >= 59 && value <= 76,
                    _ => panic!("unknown unit: {}", unit),
                } {
                    return false;
                }
            } else {
                return false;
            };
        } else {
            return false;
        }

        if !regex!("^#[0-9a-f]{6}$").is_match(self.hcl.unwrap()) {
            return false;
        }

        if !regex!("^amb|blu|brn|gry|grn|hzl|oth$").is_match(self.ecl.unwrap()) {
            return false;
        }

        if !regex!("^\\d{9}$").is_match(self.pid.unwrap()) {
            return false;
        }

        true
    }
}

fn main() {
    let input = std::fs::read_to_string("inputs/day4/input").unwrap();
    let passports = parse_passwords(&input);

    println!(
        "solution 1: {}",
        passports
            .iter()
            .filter(|p| p.required_fields_present())
            .count()
    );
    println!(
        "solution 2: {}",
        passports
            .iter()
            .filter(|p| p.required_fields_valid())
            .count()
    );
}
