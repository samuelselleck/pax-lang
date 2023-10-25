use crate::types::Color;
use pax_lang::api::{Numeric, Property, PropertyLiteral};
use pax_lang::*;
use pax_message::{
    ColorVariantMessage, CheckboxStyleMessage,
};

#[derive(Pax)]
#[custom(Default)]
pub struct CheckboxStyle {
    pub background: Property<Color>,
    pub border: Property<Color>,
    pub color: Property<Color>,
    //Maybe later
    //pub border_width: Property<SizePixels>,
    //pub corner_radii: Property<CornerRadii>,
}

impl Default for CheckboxStyle {
    fn default() -> Self {
        Self {
            background: Box::new(PropertyLiteral::new(Color::rgba(
                Numeric::Float(1.0),
                Numeric::Float(1.0),
                Numeric::Float(1.0),
                Numeric::Float(1.0),
            ))),
            border: Box::new(PropertyLiteral::new(Color::rgba(
                Numeric::Float(0.0),
                Numeric::Float(0.0),
                Numeric::Float(0.0),
                Numeric::Float(1.0),
            ))),
            color: Box::new(PropertyLiteral::new(Color::rgba(
                Numeric::Float(0.3),
                Numeric::Float(0.3),
                Numeric::Float(1.0),
                Numeric::Float(1.0),
            ))),
        }
    }
}

impl From<CheckboxStyle> for CheckboxStyleMessage {
    fn from(cs: CheckboxStyle) -> Self {
        CheckboxStyleMessage {
            background: Some(Into::<ColorVariantMessage>::into(cs.background.get())),
            border: Some(Into::<ColorVariantMessage>::into(cs.border.get())),
            color: Some(Into::<ColorVariantMessage>::into(cs.color.get())),
        }
    }
}

impl PartialEq<CheckboxStyleMessage> for CheckboxStyle {
    fn eq(&self, other: &CheckboxStyleMessage) -> bool {
        let background_equal = other
        .background
        .as_ref()
        .map_or(false, |c| self.background.get().eq(c));
        
        let border_equal = other
        .border
        .as_ref()
        .map_or(false, |c| self.border.get().eq(c)); 

        let color_equal = other
        .color
        .as_ref()
        .map_or(false, |c| self.color.get().eq(c)); 

        background_equal && border_equal && color_equal
    }
}
