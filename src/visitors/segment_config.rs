use ansi_term::Style;
use std::fmt;

use super::style::StyleDef;

use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::Deserialize;

#[derive(Debug)]
struct SegmentConfig<'a> {
    value: &'a str,
    style: Option<Style>,
}

#[derive(Deserialize)]
struct StyleDefWrapper(#[serde(with = "StyleDef")] Option<Style>);

impl<'de: 'a, 'a> Deserialize<'de> for SegmentConfig<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Value,
            Style,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`value` or `style`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "value" => Ok(Field::Value),
                            "style" => Ok(Field::Style),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct SegmentConfigVisitor;

        impl<'de> Visitor<'de> for SegmentConfigVisitor {
            type Value = SegmentConfig<'de>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct SegmentConfig")
            }

            fn visit_map<V>(self, mut map: V) -> Result<SegmentConfig<'de>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut value = None;
                let mut style = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Value => {
                            if value.is_some() {
                                return Err(de::Error::duplicate_field("value"));
                            }
                            value = Some(map.next_value()?);
                        }
                        Field::Style => {
                            if style.is_some() {
                                return Err(de::Error::duplicate_field("style"));
                            }
                            let StyleDefWrapper(style_def) = map.next_value()?;
                            style = Some(style_def);
                        }
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                let style = style.ok_or_else(|| de::Error::missing_field("style"))?;

                Ok(SegmentConfig { value, style })
            }

            fn visit_borrowed_str<E>(self, value: &'de str) -> Result<SegmentConfig<'de>, E>
            where
                E: de::Error,
            {
                Ok(SegmentConfig { value, style: None })
            }
        }

        const FIELDS: &[&str] = &["value", "style"];
        deserializer.deserialize_struct("SegmentConfig", FIELDS, SegmentConfigVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ansi_term::Color;

    #[derive(Deserialize)]
    struct Config<'a> {
        #[serde(borrow)]
        symbol: SegmentConfig<'a>,
    }

    #[test]
    fn test_load_string() {
        let cfg: Config = toml::from_str(
            r#"
        symbol = "S "
        "#,
        )
        .unwrap();

        assert_eq!(cfg.symbol.style, None);
        assert_eq!(cfg.symbol.value, "S ");
    }

    #[test]
    fn test_load_struct() {
        #[derive(Deserialize)]
        struct Config<'a> {
            #[serde(borrow)]
            symbol: SegmentConfig<'a>,
        }

        let cfg: Config = toml::from_str(
            r#"
        symbol = { value = "S ", style = "red bold" }
        "#,
        )
        .unwrap();

        assert_eq!(cfg.symbol.style, Some(Color::Red.bold()));
        assert_eq!(cfg.symbol.value, "S ");
    }
}
