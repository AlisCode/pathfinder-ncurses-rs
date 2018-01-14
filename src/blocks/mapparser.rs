use nom::{IResult, digit, rest};
use std::str::{FromStr, from_utf8};

use std::fs::File;
use std::io::prelude::*;

use blocks::map::Map;
use blocks::case::{Case, TypeCase};

/// Stores the basic informations about a map
#[derive(Debug)]
pub struct MapInfos {
    width: i32,
    height: i32,
    flags: Vec<u8>,
}

/// MapInfos functions
impl MapInfos {
    /// Constructor of MapInfos
    /// width: i32
    pub fn new(width: i32, height: i32, flags: Vec<u8>) -> Self {
        let flags = flags
            .into_iter()
            .filter(|x| *x != b'\n' && *x != b'\r')
            .collect();

        MapInfos {
            width,
            height,
            flags,
        }
    }

    pub fn create_map(self) -> Map {

        let mut m: Map = Map::new();

        for (i, val) in self.flags.into_iter().enumerate() {
            let x = i as i32 % self.width + 1;
            let y = i as i32 / self.width + 1;
            let case: Case = Case::new(x, y, TypeCase::from_u8(val));
            m.add_case(case);
        }

        m.set_width_height(self.width, self.height);

        m
    }
}


/// Generic parser for numeric values
named!(numeric_string<&str>,
    map_res!(
        digit,
        from_utf8
    )
);

/// Generic parser for an i32 value
named!(i32_digit<i32>,
    map_res!(
        numeric_string,
        FromStr::from_str
    )
);

/// Parser to get the map's width
named!(get_map_width<&[u8], i32>, preceded!(tag!("w:"), i32_digit));

// Parser to get the map's height
named!(get_map_height<&[u8], i32>, preceded!(tag!("h:"), i32_digit));

// Parser for the whole map format
named!(parse_map_infos<&[u8], MapInfos>, 
do_parse!(
    width: get_map_width >>     // Gets the map width
    opt!(tag!("\r")) >>         // Allows to jump lines between infos (windows)
    opt!(tag!("\n")) >>         // Allows to jump lines between infos
    height: get_map_height >>   // Gets the map height
    opt!(tag!("\r")) >>         // Allows to jump lines between infos (windows)
    opt!(tag!("\n")) >>         // Allows to jump lines between infos
    tag!("d:") >>               // Detects the start of the data bytes
    flags: rest >>              // All the bytes until the end are the map's data
    (MapInfos::new(width, height, flags.into())) // Returns the MapInfos struct
));

/// Loads the map using the given filename.
/// Returns a MapInfos struct if the map could be loaded, or an error with a &'static str explaining why the map could not be loaded
pub fn load_map(filename: String) -> Result<MapInfos, &'static str> {
    // Opens the file
    let mut file = match File::open(filename) {
        Ok(file) => file,
        _ => return Err("File not found"),
    };

    // Reads the file and tries to parse it using nom and the parses defined previously
    let mut buf = vec![];
    let res = match file.read_to_end(&mut buf) {
        Ok(_) => parse_map_infos(&buf),
        Err(_) => return Err("Could not read file"),
    };

    // Matches the result to return what we want
    match res {
        IResult::Done(_, map_infos) => Ok(map_infos),
        IResult::Error(_) => Err("Invalid map format"),
        _ => Err("Incomplete file"),
    }
}
