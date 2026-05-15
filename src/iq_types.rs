use std::fmt;
use std::str::FromStr;

use clap::{
    ValueEnum,
    builder::{PossibleValue, ValueParser},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum IQTypes {
    cf64le,
    cf32le,
    ci32le,
    cu32le,
    ci16le,
    cu16le,
    cu8,
    ci8,
}

impl fmt::Display for IQTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            IQTypes::cf64le => "cf64_le",
            IQTypes::cf32le => "cf32_le",
            IQTypes::ci32le => "ci32_le",
            IQTypes::cu32le => "cu32_le",
            IQTypes::ci16le => "ci16_le",
            IQTypes::cu16le => "cu16_le",
            IQTypes::cu8 => "cu8",
            IQTypes::ci8 => "ci8",
        };

        write!(f, "{s}")
    }
}

impl From<IQTypes> for String {
    fn from(value: IQTypes) -> Self {
        value.to_string()
    }
}

impl FromStr for IQTypes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "cf64_le" => Ok(IQTypes::cf64le),
            "cf32_le" => Ok(IQTypes::cf32le),
            "ci32_le" => Ok(IQTypes::ci32le),
            "cu32_le" => Ok(IQTypes::cu32le),
            "ci16_le" => Ok(IQTypes::ci16le),
            "cu16_le" => Ok(IQTypes::cu16le),
            "cu8" => Ok(IQTypes::cu8),
            "ci8" => Ok(IQTypes::ci8),
            _ => Err(format!("Ungültiger IQ type: {s}")),
        }
    }
}

impl ValueEnum for IQTypes {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            IQTypes::cf64le,
            IQTypes::cf32le,
            IQTypes::ci32le,
            IQTypes::cu32le,
            IQTypes::ci16le,
            IQTypes::cu16le,
            IQTypes::cu8,
            IQTypes::ci8,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            IQTypes::cf64le => PossibleValue::new("cf64_le"),
            IQTypes::cf32le => PossibleValue::new("cf32_le"),
            IQTypes::ci32le => PossibleValue::new("ci32_le"),
            IQTypes::cu32le => PossibleValue::new("cu32_le"),
            IQTypes::ci16le => PossibleValue::new("ci16_le"),
            IQTypes::cu16le => PossibleValue::new("cu16_le"),
            IQTypes::cu8 => PossibleValue::new("cu8"),
            IQTypes::ci8 => PossibleValue::new("ci8"),
        })
    }
}

impl clap::builder::ValueParserFactory for IQTypes {
    type Parser = ValueParser;

    fn value_parser() -> Self::Parser {
        clap::builder::EnumValueParser::<IQTypes>::new().into()
    }
}
