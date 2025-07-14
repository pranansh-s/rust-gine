use crate::utility::Color::Color;
use crate::ui::attribute::Attribute;

trait UiComponent {
    fn render(&self);
}

pub struct UiTransform {
    pub top: f32,
    pub left: f32,
    pub width: f32,
    pub height: f32,
    pub background_color: Color
}

pub struct Div<'parent> {
    pub id: u32,
    pub name: String,
    pub transform: UiTransform,
    pub parent: &'parent Div<'parent>,
}

impl<'parent> Div<'parent> {
    pub fn new(id: u32, name: &str, attrbs: Vec<(&str, Attribute)>) -> Self {
        let mut transform: UiTransform = UiTransform {
            top: 0.0,
            left: 0.0,
            width: 0.0,
            height: 0.0,
            background_color: Color(0.0, 0.0, 0.0, 1.0)
        };

        for (attrb_name, attrb_value) in attrbs {
            match attrb_name {
                "top" => transform.top = attrb_value.as_float(),
                "left" => transform.left = attrb_value.as_float(),
                "width" => transform.width = attrb_value.as_float(),
                "height" => transform.height = attrb_value.as_float(),
                "background-color" => transform.background_color = attrb_value.as_color(),
                _ => eprintln!("Unrecognized attribute name in {} code:ui_component.rs", name)
            }
        }

        Self {
            id,
            name: name.to_string(),
            transform,
            parent: Vec::new()
        }
    }

    pub fn add_child(&mut self, child: Div) {
        self.children.push(child);
    }
}