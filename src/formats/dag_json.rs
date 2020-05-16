//! IPLD DagJson codec.

use crate::dev::*;
use delegate::delegate;
use serde::{de, ser};
#[cfg(not(feature = "simd"))]
use serde_json::{
    de::Read as JsonRead, from_reader, from_slice, to_writer, Deserializer as JsonDeserializer,
    Error as JsonError, Serializer as JsonSerializer,
};
#[cfg(feature = "simd")]
use simd_json::{};
use std::{
    fmt,
    io::{Read, Write},
};

const MULTIBASE: Multibase = Multibase::Base64;

// TODO: add support for simd-json
#[cfg(not(feature = "simd"))]
/// The DagJSON codec, that delegates to `serde_json`.
pub struct DagJson;

impl Format for DagJson {
    const VERSION: cid::Version = cid::Version::V1;
    const CODEC: cid::Codec = cid::Codec::DagJSON;

    // type Encoder<W> = JsonSerializer<W>;
    // type Decoder<R> = JsonDeserializer<R>;

    // fn encoder<W: Write>(writer: W) -> Self::Encoder<W> {
    //     unimplemented!()
    // }
    // fn decoder<'de, R: Read>(reader: R) -> Self::Decoder<R> {
    //     unimplemented!()
    // }

    // type Error = JsonError;

    fn encode<T, W>(dag: &T, writer: W) -> Result<(), Error>
    where
        T: Representation + Serialize,
        W: Write,
    {
        to_writer(writer, dag).map_err(|e| Error::Encoder(anyhow::Error::new(e)))
    }

    fn decode<'de, T>(bytes: &'de [u8]) -> Result<T, Error>
    where
        T: Representation + Deserialize<'de>,
    {
        from_slice(bytes).map_err(|e| Error::Decoder(anyhow::Error::new(e)))
    }

    fn read<T, R>(reader: R) -> Result<T, Error>
    where
        T: Representation + for<'de> Deserialize<'de>,
        R: Read,
    {
        from_reader(reader).map_err(|e| Error::Decoder(anyhow::Error::new(e)))
    }
}

impl<'a, W: Write> Encoder for &'a mut JsonSerializer<W> {
    /// Serializes bytes as a struct variant, e.g.
    /// `{ "/": { "base64": <some base64-encoded string> } }`.
    #[inline]
    fn serialize_bytes(self, bytes: &[u8]) -> Result<Self::Ok, JsonError> {
        use ser::SerializeStructVariant as SV;

        let mut sv = self.serialize_struct_variant("", 0, "/", 1)?;
        SV::serialize_field(&mut sv, "base64", &multibase::encode(MULTIBASE, bytes))?;
        SV::end(sv)
    }

    /// Serializes links as a newtype variant, e.g.  `{ "/": "Qm..." }`.
    #[inline]
    fn serialize_link(self, cid: &Cid) -> Result<Self::Ok, JsonError> {
        self.serialize_newtype_variant("", 0, "/", &cid.to_string())
    }
}

impl<'de, 'a, R: JsonRead<'de>> Decoder<'de> for &'a mut JsonDeserializer<R> {
    /// In the DagJSON IPLD codec, three IPLD types map to the map as represented
    /// in the Serde data model:
    ///     - maps
    ///     - links, e.g. `{ "/": "Qm..." }`
    ///     - byte sequences, e.g. `{ "/": { "base64": <some base64-encoded string> } }`
    ///
    /// This method wraps the provided `Visitor`, delegating the visiting of all
    /// types found in the input data to the provided `Visitor` (except for maps,
    /// which are handled separately as they may be IPLD bytes, links or actual maps.
    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: IpldVisitorExt<'de>,
    {
        Deserializer::deserialize_any(self, JsonVisitor(visitor))
    }

    #[inline]
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: IpldVisitorExt<'de>,
    {
        Decoder::deserialize_byte_buf(self, visitor)
    }

    #[inline]
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: IpldVisitorExt<'de>,
    {
        self.deserialize_map(JsonVisitor(visitor))
    }

    #[inline]
    fn deserialize_link<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: IpldVisitorExt<'de>,
    {
        self.deserialize_map(JsonVisitor(visitor))
    }
}

/// `JsonVisitor` wraps an "any" type `Visitor` in order to enhance how maps are
/// deserialized.
struct JsonVisitor<V>(V);

