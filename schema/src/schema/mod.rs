//! `schema!` macro.
//!
//! TODO: next steps:
//! - support pub/pub(crate) and additional #[derive(...)] statements
//! - anything can have an advanced representation, so add support to all types
mod primitive;
mod recursive;
mod typedef;

/// Defines a native type with a standard IPLD Schema and Representation.
///
/// ```edition2018
/// # use libipld_schema;
/// ```
#[macro_export]
macro_rules! schema {
    ($($schema:tt)+) => {
        typedef!($($schema)*);
    };
}

#[cfg(test)]
mod tests {
    use crate::*;

    //////////////////////////////////////////////////////////////////////////
    // IPLD data types
    //////////////////////////////////////////////////////////////////////////

    // primitive types
    schema!(type Null null);
    schema!(type Bool bool);
    schema!(type Int8 i8);
    schema!(type Int16 i16);
    schema!(type Int32 i32);
    schema!(type Int64 i64);
    schema!(type Uint8 u8);
    schema!(type Uint16 u16);
    schema!(type Uint32 u32);
    schema!(type Uint64 u64);
    schema!(type Float32 f32);
    schema!(type Float64 f64);
    schema!(type TString String);
    schema!(type Bytes1 bytes);
    schema!(type BytesCopy = Bytes1);

    // recursive types
    schema!(type StringLink Link<String>);
    schema!(type List [TString]);
    schema!(type Map {String: List});

    //////////////////////////////////////////////////////////////////////////
    // IPLD schema types and representations
    //////////////////////////////////////////////////////////////////////////

    // map
    schema!(type MapMap {String: List} representation map);
    schema!(type MapStringpairs {String: List} representation stringpairs {
        innerDelim: ":",
        entryDelim: ","
    });
    schema!(type MapListpairs {String: List} representation listpairs);

    // struct
    // schema!(type Struct struct {});
    // schema!(type StructMap struct {} representation map);
    // schema!(type StructTuple struct {} representation tuple);
    // schema!(type StructStringpairs struct {} representation stringpairs);
    // schema!(type StructStringjoin struct {} representation stringjoin);
    // schema!(type StructListpairs struct {} representation listpairs);

    // enum
    // schema!(type Enum enum {});
    // schema!(type EnumString enum {} representation string);
    // schema!(type EnumInt enum {} representation int);

    // union
    // schema!(type Union union {});
    // schema!(type UnionKeyed union {} representation keyed);
    // schema!(type UnionKinded union {} representation kinded);
    // schema!(type UnionEnvelope union {} representation envelope);
    // schema!(type UnionInline union {} representation inline);
    // schema!(type UnionByteprefix union {} representation byteprefix);
}
