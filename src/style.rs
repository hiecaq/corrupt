//! defines how a match should highlight

use crossterm::style::{Attribute, Color};
use ron::de::from_str as ron_from_str;
use serde::{de::Deserializer, Deserialize};

use crate::error::Error;

#[derive(Debug, Deserialize)]
pub enum Style {
    /// a foreground color
    Fg(Color),
    /// a background color
    Bg(Color),
    /// an attribute, such as underlined
    Attr(Attribute),
}

/// a newtype that defines how a match should highlight
#[derive(Debug)]
pub struct StyleBundle(pub(crate) Vec<Style>);

impl<'de> Deserialize<'de> for StyleBundle {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Vec::deserialize(deserializer).map(|v| StyleBundle(v))
    }
}

impl std::str::FromStr for StyleBundle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ron_from_str(s)?)
    }
}
