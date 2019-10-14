use ansi_term::Style;
use std::fmt;

use serde::de::{self, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::Deserialize;

#[derive(Debug)]
struct SegmentConfig<'a> {
    value: &'a str,
    style: Option<Style>,
}

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

            fn visit_seq<V>(self, mut seq: V) -> Result<SegmentConfig<'de>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let value = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let style = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(SegmentConfig { value, style })
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
                            style = Some(map.next_value()?);
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

        const FIELDS: &'static [&'static str] = &["value", "style"];
        deserializer.deserialize_struct("SegmentConfig", FIELDS, SegmentConfigVisitor)
    }
}
