use crate::colors::{rgb::RGBColor, ColorComponent, ColorComponentConvert};

#[derive(Debug, Clone, Copy)]
pub struct HSVColor {
    pub hue: ColorComponent<u16>,
    pub saturation: ColorComponent<f64>,
    pub value: ColorComponent<f64>,
}

impl HSVColor {
    pub fn new(hue: u16, saturation: f64, value: f64) -> Self {
        Self {
            hue: hue.clamp(0, 360).into(),
            saturation: saturation.clamp(0.0, 1.0).into(),
            value: value.clamp(0.0, 1.0).into(),
        }
    }

    pub fn to_str(&self) -> String {
        format!(
            "{}, {:.2}, {:.2}",
            self.hue.to_str(),
            self.saturation.value,
            self.value.value
        )
    }
}

impl From<RGBColor> for HSVColor {
    fn from(rgb: RGBColor) -> Self {
        let f_red = rgb.red.value as f64 / 255.0;
        let f_green = rgb.green.value as f64 / 255.0;
        let f_blue = rgb.blue.value as f64 / 255.0;

        let max = f_red.max(f_green.max(f_blue));
        let min = f_red.min(f_green.min(f_blue));
        let del = max - min;

        if del == 0.0 {
            return HSVColor::new(0, 0.0, min);
        }

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

        let comp_h = (60.0 * (h - d / del)).round() as u16;
        let comp_s = del / max;
        let comp_v = max;

        HSVColor::new(comp_h, comp_s, comp_v)
    }
}
