use nom::{IResult, digit, rest};
use std::str::{FromStr,from_utf8};

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct MapInfos {
    width: i32,
    height: i32,
    flags: Vec<u8>,
}

impl MapInfos {

    pub fn new(width: i32, height: i32, flags: Vec<u8>) -> Self {

        let flags = flags.into_iter().filter(|x| *x != b'\n').collect();

        MapInfos { 
            width: width,
            height: height,
            flags: flags,        
        }
    }
}

named!(numeric_string<&str>,
    map_res!(
        digit,
        from_utf8
    )
);

named!(i32_digit<i32>,
    map_res!(
        numeric_string,
        FromStr::from_str
    )
);

named!(get_map_width<&[u8], i32>, preceded!(tag!("w:"), i32_digit));
named!(get_map_height<&[u8], i32>, preceded!(tag!("h:"), i32_digit));

named!(parse_map_infos<&[u8], MapInfos>, 
do_parse!(
    width: get_map_width >>
    opt!(tag!("\n")) >>
    height: get_map_height >>
    opt!(tag!("\n")) >>
    tag!("d:") >>
    flags: rest >>
    (MapInfos::new(width, height, flags.into()))
));

pub fn load_map(filename: String) -> Result<MapInfos, &'static str> {

    // Opens the file
    let mut file = match File::open(filename) {
        Ok(file) => file,
        _ => return Err("File not found"),
    };

    // Reads the file
    let mut buf = vec!();
    let res = match file.read_to_end(&mut buf) {
        Ok(_) => parse_map_infos(&buf),
        Err(_) => return Err("Could not read file"), 
    };

    // Matches the result to return what we want
    match res {
        IResult::Done(_,map_infos) => {
            Ok(map_infos)
        },
        IResult::Error(_) => Err("Invalid map format"),
        _ => Err("Incomplete file"), 
    }
    
}