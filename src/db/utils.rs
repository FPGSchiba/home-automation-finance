use bson::{oid::ObjectId, DateTime};
use serde::de::{self, Deserializer, SeqAccess, Visitor};
use serde::{ser::SerializeSeq, Deserialize, Serializer};
use std::fmt;

pub fn serialize_option_bson_datetime_as_rfc3339_string<S>(
    date: &Option<DateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(d) => bson::serde_helpers::serialize_bson_datetime_as_rfc3339_string(d, serializer),
        None => serializer.serialize_none(),
    }
}

pub fn serialize_vec_object_id_as_hex_string<S>(
    vec: &Vec<ObjectId>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(vec.len()))?;
    for oid in vec {
        seq.serialize_element(&oid.to_hex())?;
    }
    seq.end()
}

pub fn deserialize_option_bson_datetime_from_rfc3339_string<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    match opt {
        Some(s) => {
            let dt = DateTime::parse_rfc3339_str(&s).map_err(de::Error::custom)?;
            Ok(Some(dt))
        }
        None => Ok(None),
    }
}

pub fn deserialize_vec_object_id_from_hex_string<'de, D>(
    deserializer: D,
) -> Result<Vec<ObjectId>, D::Error>
where
    D: Deserializer<'de>,
{
    struct VecObjectIdVisitor;

    impl<'de> Visitor<'de> for VecObjectIdVisitor {
        type Value = Vec<ObjectId>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a sequence of hex strings representing ObjectIds")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(hex) = seq.next_element::<String>()? {
                let oid = ObjectId::parse_str(&hex).map_err(de::Error::custom)?;
                vec.push(oid);
            }
            Ok(vec)
        }
    }

    deserializer.deserialize_seq(VecObjectIdVisitor)
}
pub fn deserialize_bson_datetime_from_rfc3339_string<'de, D>(
    deserializer: D,
) -> Result<DateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    DateTime::parse_rfc3339_str(&s).map_err(de::Error::custom)
}
