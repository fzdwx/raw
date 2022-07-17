use crossterm::style::Color;

#[derive(PartialEq)]
pub enum Type {
    None,
    Number,
    Match,
    String,
    Character,
    Comment,
}

impl Type {
    pub fn to_color(&self) -> Color {
        match self {
            Type::Number => Color::Rgb {
                r: 192,
                g: 232,
                b: 127,
            },
            Type::Match => Color::Rgb {
                r: 38,
                g: 139,
                b: 210,
            },
            Type::String => Color::Rgb {
                r: 211,
                g: 54,
                b: 130,
            },
            Type::Character => Color::Rgb {
                r: 108,
                g: 113,
                b: 196,
            },
            Type::Comment => Color::Rgb {
                r: 133,
                g: 153,
                b: 0,
            },
            _ => Color::Reset,
        }
    }
}