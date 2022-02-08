use serde::{de, de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
pub struct Snowflake(u64);

impl Serialize for Snowflake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

struct SnowflakeVisitor;

impl<'de> Visitor<'de> for SnowflakeVisitor {
    type Value = Snowflake;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("snowflake as a u64 or string")
    }

    fn visit_u64<E>(self, id: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Snowflake(id))
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        s.parse().map(Snowflake).map_err(de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(SnowflakeVisitor)
    }
}
