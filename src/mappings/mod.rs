use std::fmt::Display;

use clap::ValueEnum;
use r34::R34;

pub mod r34;

#[derive(Debug, Clone, Default, ValueEnum)]
/// Possible mapping types
pub enum MappingType {
    #[default]
    R34,
}

impl Display for MappingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MappingType::R34 => write!(f, "r34"),
        }
    }
}

impl MappingType {
    /// Convert sequence of characters to binary url sequence
    pub fn to_url<'a>(
        &self,
        input: impl Iterator<Item = char> + 'a,
    ) -> impl Iterator<Item = &'static str> + 'a {
        match self {
            MappingType::R34 => Box::new(
                input
                    .flat_map(|ch| {
                        let byte_value = ch as u8;
                        let binary_string = format!("{:08b}", byte_value);
                        binary_string.chars().collect::<Vec<_>>()
                    })
                    .map(|binary_digit| {
                        let digit = binary_digit.to_digit(10).unwrap() as u8;
                        *R34.get(&digit).unwrap()
                    }),
            ),
        }
    }
}

/// Convert sequence of mappings to html url S
pub fn to_html<'a>(input: impl Iterator<Item = &'static str> + 'a) -> String {
    let mut html = String::with_capacity(512);

    html.push_str(r#"<div align="center" style="width:100%">"#);

    for url in input {
        html.push_str(format!(r#"<img src="{}"/>"#, url).as_str());
    }
    html.push_str("</div>");

    html
}
