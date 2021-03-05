mod colors;

use std::env;

use colors::ComplementaryColors;
use colors::rgb::RGBColor;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    if args.len() == 0 || args[0].eq("-h") || args[0].eq("--help") {
        send_help();

        return;
    }

    let mut mode = 0;

    match args[0].as_ref() {
        "-s" | "--shades" => mode = 1,
        "-c" | "--complementary" => mode = 2,
        _ => {}
    }

    if mode != 0 {
        args.remove(0);
    }

    let mut colors: Vec<RGBColor> = Vec::new();
    for arg in args {
        if colors::is_rgb(&arg) {
            match RGBColor::from_rgb(&arg) {
                Ok(color) => {
                    colors.push(color);
                }
                Err(why) => {
                    if let colors::Error::ParseError(component) = why {
                        println!("Unable to parse {} component for '{}'", component, arg);
                    } else {
                        println!("Unknown error while parsing '{}'", arg);
                    }
                }
            }
        } else if colors::is_hex(&arg) {
            match RGBColor::from_hex(&arg) {
                Ok(color) => {
                    colors.push(color);
                }
                Err(why) => {
                    if let colors::Error::ParseError(component) = why {
                        println!("Unable to parse {} component for '{}'", component, arg);
                    } else {
                        println!("Unknown error while parsing '{}'", arg);
                    }
                }
            }
        } else {
            println!("Unknown format '{}'", &arg);
        }
    }

    match mode {
        0 => color_info(colors),
        1 => println!("Shades"),
        2 => complementary_colors(colors),
        _ => {}
    }
}

fn send_help() {
    println!(
        "  \
        Colou-rs - A terminal color utility\n  \
        Usage\n    \
        color-rs [] - Display"
    )
}

fn color_info(colors: Vec<RGBColor>) {
    for color in colors {
        let hsv = colors::hsv::HSVColor::from(color);
        let hsl = colors::hsl::HSLColor::from(color);

        let mut ansi = color.to_ansi_background();
        ansi.foreground(if color.is_light() {
            RGBColor::new(0, 0, 0)
        } else {
            RGBColor::new(255, 255, 255)
        });

        println!(
            "{}\nHEX: {}\nRGB: {}\nHSV: {}\nHSL: {}",
            ansi.paint("Colou-rs"),
            color.to_hex(),
            color.to_rgb(", "),
            hsv.to_str(),
            hsl.to_str(),
        );
    }
}

fn complementary_colors(colors: Vec<RGBColor>) {
    for color in colors {
        let mut ul = colors::ansi::ColorAnsi::new(None, None);
        ul.underline(true);

        let mut ulb = colors::ansi::ColorAnsi::new(None, None);
        ulb.underline(true);
        ulb.bold(true);

        let triad = color.triad();
        let tetradic = color.tetradic();
        let anal = color.analogous();
        let s_comp = color.split_complemetary();

        println!(
            "  {}\
            \n ╭───────────╮ ╭───────────╮\
            \n │   {}   │ │  {} │\
            \n │  {}  │ │  {}  │\
            \n │  {}  │ │  {}  │\
            \n │  {}  │ │  {}  │\
            \n ╰───────────╯ │  {}  │\
            \n ╭───────────╮ ╰───────────╯\
            \n │ {} │ ╭───────────╮\
            \n │  {}  │ │   {}  │\
            \n │  {}  │ │  {}  │\
            \n │  {}  │ │  {}  │\
            \n ╰───────────╯ │  {}  │\
            \n ╭───────────╮ │  {}  │\
            \n │   {}   │ │  {}  │\
            \n │  {}  │ │  {}  │\
            \n │  {}  │ │  {}  │\
            \n │  {}  │ │  {}  │\
            \n ╰───────────╯ ╰───────────╯",
            ulb.paint(format!("Complementary color for {}", color.to_hex())),
            ul.paint("Triad"),     ul.paint("Tetradic"),
            triad[0].paint_hex(),  tetradic[0].paint_hex(),
            triad[1].paint_hex(),  tetradic[1].paint_hex(),
            triad[2].paint_hex(),  tetradic[2].paint_hex(),
                                   tetradic[3].paint_hex(),
            ul.paint("Analogous"), 
            anal[0].paint_hex(),   ul.paint("Shades"),
            anal[1].paint_hex(),   anal[0].paint_hex(),
            anal[2].paint_hex(),   anal[0].paint_hex(),
                                   anal[0].paint_hex(),
            ul.paint("Split"),     anal[0].paint_hex(),
            s_comp[0].paint_hex(), anal[0].paint_hex(),
            s_comp[1].paint_hex(), anal[0].paint_hex(),
            s_comp[2].paint_hex(), anal[0].paint_hex(),
        );
    }
}
