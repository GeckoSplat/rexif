use exif::{In, Tag, Value};
use std::fmt;

struct GPSData2 {
    image: String,
    latitude: f64,
    longitude: f64,
}

impl GPSData2 {
    fn new(image: String, latitude: f64, longitude: f64) -> Self {
        Self {
            image,
            latitude,
            longitude,
        }
    }
}

impl fmt::Display for GPSData2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\nhttps://maps.google.com/?q={},{}\n",
            self.image, self.latitude, self.longitude
        )
    }
}

struct Parser;

impl Parser {
    fn new(path: &str) -> Vec<GPSData2> {
        let folder_path = std::fs::read_dir(path).expect("Directory read ERROR");
        let mut gps2vec = vec![];
        for mut file_buffer in folder_path {
            let file = std::fs::File::open(file_buffer.as_mut().unwrap().path().as_path())
                .expect("file ooops");
            let mut bufreader = std::io::BufReader::new(&file);
            let exifreader = exif::Reader::new();
            let exif = exifreader
                .read_from_container(&mut bufreader)
                .expect("exif read ooops");

            let gps_lat_data = exif.get_field(Tag::GPSLatitude, In::PRIMARY);
            let gps_long_data = exif.get_field(Tag::GPSLongitude, In::PRIMARY);
            if gps_lat_data.is_none() {
                println!("File contains no Exif Data");
                continue;
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

                let latitude = match gps_lat_data {
                    Some(xres) => match xres.value {
                        Value::Rational(ref v) if !v.is_empty() => {
                            let mut formula =
                                (v[0].to_f64() + v[1].to_f64() / 60.0 + v[2].to_f64() / 3600.0)
                                    * -1.0; //assume this W or S.

                            if !gps_lat.to_string().ends_with("W")
                                && !gps_lat.to_string().ends_with("S")
                            {
                                formula *= -1.0;
                            }
                            formula
                        }

                        _ => 0.0,
                    },

                    None => 0.0,
                };

                let longitude = match gps_long_data {
                    Some(xres) => match xres.value {
                        Value::Rational(ref v) if !v.is_empty() => {
                            let mut formula =
                                (v[0].to_f64() + v[1].to_f64() / 60.0 + v[2].to_f64() / 3600.0)
                                    * -1.0; //assume this W or S.

                            if !gps_long.to_string().ends_with("W")
                                && !gps_long.to_string().ends_with("S")
                            {
                                formula *= -1.0;
                            }
                            formula
                        }

                        _ => 0.0,
                    },

                    None => 0.0,
                };
                let image = file_buffer // fixed error by making mut on line 150.
                    .unwrap()
                    .file_name()
                    .to_str()
                    .unwrap()
                    .to_string();
                gps2vec.push(GPSData2::new(image.clone(), latitude, longitude));
            }
        }
        return gps2vec;
    }
}

pub fn extract_exif() {
    let gps_data = Parser::new("./pictures");
    for data in gps_data {
        println!("{}", data);
    }
}
