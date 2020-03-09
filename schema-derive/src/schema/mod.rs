//! `schema!` macro.

#[macro_use]
mod r#enum;
#[macro_use]
mod primitive;
#[macro_use]
mod r#struct;
#[macro_use]
mod union;

/// Defines a native type with a standard IPLD Schema and Representation.
///
/// ```edition2018
/// # use libipld_schema;
/// ```
#[macro_export]
macro_rules! schema {
    //////////////////////////////////////////////////////////////////////////
    // IPLD Primitive Data Types
    //////////////////////////////////////////////////////////////////////////

    // Null
    (type $name:ident null) => {
        typedef_null!($name);
    };

    // Bool
    (type $name:ident bool) => {
        typedef_bool!($name);
    };

    // Integer
    (type $name:ident int) => {
        typedef_num!($name : i32);
    };
    (type $name:ident i8) => {
        typedef_num!($name : i8);
    };
    (type $name:ident i16) => {
        typedef_num!($name : i16);
    };
    (type $name:ident i32) => {
        typedef_num!($name : i32);
    };
    (type $name:ident i64) => {
        typedef_num!($name : i64);
    };
    (type $name:ident u8) => {
        typedef_num!($name : u8);
    };
    (type $name:ident u16) => {
        typedef_num!($name : u16);
    };
    (type $name:ident u32) => {
        typedef_num!($name : u32);
    };
    (type $name:ident u64) => {
        typedef_num!($name : u64);
    };

    // Float
    (type $name:ident float) => {
        typedef_num!($name : f64);
    };
    (type $name:ident f32) => {
        typedef_num!($name : f32);
    };
    (type $name:ident f64) => {
        typedef_num!($name : f64);
    };

    // String
    (type $name:ident String) => {
        typedef_str!($name);
    };

    // Bytes
    (type $name:ident bytes) => {
        typedef_bytes!($name);
    };

    // Copy
    // TODO: ? create a new struct that wraps the copied and delegates?
    (type $name:ident = $type:ty) => {
        type $name = $type;
    };

    //////////////////////////////////////////////////////////////////////////
    // IPLD Recursive Data Types
    //////////////////////////////////////////////////////////////////////////

    // Link
    (type $name:ident &$type:tt) => {
        typedef_link!($name $type);
    };

    // List
    (type $name:ident [ $elem_type:ty ]) => {
        typedef_list!($name $elem_type);
    };

    // Map
    (type $name:ident { $key:ty : $value:ty }) => {
        typedef_map!($name { $key: $value });
    };
    // basic map representation
    (type $name:ident { $key:ty : $value:ty } representation map) => {
        typedef_map!($name { $key: $value });
    };
    // TODO: stringpairs map representation
    (type $name:ident { $key:ty : $value:ty } representation stringpairs {
        innerDelim : $inner:expr,
        entryDelim : $entry:expr
    }) => {
        typedef_map!(@stringpairs $name { $key: $value } $inner, $entry);
    };
    // TODO: listpairs map representation
    (type $name:ident { $key:ty : $value:ty } representation listpairs) => {
        typedef_map!(@listpairs $name { $key: $value });
    };

    //////////////////////////////////////////////////////////////////////////
    // IPLD Schema Types
    //////////////////////////////////////////////////////////////////////////

    // TODO: Struct
    (type $name:ident struct { $($member:ident : $value_type:ty,)* }) => {
        typedef_struct!($name { $($member : $value_type)* });
    };
    // TODO: basic map representation
    (type $name:ident struct { $($member:ident : $value_type:ty,)* } representation map) => {
        typedef_struct!($name { $($member : $value_type)* });
    };
    // TODO: struct tuple representation
    (type $name:ident struct { $($member:ident : $value_type:ty,)* } representation tuple) => {
        typedef_struct!(@tuple $name { $($member : $value_type)* });
    };
    // TODO: struct stringpairs representation
    (type $name:ident struct { $($member:ident : $value_type:ty,)* } representation stringpairs {
        innerDelim : $inner:expr,
        entryDelim : $entry:expr
    }) => {
        typedef_struct!(@stringpairs $name { $($member : $value_type)* } $inner, $entry);
    };
    // TODO: struct stringjoin representation
    (type $name:ident struct { $($member:ident : $value_type:ty,)* } representation stringjoin {
        join : $joiner:expr
    }) => {
        typedef_struct!(@stringjoin $name { $($member : $value_type)* } $joiner);
    };
    // TODO: struct listpairs representation
    (type $name:ident struct { $($member:ident : $value_type:ty,)* } representation listpairs) => {
        typedef_struct!(@listpairs $name { $($member : $value_type)* });
    };

    // TODO: Enum
    (type $name:ident enum { $(| $member:ident,)* }) => {
        typedef_enum!($name { $($member)* });
    };
    // TODO: basic enum representation
    (type $name:ident enum { $(| $member:ident ($alias:expr),)* } representation string) => {
        typedef_enum!($name { $($member $alias)* });
    };
    // TODO: enum int representation
    (type $name:ident enum { $(| $member:ident, ($alias:expr),)* } representation int) => {
        typedef_enum!(@int $name { $($member $alias)* });
    };

    // TODO: Union
    (type $name:ident union { $(| $member:ident,)* }) => {
        typedef_union!($name { $($member)* });
    };
    // TODO: union keyed representation
    (type $name:ident union { $(| $member:ident $alias:expr,)* } representation keyed) => {
        typedef_union!(@keyed $name { $($member $alias)* });
    };
    // TODO: union kinded representation
    (type $name:ident union { $(| $member:ident,)* } representation kinded) => {
        typedef_union!(@kinded $name { $($member)* });
    };
    // TODO: union envelope representation
    (type $name:ident union { $(| $member:ident $alias:expr,)* } representation envelope {
        discriminantKey : $discriminant:expr,
        contentKey      : $content:expr
    }) => {
        typedef_union!(@envelope $name { $($member $alias)* } $discriminant, $content);
    };
    // TODO: union inline representation
    (type $name:ident union { $(| $member:ident $alias:expr,)* } representation inline {
        discriminantKey : $discriminant:expr
    }) => {
        typedef_union!(@inline $name { $($member $alias)* } $discriminant);
    };
    // TODO: union byteprefix representation
    (type $name:ident union { $(| $member:ident $prefix:expr,)* } representation byteprefix) => {
        typedef_union!(@byteprefix $name { $($member $prefix)* });
    };
}

