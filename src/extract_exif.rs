use exif::{In, Tag, Value};

pub fn extract_exif() {
    let folder_path = std::fs::read_dir("./pictures").expect("Directory read ERROR");

    for pic in folder_path {
        println!("{:?}", pic.as_ref().unwrap().path().file_name().unwrap());
        let file = std::fs::File::open(pic.unwrap().path().as_path()).expect("file ooops");

        let mut bufreader = std::io::BufReader::new(&file);

        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader).expect("exif read ooops");

        let gps_check_data = exif.get_field(Tag::GPSLatitude, In::PRIMARY);

        if gps_check_data.is_none() {
            println!("File contains no Exif Data")
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

        match exif.get_field(Tag::GPSLatitude, In::PRIMARY) {
            Some(xres) => match xres.value {
                Value::Rational(ref v) if !v.is_empty() => {
                    if gps_lat.to_string().ends_with("W") {
                        let lodeg = v[0].to_f64() + v[1].to_f64() / 60.0 + v[2].to_f64() / 3600.0;

                        print!("https://maps.google.com/?q={},", lodeg * -1.0);
                    } else if gps_lat.to_string().ends_with("S") {
                        let lodeg = v[0].to_f64() + v[1].to_f64() / 60.0 + v[2].to_f64() / 3600.0;
                        print!("https://maps.google.com/?q={},", lodeg * -1.0);
                    } else {
                        let lodeg = v[0].to_f64() + v[1].to_f64() / 60.0 + v[2].to_f64() / 3600.0;

                        print!("https://maps.google.com/?q={},", lodeg);
                    }
                }

                _ => eprintln!("Latitude value is broken"),
            },

            None => eprintln!("Latitude tag is missing"),
        }

        match exif.get_field(Tag::GPSLongitude, In::PRIMARY) {
            Some(xres) => match xres.value {
                Value::Rational(ref v) if !v.is_empty() => {
                    if gps_long.to_string().ends_with("W") {
                        let lodeg = v[0].to_f64() + v[1].to_f64() / 60.0 + v[2].to_f64() / 3600.0;
                        println!("{}", lodeg * -1.0)
                    } else if gps_long.to_string().ends_with("S") {
                        let lodeg = v[0].to_f64() + v[1].to_f64() / 60.0 + v[2].to_f64() / 3600.0;
                        println!("{}", lodeg * -1.0)
                    } else {
                        let lodeg = v[0].to_f64() + v[1].to_f64() / 60.0 + v[2].to_f64() / 3600.0;
                        println!("{}", lodeg)
                        
                    }
                }

                _ => eprintln!("Longitude value is broken"),
            },
            None => eprintln!("Longitude tag is missing"),
        }
        }
    }
}
