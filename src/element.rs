use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use std::str::from_utf8;

#[derive(Debug)]
pub enum Classification {
    Unclassified,
    Secret,
    ClassificatedError
}

#[derive(Debug)]
pub struct Element {
    pub name: String,
    pub norad_id: u64,
    pub classification: Classification,
    pub international_designator: String,
    pub datetime: NaiveDateTime,
    pub mean_motion_dot: f64,
    pub mean_motion_dotdot: String,
    pub bster_drag_term: String,
    pub ephemeris_type: f64,
    pub element_number: f64,
    pub inclination: f64,
    pub right_ascension: f64,
    pub eccentricity: f64,
    pub argument_of_perigee: f64,
    pub mean_anomaly: f64,
    pub mean_motion: f64,
    pub revolution_number: f64,
}

impl Element {
    pub fn from_tle(line1: String, line2: String, line3: String) -> Result<Element, Box<dyn std::error::Error>> {
        let line2: Vec<&[u8]> = line2
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|c| c.as_bytes()).collect();
        let line3: Vec<&[u8]> = line3
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|c| c.as_bytes()).collect();

        let element = Element {
            name: line1,
            norad_id: from_utf8(&line2[1][0..line2[1].len()-2])?.parse()?,
            // Unchecked!
            classification: match line2[1].last().unwrap() {
                83 => Classification::Secret,
                85 => Classification::Unclassified,
                _ => Classification::ClassificatedError
            },
            international_designator: from_utf8(line2[2])?.to_string(),
            datetime: parse_date_time(from_utf8(line2[3])?.parse()?).unwrap(),
            mean_motion_dot: from_utf8(line2[4])?.parse()?,
            mean_motion_dotdot: from_utf8(line2[5])?.to_string(),
            bster_drag_term: from_utf8(line2[6])?.to_string(),
            ephemeris_type: from_utf8(line2[7])?.parse()?,
            element_number: from_utf8(&line2[8][0..line2[8].len()-1])?.parse()?,
            inclination: from_utf8(line3[2])?.parse()?,
            right_ascension: from_utf8(line3[3])?.parse()?,
            eccentricity: from_utf8(line3[4])?.parse()?,
            argument_of_perigee: from_utf8(line3[5])?.parse()?,
            mean_anomaly: from_utf8(line3[6])?.parse()?,
            mean_motion: from_utf8(&line3[7][0..=10])?.parse()?,
            revolution_number: from_utf8(&line3[7][11..line3[7].len()-1])?.parse()?
            
        };

        Ok(element)
    }
}

fn parse_date_time(input: f64) -> Option<chrono::NaiveDateTime> {
    let input_str = input.to_string();

    let year = &input_str[0..2];
    let day = &input_str[2..];

    let year = match year.parse::<i32>() {
        Ok(year) => {
            if year >= 57 && year <= 99 {
                1900 + year
            } else {
                2000 + year
            }
        }
        Err(_) => return None,
    };

    let day = match day.parse::<f64>() {
        Ok(day) => day,
        Err(_) => return None,
    };

    let seconds = day.fract() * (24.0 * 60.0 * 60.0);
    let mut nsecs = (seconds.fract() * 1e9).round() as u32;
    let mut seconds = seconds as u32;
    if nsecs >= 1_000_000_000 {
        nsecs -= 1_000_000_000;
        seconds += 1;
    }

    println!("{}, {}, {}, {}", year, day, seconds, nsecs);
    let date = NaiveDate::from_yo_opt(year, day as u32).unwrap().and_time(NaiveTime::from_num_seconds_from_midnight_opt(seconds, nsecs).unwrap());

    Some(date)
}
