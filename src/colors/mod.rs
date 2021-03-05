pub mod ansi;
pub mod hsl;
pub mod hsv;
pub mod rgb;

use num::Integer;
use once_cell::sync::Lazy;
use regex::bytes::Regex;
use std::fmt::{Display, UpperHex};

pub const ROT_OFFSET: u8 = 30;

pub enum Error {
    ParseError(String),
}

pub const HEX_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^#?(?:(?:[a-fA-F0-9]{2})|[a-fA-F0-9]){3}$").unwrap());
pub const RGB_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^((25[0-5]|2[0-4][0-9]|[0-1]?[0-9]{0,2}),){2}(25[0-5]|2[0-4][0-9]|[0-1]?[0-9]{0,2})$").unwrap()
});

pub fn is_hex<T: ToString>(value: &T) -> bool {
    HEX_REGEX.is_match(value.to_string().into_bytes().as_slice())
}

pub fn is_rgb<T: ToString>(value: &T) -> bool {
    RGB_REGEX.is_match(value.to_string().into_bytes().as_slice())
}

pub trait ColorComponentConvert {
    fn to_str(&self) -> String;
    fn to_hex(&self) -> String;
}

#[derive(Debug, Clone, Copy)]
pub struct ColorComponent<T> {
    pub value: T,
}

impl<T> From<T> for ColorComponent<T> {
    fn from(value: T) -> Self {
        ColorComponent { value }
    }
}

impl<T> ColorComponentConvert for ColorComponent<T>
where
    T: Display + UpperHex + Integer,
{
    fn to_str(&self) -> String {
        format!("{:0>3}", self.value)
    }

    fn to_hex(&self) -> String {
        format!("{:02X}", self.value)
    }
}

// impl<T> ColorComponentConvert for ColorComponent<T>
// where T: Display + Float + Into<u8>{
//     fn to_str(&self) -> String {
//         format!("{:.2}", self.value)
//     }

//     fn to_hex(&self) -> String {
//         format!("{:02X}", (self.value.min(NumCast::from(1.0).unwrap()) * NumCast::from(255.0).unwrap()).into())
//     }
// }

pub trait ComplementaryColors {
    fn triad(self) -> [Self; 3]
    where
        Self: Sized;
    fn tetradic(self) -> [Self; 4]
    where
        Self: Sized;
    fn analogous(self) -> [Self; 3]
    where
        Self: Sized;
    fn shades(self) -> [Self; 16]
    where
        Self: Sized;
    fn split_complemetary(self) -> [Self; 3]
    where
        Self: Sized;
}

trait ColorManipulate {
    fn rotate_hue(&self, amount: i16) -> Self;
    fn brighten(&self, amount: f64) -> Self;
    fn darken(&self, amount: f64) -> Self;
}
