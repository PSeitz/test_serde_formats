use serde::{de::DeserializeOwned, Serialize};

use crate::Deser;

use anyhow::Result;

pub struct Json;
impl Deser for Json {
    type Serialized = String;
    fn name() -> String {
        "Json".to_string()
    }
    fn serialize<T: Serialize>(t: &T) -> Result<(usize, Self::Serialized)> {
        let output: String = serde_json::to_string(t)?;
        Ok((output.len(), output))
    }
    fn deserialize<T: DeserializeOwned>(s: Self::Serialized) -> Result<T> {
        let deser: T = serde_json::from_str(&s)?;
        Ok(deser)
    }
}

pub struct Postcard;
impl Deser for Postcard {
    type Serialized = Vec<u8>;
    fn name() -> String {
        "Postcard".to_string()
    }
    fn serialize<T: Serialize>(t: &T) -> Result<(usize, Self::Serialized)> {
        let output: Vec<u8> = postcard::to_allocvec(t)?;
        Ok((output.len(), output))
    }
    fn deserialize<T: DeserializeOwned>(s: Self::Serialized) -> Result<T> {
        let deser: T = postcard::from_bytes(&s)?;
        Ok(deser)
    }
}

pub struct Ron;
impl Deser for Ron {
    type Serialized = String;
    fn name() -> String {
        "RON".to_string()
    }
    fn serialize<T: Serialize>(t: &T) -> Result<(usize, Self::Serialized)> {
        let output: String = ron::to_string(t)?;
        Ok((output.len(), output))
    }
    fn deserialize<T: DeserializeOwned>(s: Self::Serialized) -> Result<T> {
        let deser: T = ron::from_str(&s)?;
        Ok(deser)
    }
}

pub struct Rmp;
impl Deser for Rmp {
    type Serialized = Vec<u8>;
    fn name() -> String {
        "MessagePack".to_string()
    }
    fn serialize<T: Serialize>(t: &T) -> Result<(usize, Self::Serialized)> {
        let output: Vec<u8> = rmp_serde::to_vec(t)?;
        Ok((output.len(), output))
    }
    fn deserialize<T: DeserializeOwned>(s: Self::Serialized) -> Result<T> {
        let deser: T = rmp_serde::from_slice(&s)?;
        Ok(deser)
    }
}

pub struct Bincode;
impl Deser for Bincode {
    type Serialized = Vec<u8>;
    fn name() -> String {
        "Bincode".to_string()
    }
    fn serialize<T: Serialize>(t: &T) -> Result<(usize, Self::Serialized)> {
        let output = bincode::serialize(t)?;
        Ok((output.len(), output))
    }
    fn deserialize<T: DeserializeOwned>(s: Self::Serialized) -> Result<T> {
        let deser: T = bincode::deserialize(&s)?;
        Ok(deser)
    }
}

pub struct Ciborium;
impl Deser for Ciborium {
    type Serialized = Vec<u8>;
    fn name() -> String {
        "Ciborium".to_string()
    }
    fn serialize<T: Serialize>(t: &T) -> Result<(usize, Self::Serialized)> {
        let mut output: Vec<u8> = Vec::new();
        ciborium::ser::into_writer(t, &mut output).unwrap();
        Ok((output.len(), output))
    }
    fn deserialize<T: DeserializeOwned>(s: Self::Serialized) -> Result<T> {
        let deser: T = ciborium::de::from_reader(&mut s.as_slice())?;
        Ok(deser)
    }
}

pub struct Bson;
impl Deser for Bson {
    type Serialized = bson::Bson;
    fn name() -> String {
        "BSON".to_string()
    }
    fn serialize<T: Serialize>(t: &T) -> Result<(usize, Self::Serialized)> {
        let output = bson::to_bson(t)?;
        Ok((0, output))
    }
    fn deserialize<T: DeserializeOwned>(s: Self::Serialized) -> Result<T> {
        let deser: T = bson::from_bson(s)?;
        Ok(deser)
    }
}

pub struct Bitcode;
impl Deser for Bitcode {
    type Serialized = Vec<u8>;
    fn name() -> String {
        "Bitcode".to_string()
    }
    fn serialize<T: Serialize>(t: &T) -> Result<(usize, Self::Serialized)> {
        let output: Vec<u8> = bitcode::serialize(t)?;
        Ok((output.len(), output))
    }
    fn deserialize<T: DeserializeOwned>(s: Self::Serialized) -> Result<T> {
        let deser: T = bitcode::deserialize(&s)?;
        Ok(deser)
    }
}
