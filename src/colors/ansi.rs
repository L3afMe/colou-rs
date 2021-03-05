use crate::colors::{rgb::RGBColor, ColorComponentConvert};

pub const ANSI_RESET: &str = "\x1b[0m";

#[derive(Debug, Clone, Copy)]
pub struct ColorAnsi {
    pub foreground: Option<RGBColor>,
    pub background: Option<RGBColor>,
    pub bold: bool,
    pub underline: bool,
}

#[allow(dead_code)]
impl ColorAnsi {
    pub fn new(foreground: Option<RGBColor>, background: Option<RGBColor>) -> Self {
        Self {
            foreground,
            background,
            bold: false,
            underline: false,
        }
    }

    pub fn paint<T: ToString>(&self, value: T) -> String {
        format!("{}{}{}", self.to_escape_str(), value.to_string(), ANSI_RESET)
    }

    pub fn to_escape_str(&self) -> String {
        let mut esc_str_parts: Vec<String> = Vec::new();

        if self.bold {
            esc_str_parts.push(String::from("1"));
        }

        if self.underline {
            esc_str_parts.push(String::from("4"));
        }

        if let Some(bg_color) = self.background {
            esc_str_parts.push(format!(
                "48;2;{};{};{}",
                bg_color.red.to_str(),
                bg_color.green.to_str(),
                bg_color.blue.to_str()
            ));
        }

        if let Some(fg_color) = self.foreground {
            esc_str_parts.push(format!(
                "38;2;{};{};{}",
                fg_color.red.to_str(),
                fg_color.green.to_str(),
                fg_color.blue.to_str()
            ));
        }

        let esc_str = esc_str_parts.join(";");
        format!("\x1b[{}m", esc_str)
    }

    pub fn background(&mut self, background: RGBColor) -> &Self {
        self.background = Some(background);

        self
    }

    pub fn foreground(&mut self, foreground: RGBColor) -> &Self {
        self.foreground = Some(foreground);

        self
    }

    pub fn bold(&mut self, bold: bool) -> &Self {
        self.bold = bold;

        self
    }

    pub fn underline(&mut self, underline: bool) -> &Self {
        self.underline = underline;

        self
    }
}
