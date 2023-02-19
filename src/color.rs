use alacritty_terminal::{
    ansi::{Color, NamedColor},
    term::color::Rgb,
};

pub(crate) fn named_to_rgb(named: NamedColor) -> Rgb {
    match named {
        NamedColor::Black => Rgb {
            r: 0x1d,
            g: 0x1f,
            b: 0x21,
        },
        NamedColor::Red => Rgb {
            r: 0xcc,
            g: 0x66,
            b: 0x66,
        },
        NamedColor::Green => Rgb {
            r: 0xb5,
            g: 0xbd,
            b: 0x68,
        },
        NamedColor::Yellow => Rgb {
            r: 0xf0,
            g: 0xc6,
            b: 0x74,
        },
        NamedColor::Blue => Rgb {
            r: 0x81,
            g: 0xa2,
            b: 0xbe,
        },
        NamedColor::Magenta => Rgb {
            r: 0xb2,
            g: 0x94,
            b: 0xbb,
        },
        NamedColor::Cyan => Rgb {
            r: 0x8a,
            g: 0xbe,
            b: 0xb7,
        },
        NamedColor::White => Rgb {
            r: 0xc5,
            g: 0xc8,
            b: 0xc6,
        },
        NamedColor::BrightBlack => Rgb {
            r: 0x66,
            g: 0x66,
            b: 0x66,
        },
        NamedColor::BrightRed => Rgb {
            r: 0xd5,
            g: 0x4e,
            b: 0x53,
        },
        NamedColor::BrightGreen => Rgb {
            r: 0xb9,
            g: 0xca,
            b: 0x4a,
        },
        NamedColor::BrightYellow => Rgb {
            r: 0xe7,
            g: 0xc5,
            b: 0x47,
        },
        NamedColor::BrightBlue => Rgb {
            r: 0x7a,
            g: 0xa6,
            b: 0xda,
        },
        NamedColor::BrightMagenta => Rgb {
            r: 0xc3,
            g: 0x97,
            b: 0xd8,
        },
        NamedColor::BrightCyan => Rgb {
            r: 0x70,
            g: 0xc0,
            b: 0xb1,
        },
        NamedColor::BrightWhite => Rgb {
            r: 0xea,
            g: 0xea,
            b: 0xea,
        },
        NamedColor::Foreground => Rgb {
            r: 0xc5,
            g: 0xc8,
            b: 0xc6,
        },
        NamedColor::Background => Rgb {
            r: 0x1d,
            g: 0x1f,
            b: 0x21,
        },
        NamedColor::Cursor => Rgb {
            r: 0x3d,
            g: 0x3f,
            b: 0x41,
        },
        NamedColor::DimBlack => Rgb {
            r: 0x13,
            g: 0x14,
            b: 0x15,
        },
        NamedColor::DimRed => Rgb {
            r: 0x86,
            g: 0x43,
            b: 0x43,
        },
        NamedColor::DimGreen => Rgb {
            r: 0x77,
            g: 0x7c,
            b: 0x44,
        },
        NamedColor::DimYellow => Rgb {
            r: 0x9e,
            g: 0x82,
            b: 0x4c,
        },
        NamedColor::DimBlue => Rgb {
            r: 0x55,
            g: 0x6a,
            b: 0x7d,
        },
        NamedColor::DimMagenta => Rgb {
            r: 0x75,
            g: 0x61,
            b: 0x7b,
        },
        NamedColor::DimCyan => Rgb {
            r: 0x5b,
            g: 0x7d,
            b: 0x78,
        },
        NamedColor::DimWhite => Rgb {
            r: 0x82,
            g: 0x84,
            b: 0x82,
        },
        NamedColor::BrightForeground => Rgb {
            r: 0xc5,
            g: 0xc8,
            b: 0xc6,
        },
        NamedColor::DimForeground => Rgb {
            r: 0xc5,
            g: 0xc8,
            b: 0xc6,
        },
    }
}

pub(crate) fn to_string(color: Color) -> String {
    match color {
        Color::Spec(rgb) => rgb.to_string(),
        Color::Named(name) => named_to_rgb(name).to_string(),
        Color::Indexed(index) => {
            let ansi_colors = vec![
                NamedColor::Black,
                NamedColor::Red,
                NamedColor::Green,
                NamedColor::Yellow,
                NamedColor::Blue,
                NamedColor::Magenta,
                NamedColor::Cyan,
                NamedColor::White,
                NamedColor::BrightBlack,
                NamedColor::BrightRed,
                NamedColor::BrightGreen,
                NamedColor::BrightYellow,
                NamedColor::BrightBlue,
                NamedColor::BrightMagenta,
                NamedColor::BrightCyan,
                NamedColor::BrightWhite,
            ]
            .into_iter()
            .map(|named| named_to_rgb(named).to_string())
            .collect::<Vec<_>>();
            match index {
                0..=15 => ansi_colors[index as usize].clone(),
                16..=231 => {
                    let index = index - 16;
                    let r = ((index / (6 * 6)) % 6) * 40 + 55;
                    let g = ((index / 6) % 6) * 40 + 45;
                    let b = (index % 6) * 40 + 45;
                    format!("#{r:X}{g:X}{b:X}")
                }
                232..=255 => {
                    let gray_level = index - 233;
                    let gray_value = gray_level * 10 + 8;
                    format!("#{0:X}{0:X}{0:X}", gray_value)
                }
            }
        }
    }
}
