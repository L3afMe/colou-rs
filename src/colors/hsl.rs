use crate::colors::{rgb::RGBColor, ColorComponent, ColorComponentConvert, ComplementaryColors};

use super::{ColorManipulate, ROT_OFFSET};

#[derive(Debug, Clone, Copy)]
pub struct HSLColor {
    pub hue: ColorComponent<u16>,
    pub saturation: ColorComponent<f64>,
    pub lightness: ColorComponent<f64>,
}

impl HSLColor {
    pub fn new(hue: u16, saturation: f64, lightness: f64) -> Self {
        Self {
            hue: hue.clamp(0, 360).into(),
            saturation: saturation.clamp(0.0, 1.0).into(),
            lightness: lightness.clamp(0.0, 1.0).into(),
        }
    }

    pub fn to_str(&self) -> String {
        format!(
            "{}, {:.2}, {:.2}",
            self.hue.to_str(),
            self.saturation.value,
            self.lightness.value
        )
    }
}

impl From<RGBColor> for HSLColor {
    fn from(rgb: RGBColor) -> Self {
        let f_red = rgb.red.value as f64 / 255.0;
        let f_green = rgb.green.value as f64 / 255.0;
        let f_blue = rgb.blue.value as f64 / 255.0;

        let max = f_red.max(f_green.max(f_blue));
        let min = f_red.min(f_green.min(f_blue));
        let del = max - min;

        let l = (min + max) / 2.0;

        let h;
        let d;
        if f_red == min {
            d = f_green - f_blue;
            h = 3.0;
        } else if f_blue == min {
            d = f_red - f_green;
            h = 1.0;
        } else {
            d = f_blue - f_red;
            h = 5.0;
        };

        let s = if l == 0.0 || l == 1.0 {
            0.0
        } else {
            del / (1.0 - (2.0 * l - 1.0).abs())
        };

        let comp_h = (60.0 * (h - d / del)).round() as u16;
        let comp_s = s;
        let comp_l = l;

        HSLColor::new(comp_h, comp_s, comp_l)
    }
}

impl ColorManipulate for HSLColor {
    fn rotate_hue(&self, amount: i16) -> Self {
        let mut color = self.clone();
        color.hue.value = (self.hue.value as i16 + amount).abs() as u16 % 360;

        color
    }

    fn brighten(&self, amount: f64) -> Self {
        self.clone()
    }

    fn darken(&self, amount: f64) -> Self {
        self.clone()
    }
}

impl ComplementaryColors for HSLColor {
    fn analogous(self) -> [Self; 3]
    where
        Self: Sized,
    {
        let first = self.rotate_hue(-(ROT_OFFSET as i16));
        let third = self.rotate_hue(ROT_OFFSET as i16);

        [first, self, third]
    }

    fn triad(self) -> [Self; 3]
    where
        Self: Sized,
    {
        let second = self.rotate_hue(120);
        let third = second.rotate_hue(120);

        [self, second, third]
    }

    fn tetradic(self) -> [Self; 4]
    where
            Self: Sized {
        let second = self.rotate_hue(ROT_OFFSET as i16);
        let third = self.rotate_hue(180);
        let fourth = third.rotate_hue(ROT_OFFSET as i16);

        [self, second, third, fourth]
    }

    fn shades(self) -> [Self; 16]
    where
        Self: Sized,
    {
        let mut colors = [self; 16];

        for i in 1..=16 {
            let mut color = self.clone();
            color.lightness.value = 1.0 / 17.0 * i as f64;
            colors[i - 1] = color;
        }

        colors
    }

    fn split_complemetary(self) -> [Self; 3]
    where
        Self: Sized,
    {
        let first = self.rotate_hue(180 - ROT_OFFSET as i16);
        let third = self.rotate_hue(180 + ROT_OFFSET as i16);

        [first, self, third]
    }
}
