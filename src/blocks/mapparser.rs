use nom::{IResult, digit, eol};
use std::str::{FromStr,from_utf8};

#[derive(Debug)]
pub struct MapInfos {
    width: i32,
    height: i32,
    flags: Vec<u8>,
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

named!(all_bytes_to_end< &[u8] >, 
    take_until!(
        eol
    )
);

named!(get_map_width<&[u8], i32>, preceded!(tag!("w:"), i32_digit));
named!(get_map_height<&[u8], i32>, preceded!(tag!("h:"), i32_digit));
named!(get_map_flags<&[u8], &[u8] >, preceded!(tag!("d:"), all_bytes_to_end));

named!(parse_map_infos<&[u8], MapInfos>, 
do_parse!(
    width: get_map_width >>
    height: get_map_height >>
    flags: get_map_flags >>
    (MapInfos { width: width, height: height, flags: flags.into()})
));

pub fn load_map() -> Result<MapInfos, &'static str> {

    let res = parse_map_infos(b"w:15h:15d:1111100011100011100");

    match res {
        IResult::Done(_,map_infos) => {
            Ok(map_infos)
        },
        IResult::Error(_) => Err("Error parsing file"),
        _ => Err("Incomplete file"), 
    }
    
}