use std::fs::File;
use std::io::{BufRead, BufReader};
use super::element::Element;

pub fn tle_parse(filename: &str) -> Result<Vec<Element>, Box<dyn std::error::Error>>{
    let file = File::open(filename).unwrap();
    let mut lines = BufReader::new(file).lines();

    let mut elements = Vec::new();
    while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        let line1 = line1.unwrap();
        let line2 = line2.unwrap();
        let line3 = line3.unwrap();

        let element = Element::from_tle(line1, line2, line3)?;
        elements.push(element);
    }

    Ok(elements)

}
