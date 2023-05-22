use exif::{In, Tag, Value};
use std::fmt;

struct GPSData {
    image: String,
    latitude: f64,
    longitude: f64,
}

impl GPSData {
    fn new(file_buffer: std::path::PathBuf) -> Self {
        let image = file_buffer
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let file = std::fs::File::open(file_buffer.as_path()).expect("file ooops");
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = exifreader
            .read_from_container(&mut bufreader)
            .expect("exif read ooops");

        let gps_check_data = exif.get_field(Tag::GPSLatitude, In::PRIMARY);
        let gps_long_data = exif.get_field(Tag::GPSLongitude, In::PRIMARY);

        if gps_check_data.is_none() {
            println!("File contains no Exif Data");
            todo!();
        } else {
            let gps_lat = &exif
                .get_field(Tag::GPSLatitude, In::PRIMARY)
                .unwrap()
                .display_value()
                .with_unit(&exif);

            let gps_long = exif
                .get_field(Tag::GPSLongitude, In::PRIMARY)
                .unwrap()
                .display_value()
                .with_unit(&exif);

            // latitude first , then longitude for maps.
            // match statements combine the string needed to load straight to google maps with print! macro.

            let latitude = match gps_check_data {
                Some(xres) => match xres.value {
                    Value::Rational(ref v) if !v.is_empty() => {
                        let mut formula =
                            (v[0].to_f64() + v[1].to_f64() / 60.0 + v[2].to_f64() / 3600.0) * -1.0; //assume this W or S.

                        if !gps_lat.to_string().ends_with("W")
                            && !gps_lat.to_string().ends_with("S")
                        {
                            formula *= -1.0;
                        }
                        formula
                    }

                    _ => {
                        eprintln!("Latitude value is broken");
                        todo!();
                    }
                },

                None => {
                    eprintln!("Latitude tag is missing");
                    todo!();
                }
            };

            let longitude = match gps_long_data {
                Some(xres) => match xres.value {
                    Value::Rational(ref v) if !v.is_empty() => {
                        let mut formula =
                            (v[0].to_f64() + v[1].to_f64() / 60.0 + v[2].to_f64() / 3600.0) * -1.0; //assume this W or S.

                        if !gps_long.to_string().ends_with("W")
                            && !gps_long.to_string().ends_with("S")
                        {
                            formula *= -1.0;
                        }
                        formula
                    }

                    _ => {
                        eprintln!("Longitude value is broken");
                        todo!();
                    }
                },

                None => {
                    eprintln!("Longitude tag is missing");
                    todo!();
                }
            };
            Self {
                image,
                latitude,
                longitude,
            }
        }
    }
}

impl fmt::Display for GPSData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\nhttps://maps.google.com/?q={},{}\n",
            self.image, self.latitude, self.longitude
        )
    }
}

pub fn extract_exif() {
    let folder_path = std::fs::read_dir("./pictures").expect("Directory read ERROR");

    for pic in folder_path {
        let gps_data = GPSData::new(pic.unwrap().path());
        print!("{}", gps_data);
    }
}

//"https://maps.google.com/?q={},"
