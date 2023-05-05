fn main() {
    extract_exif();
}

extern crate exif;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::os::fd::AsFd;
use std::path::{Path, PathBuf};

use exif::{In, Tag};
fn extract_exif() -> Result<(), exif::Error> {
    let folder_path = std::fs::read_dir("./pictures").expect("file import ERROR");

    for pic in folder_path {
        println!("{:?}", pic.as_ref().unwrap().path().file_name().unwrap());
        let file = std::fs::File::open(pic.unwrap().path().as_path())?;

        let mut bufreader = std::io::BufReader::new(&file);

        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader).unwrap();

        let gps_lat = exif.get_field(Tag::GPSLatitude, In::PRIMARY);
        let gps_long = exif.get_field(Tag::GPSLongitude, In::PRIMARY);

        if gps_lat.is_none() {
            println!("File contains no Exif Data")
        } else {
            let print_lat = gps_lat
                .expect("NO DATA")
                .display_value()
                .with_unit(&exif)
                .to_string();
            let print_long = gps_long
                .expect("NO DATA")
                .display_value()
                .with_unit(&exif)
                .to_string();

            let reg_exclude = Regex::new(r"[a-z]").unwrap();

            let reg_replace_empty_str_lat = reg_exclude.replace_all(&print_lat, "");
            let reg_replace_empty_str_long = reg_exclude.replace_all(&print_long, "");

            println!(
                "Latitude,Longitude: {} {}",
                reg_replace_empty_str_lat, reg_replace_empty_str_long
            );
        }
    }

    Ok(())
}

// Todo :
//        parse to load straight to google map .
//        print to terminal, print to file ?
