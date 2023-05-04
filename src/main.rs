
fn main() {
    extract_exif();
}

extern crate exif;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use exif::{In, Tag};
fn extract_exif() -> Result<(), exif::Error> {
    let folder_path = std::fs::read_dir("./pictures").expect("file import ERROR");

    for pic in folder_path {
        let file = std::fs::File::open(pic.unwrap().path().as_path())?;
        println!("{:?}", file);

        let mut bufreader = std::io::BufReader::new(&file);
        // println!("{:?}",bufreader);
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader).unwrap();

        let gps_lat = exif.get_field(Tag::GPSLatitude, In::PRIMARY).unwrap(); // problem here if data absent
        let gps_long = exif.get_field(Tag::GPSLongitude, In::PRIMARY).unwrap();

        let print_lat = gps_lat.display_value().with_unit(&exif).to_string();
        let print_long = gps_long.display_value().with_unit(&exif).to_string();

        let reg_exclude = Regex::new(r"[a-z]").unwrap();

        let reg_replace_empty_str_lat = reg_exclude.replace_all(&print_lat, "");
        let reg_replace_empty_str_long = reg_exclude.replace_all(&print_long, "");

        println!(
            "Latitude,Longitude: {} {}",
            reg_replace_empty_str_lat, reg_replace_empty_str_long
        );
    }
    Ok(())
}

// Todo : negate empty data at start , match ? 
//        parse to load straight to google map .
//        print to terminal, print to file ? 