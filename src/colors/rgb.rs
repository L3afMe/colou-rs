use crate::colors::{ansi::ColorAnsi, hsl::HSLColor, ColorComponent, ColorComponentConvert, Error};
use rand::Rng;

use super::ComplementaryColors;

#[derive(Debug, Clone, Copy)]
pub struct RGBColor {
    pub red: ColorComponent<u8>,
    pub green: ColorComponent<u8>,
    pub blue: ColorComponent<u8>,
}

#[allow(dead_code)]
impl RGBColor {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red: red.clamp(0, 255).into(),
            green: green.clamp(0, 255).into(),
            blue: blue.clamp(0, 255).into(),
        }
    }

    pub fn from_rgb<T: ToString>(value: &T) -> Result<Self, Error> {
        let rgb = value
            .to_string()
            .split(",")
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let red = if let Ok(color) = rgb[0].parse::<u8>() {
            color
        } else {
            return Err(Error::ParseError("red".to_string()));
        };
        let green = if let Ok(color) = rgb[1].parse::<u8>() {
            color
        } else {
            return Err(Error::ParseError("green".to_string()));
        };
        let blue = if let Ok(color) = rgb[2].parse::<u8>() {
            color
        } else {
            return Err(Error::ParseError("blue".to_string()));
        };

        let color = RGBColor::new(red, green, blue);
        Ok(color)
    }

    pub fn from_hex<T: ToString>(value: &T) -> Result<Self, Error> {
        let mut hex = value.to_string().trim_start_matches("#").to_string();

        if hex.len() == 3 {
            let r = hex.chars().nth(0).unwrap().to_string();
            let g = hex.chars().nth(1).unwrap().to_string();
            let b = hex.chars().nth(2).unwrap().to_string();

            hex = format!("{}{}{}{}{}{}", r, r, g, g, b, b);
        }

        let rgb = vec![hex[..2].to_string(), hex[2..4].to_string(), hex[4..].to_string()];

        let red = if let Ok(color) = u8::from_str_radix(&rgb[0], 16) {
            color
        } else {
            return Err(Error::ParseError("red".to_string()));
        };
        let green = if let Ok(color) = u8::from_str_radix(&rgb[1], 16) {
            color
        } else {
            return Err(Error::ParseError("green".to_string()));
        };
        let blue = if let Ok(color) = u8::from_str_radix(&rgb[2], 16) {
            color
        } else {
            return Err(Error::ParseError("blue".to_string()));
        };

        let color = RGBColor::new(red, green, blue);
        Ok(color)
    }

    pub fn new_rand() -> Self {
        let red = rand::thread_rng().gen_range(0..=255);
        let green = rand::thread_rng().gen_range(0..=255);
        let blue = rand::thread_rng().gen_range(0..=255);

        RGBColor::new(red, green, blue)
    }

    pub fn is_light(&self) -> bool {
        self.red.value as u16 + self.green.value as u16 + self.blue.value as u16 >= 383
    }

    pub fn to_ansi_foreground(&self) -> ColorAnsi {
        ColorAnsi::new(Some(self.clone()), None)
    }

    pub fn to_ansi_background(&self) -> ColorAnsi {
        ColorAnsi::new(None, Some(self.clone()))
    }

    pub fn paint_hex(&self) -> String {
        let ansi = self.to_ansi_background();
        ansi.paint(self.to_hex())
    }

    pub fn to_hex(&self) -> String {
        format!("#{}{}{}", self.red.to_hex(), self.green.to_hex(), self.blue.to_hex(),)
    }

    pub fn to_rgb(&self, separator: &str) -> String {
        format!(
            "{}{}{}{}{}",
            self.red.to_str(),
            separator,
            self.green.to_str(),
            separator,
            self.blue.to_str(),
        )
    }

    pub fn invert(&self) -> RGBColor {
        RGBColor::new(255 - self.red.value, 255 - self.green.value, 255 - self.blue.value)
    }
}

impl From<HSLColor> for RGBColor {
    fn from(color: HSLColor) -> Self {
        let chroma = (1.0 - (2.0 * color.lightness.value - 1.0).abs()) * color.saturation.value;
        let h = color.hue.value as f64 / 60.0;
        let x = chroma * (1.0 - (h % 2.0 - 1.0).abs());
        let m = color.lightness.value - chroma / 2.0;

        let (r, g, b) = if h == 0.0 {
            (0.0, 0.0, 0.0)
        } else if h <= 1.0 {
            (chroma, x, 0.0)
        } else if h <= 2.0 {
            (x, chroma, 0.0)
        } else if h <= 3.0 {
            (0.0, chroma, x)
        } else if h <= 4.0 {
            (0.0, x, chroma)
        } else if h <= 5.0 {
            (x, 0.0, chroma)
        } else {
            (chroma, 0.0, x)
        };

        let calc_r = ((r + m) * 255.0).round() as u8;
        let calc_b = ((b + m) * 255.0).round() as u8;
        let calc_g = ((g + m) * 255.0).round() as u8;

        RGBColor::new(calc_r, calc_g, calc_b)
    }
}

impl ComplementaryColors for RGBColor {
    fn analogous(self) -> [Self; 3]
    where
        Self: Sized,
    {
        let hsl = HSLColor::from(self);
        let analogous = hsl.analogous();

        [analogous[0].into(), analogous[1].into(), analogous[2].into()]
    }

    fn triad(self) -> [Self; 3]
    where
        Self: Sized,
    {
        let hsl = HSLColor::from(self);
        let triad = hsl.triad();

        [triad[0].into(), triad[1].into(), triad[2].into()]
    }

    fn tetradic(self) -> [Self; 4]
    where
            Self: Sized {
        let hsl = HSLColor::from(self);
        let tetradic = hsl.tetradic();

        [tetradic[0].into(), tetradic[1].into(), tetradic[2].into(), tetradic[3].into()]
    }

    fn shades(self) -> [Self; 16]
    where
        Self: Sized,
    {
        let hsl = HSLColor::from(self);
        let shades = hsl.shades();

        let mut out = [self; 16];
        for i in 0..16 {
            out[i] = shades[i].into();
        }
        out
    }

    fn split_complemetary(self) -> [Self; 3]
    where
            Self: Sized {
        let hsl = HSLColor::from(self);
        let comp = hsl.split_complemetary();

        [comp[0].into(), comp[1].into(), comp[2].into()]
    }
}
