mod expand;
#[macro_use]
mod expand_newtypes;
mod parse;

use crate::SchemaMeta;
use proc_macro2::TokenStream;
use syn::{Path, Type};

#[derive(Debug)]
pub struct NullReprDefinition;

#[derive(Debug)]
pub struct BoolReprDefinition;

#[derive(Debug)]
pub struct IntReprDefinition(pub(crate) Type);

#[derive(Debug)]
pub struct FloatReprDefinition(pub(crate) Type);

#[derive(Debug)]
pub struct StringReprDefinition;

#[derive(Debug)]
pub struct LinkReprDefinition(pub(crate) Type);

#[derive(Debug)]
pub struct CopyReprDefinition(pub(crate) Type);

#[derive(Debug)]
pub enum BytesReprDefinition {
    Basic,
    Advanced(AdvancedBytesReprDefinition),
}

#[derive(Debug)]
pub struct AdvancedBytesSchemaDefinition {
    meta: SchemaMeta,
    repr_def: AdvancedBytesReprDefinition,
}

#[derive(Debug)]
pub struct AdvancedBytesReprDefinition {
    pub name: Path,
    pub rest: TokenStream,
}
