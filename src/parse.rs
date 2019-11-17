use nom::character::*;
use nom::error::ErrorKind;

use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::io::ErrorKind::InvalidInput;

pub type ParserResult<'a, T> = Result<(&'a [u8], T), nom::Err<(&'a [u8], ErrorKind)>>;

pub fn slice_to_string(slice: &[u8]) -> Result<String, IoError> {
	if slice.is_empty() {
		Err(IoError::new(InvalidInput, "slice has length 0"))
	} else {
		Ok(
			String::from_utf8(Vec::from(slice))
	        .map_err(|_| IoError::new(IoErrorKind::InvalidInput, "Failed to parse utf8 string"))?
		)
	}
}

pub fn parse_u32(slice: &[u8]) -> Result<u32, IoError> {
	Ok(
		::std::str::from_utf8(slice)
		.map_err(|_| IoError::new(IoErrorKind::InvalidInput, "Failed to parse utf8 u32 integer"))?
		.parse()
		.map_err(|_| IoError::new(IoErrorKind::InvalidInput, "Failed to parse u32 integer"))?
	)
}

pub fn parse_u64(slice: &[u8]) -> Result<u64, IoError> {
	Ok(
		::std::str::from_utf8(slice)
		.map_err(|_| IoError::new(IoErrorKind::InvalidInput, "Failed to parse utf8 u64 integer"))?
		.parse()
		.map_err(|_| IoError::new(IoErrorKind::InvalidInput, "Failed to parse u64 integer"))?
	)
}

named!(pub parse_string<String>, map_res!(
    take_while!(is_alphanumeric), slice_to_string
));