#[cfg(test)]
mod tests {
    use crate::schema;

    //////////////////////////////////////////////////////////////////////////
    // IPLD data types
    //////////////////////////////////////////////////////////////////////////

    #[test]
    fn schema_macro() {
        // primitive types
        schema!(type Null null);
        schema!(type Bool bool);
        schema!(type Int int);
        //     schema!(type Int8 i8);
        //     schema!(type Int16 i16);
        schema!(type Int32 i32);
        //     schema!(type Int64 i64);
        //     schema!(type Uint8 u8);
        //     schema!(type Uint16 u16);
        //     schema!(type Uint32 u32);
        //     schema!(type Uint64 u64);
        schema!(type Float float);
        //        schema!(type Float32 f32);
        schema!(type Float64 f64);
        schema!(type TString String);
        schema!(type Bytes1 bytes);
        // schema!(type BytesCopy = Bytes1);

        // recursive types
        //        schema!(type StringLink &String);
        //        schema!(type List [TString]);
        //        schema!(type Map {String: List});

        //////////////////////////////////////////////////////////////////////////
        // IPLD schema types and representations
        //////////////////////////////////////////////////////////////////////////

        // map
        //        schema!(type MapMap {TString: List} representation map);
        //        schema!(type MapStringpairs {TString: List} representation stringpairs {
        //            innerDelim: ":",
        //            entryDelim: ","
        //        });
        //        schema!(type MapListpairs {TString: List} representation listpairs);

        // struct
        // schema!(type Struct struct {});
        // schema!(type StructMap struct {} representation map);
        // schema!(type StructTuple struct {} representation tuple);
        // schema!(type StructStringpairs struct {} representation stringpairs {
        //     innerDelim: ":",
        //     entryDelim: ","
        // });
        // schema!(type StructStringjoin struct {} representation stringjoin {
        //     join: ":"
        // });
        // schema!(type StructListpairs struct {} representation listpairs);

        // enum
        // schema!(type Enum enum {});
        // schema!(type EnumString enum {} representation string);
        // schema!(type EnumInt enum {} representation int);

        // union
        // schema!(type Union union {});
        // schema!(type UnionKeyed union {} representation keyed);
        // schema!(type UnionKinded union {} representation kinded);
        // schema!(type UnionEnvelope union {} representation envelope {
        //     discriminantKey: "",
        //     contentKey: ""
        // });
        // schema!(type UnionInline union {} representation inline {
        //     discriminantKey: ""
        // });
        // schema!(type UnionByteprefix union {} representation byteprefix);
    }
}