impl<'de, V: IpldVisitorExt<'de>> Visitor<'de> for JsonVisitor<V> {
    type Value = V::Value;

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a JSON map, link object or byte object")
    }

    /// Called when a map is found in the input data.
    /// TODO: test that this works with links union-inlined into structs/maps
    #[inline]
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let first_key: Option<String> = map.next_key()?;
        if let Some("/") = first_key.as_deref() {
            match map.next_value::<MapLikeVisitor>() {
                Ok(MapLikeVisitor::Bytes(b)) => self.0.visit_byte_buf(b),
                Ok(MapLikeVisitor::Cid(cid)) => self.0.visit_link(cid),
                _ => Err(de::Error::custom("expected a CID or byte string")),
            }
        } else {
            self.0.visit_map(MapAccessor { first_key, map })
        }
    }

    // Some of these are not expected to be called, since the only data model
    // mis-match exists between Serde maps and IPLD maps.
    delegate! {
        to self.0 {
            fn visit_bool<E: de::Error>(self, v: bool) -> Result<Self::Value, E>;
            fn visit_i8<E: de::Error>(self, v: i8) -> Result<Self::Value, E>;
            fn visit_i16<E: de::Error>(self, v: i16) -> Result<Self::Value, E>;
            fn visit_i32<E: de::Error>(self, v: i32) -> Result<Self::Value, E>;
            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E>;
            fn visit_i128<E: de::Error>(self, v: i128) -> Result<Self::Value, E>;
            fn visit_u8<E: de::Error>(self, v: u8) -> Result<Self::Value, E>;
            fn visit_u16<E: de::Error>(self, v: u16) -> Result<Self::Value, E>;
            fn visit_u32<E: de::Error>(self, v: u32) -> Result<Self::Value, E>;
            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E>;
            fn visit_u128<E: de::Error>(self, v: u128) -> Result<Self::Value, E>;
            fn visit_f32<E: de::Error>(self, v: f32) -> Result<Self::Value, E>;
            fn visit_f64<E: de::Error>(self, v: f64) -> Result<Self::Value, E>;
            fn visit_char<E: de::Error>(self, v: char) -> Result<Self::Value, E>;
            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E>;
            fn visit_borrowed_str<E: de::Error>(self, v: &'de str) -> Result<Self::Value, E>;
            fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E>;
            fn visit_bytes<E: de::Error>(self, v: &[u8]) -> Result<Self::Value, E>;
            fn visit_borrowed_bytes<E: de::Error>(self, v: &'de [u8]) -> Result<Self::Value, E>;
            fn visit_byte_buf<E: de::Error>(self, v: Vec<u8>) -> Result<Self::Value, E>;
            fn visit_none<E: de::Error>(self) -> Result<Self::Value, E>;
            fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error>;
            fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E>;
            fn visit_newtype_struct<D: Deserializer<'de>>(
                self,
                deserializer: D
            ) -> Result<Self::Value, D::Error>;
            fn visit_seq<A: de::SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error>;
            fn visit_enum<A: de::EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error>;
        }
    }
}

/// Visits the IPLD types in DagJSON that are map-like (i.e. bytes and links).
enum MapLikeVisitor {
    Default,
    Bytes(Vec<u8>),
    Cid(Cid),
}

impl<'de> Deserialize<'de> for MapLikeVisitor {
    /// Will either deserialize a string (as a link), or a map (as bytes) -
    /// anything else is an error.
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MapLikeVisitor::Default)
    }
}

impl<'de> Visitor<'de> for MapLikeVisitor {
    type Value = Self;

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a JSON map, link object or byte object")
    }

    #[inline]
    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let cid = ToCid::to_cid(s).map_err(|_| de::Error::custom("expected a CID"))?;
        Ok(MapLikeVisitor::Cid(cid))
    }

    #[inline]
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let (base, s): (String, String) = map.next_entry()?.ok_or_else(|| {
            de::Error::custom("expected a multibase and multibase-encoded string")
        })?;

        if base.as_str() != "base64" {
            return Err(de::Error::custom(
                "DagJSON only supports base64-encoded strings",
            ));
        }

        let (mb, bytes) = multibase::decode(s)
            .map_err(|_| de::Error::custom("expected a base64 multibase-encoded string"))?;

        if Multibase::Base64.eq(&mb) {
            Ok(MapLikeVisitor::Bytes(bytes))
        } else {
            Err(de::Error::custom(
                "DagJSON only supports base64-encoded strings",
            ))
        }
    }
}

/// Wraps a `MapAccess` thats had it's first key removed.
struct MapAccessor<A> {
    first_key: Option<String>,
    map: A,
}

impl<'de, A: de::MapAccess<'de>> de::MapAccess<'de> for MapAccessor<A> {
    type Error = A::Error;

    #[inline]
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        use de::IntoDeserializer;

        if self.first_key.is_some() {
            let first_key = self.first_key.take().unwrap();
            seed.deserialize(first_key.into_deserializer()).map(Some)
        } else {
            self.map.next_key_seed(seed)
        }
    }

    delegate! {
        to self.map {
            fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
            where
                V: de::DeserializeSeed<'de>;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::_formats::test_utils;
    use crate::prelude::*;
    use std::fmt::Debug;

    fn test_str<'de, T>(cases: &[(T, &'de str)])
    where
        T: PartialEq + Debug + Representation + Serialize + Deserialize<'de>,
    {
        test_utils::test_str::<DagJson, T>(cases)
    }

    #[test]
    fn test_null() {
        let tests = &[((), "null")];
        test_str(tests);
        let tests = &[(None as Option<()>, "null")];
        test_str(tests);
    }

    #[test]
    fn test_bool() {
        let tests = &[(true, "true"), (false, "false")];
        test_str(tests);
    }

    #[test]
    fn test_number() {
        let tests = &[(123, "123")];
        test_str(tests);
        let tests = &[(123.123, "123.123")];
        test_str(tests);
    }

    #[test]
    fn test_string() {
        let dag = String::from("hello world");
        let tests = &[(dag, "\"hello world\"")];
        test_str(tests);
    }

    #[test]
    fn test_bytes() {
        let dag = Bytes::from(vec![0x01, 0x02, 0x03]);
        let tests = &[(dag.clone(), "{\"/\":{\"base64\":\"mAQID\"}}")];
        test_str(tests);
    }

    #[test]
    fn test_link() {
        // let tests = &[(
        //     Cid::from("QmdfTbBqBPQ7VNxZEYEj14VmRuZBkqFbiwReogJgS1zR1n"),
        //     "{\"/\":\"QmdfTbBqBPQ7VNxZEYEj14VmRuZBkqFbiwReogJgS1zR1n\"}",
        // )];
        // test_str(tests);
    }

    #[test]
    fn test_seq() {}

    #[test]
    fn test_map() {}
}
