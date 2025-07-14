use crate::utility::Color::Color;

pub enum Attribute {
    Percent(f32),
    Float(f32),
    Color(Color),
}

impl Attribute {
    pub fn as_float(&self) -> f32 {
        match self {
            Attribute::Percent(value) => *value,
            Attribute::Float(value) => *value,
            Attribute::Color(_) => 0.0,
        }
    }

    pub fn as_color(&self) -> Color {
        match self {
            Attribute::Color(color) => color.clone(),
            _ => Color(0.0, 0.0, 0.0, 1.0),
        }
    }
}