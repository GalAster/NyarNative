use shredder::{marker::GcSafe, Gc, Scan, Scanner};
use std::fmt::{Debug, Formatter};
use valkyrie_ast::helper::ValkyrieNode;

mod der;
mod jupyter;
mod ser;

#[cfg(feature = "polars")]
use crate::builtin::data_frame::ValkyrieDataFrame;
use crate::{
    builtin::{images::ValkyrieImage, ndarray::ValkyrieNDArray},
    ValkyrieClassType, ValkyrieDict, ValkyrieError, ValkyrieList, ValkyrieNumber, ValkyrieVariantType,
};

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum ValkyrieValue {
    /// ADT = -1
    Nothing,
    /// ADT = 0
    Null,
    /// ADT = 1
    Unit,
    /// ADT = 2
    ///
    /// Native boolean type, 8bit
    Boolean(bool),
    Number(ValkyrieNumber),
    Unicode(char),
    UTF8String(Gc<String>),
    Bytes(Gc<Vec<u8>>),
    Html(Gc<String>),
    NDArray(Gc<ValkyrieNDArray>),
    Image(Gc<ValkyrieImage>),
    #[cfg(feature = "polars")]
    DataFrame(ValkyrieDataFrame),
    List(ValkyrieList),
    Dict(ValkyrieDict),
    Class(ValkyrieClassType),
    Variant(Gc<ValkyrieVariantType>),
}

impl ValkyrieValue {
    pub fn is_truthy(&self) -> Result<bool, ValkyrieError> {
        match self {
            ValkyrieValue::Boolean(v) => Ok(*v),
            _ => Err(ValkyrieError::runtime_error("Expected boolean value")),
        }
    }
    pub fn is_nothing(&self) -> bool {
        matches!(self, ValkyrieValue::Nothing)
    }
}

unsafe impl GcSafe for ValkyrieValue {}

unsafe impl Scan for ValkyrieValue {
    fn scan(&self, scanner: &mut Scanner<'_>) {
        match self {
            ValkyrieValue::Nothing => {}
            ValkyrieValue::Null => {}
            ValkyrieValue::Unit => {}
            ValkyrieValue::Boolean(_) => {}
            ValkyrieValue::Number(v) => scanner.scan(v),
            ValkyrieValue::Unicode(_) => {}
            ValkyrieValue::UTF8String(v) => scanner.scan(v),
            ValkyrieValue::Bytes(v) => scanner.scan(v),
            ValkyrieValue::Html(_) => {}
            ValkyrieValue::NDArray(_) => {}
            ValkyrieValue::Image(_) => {}
            #[cfg(feature = "polars")]
            ValkyrieValue::DataFrame(_) => {}
            ValkyrieValue::List(v) => scanner.scan(v),
            ValkyrieValue::Dict(v) => scanner.scan(v),
            ValkyrieValue::Class(_) => {}
            ValkyrieValue::Variant(_) => {}
        }
    }
}

impl Debug for ValkyrieValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValkyrieValue::Nothing => f.write_str("nothing"),
            ValkyrieValue::Null => f.write_str("null"),
            ValkyrieValue::Unit => f.write_str("()"),
            ValkyrieValue::Boolean(v) => Debug::fmt(v, f),
            ValkyrieValue::Number(v) => Debug::fmt(v, f),
            ValkyrieValue::Unicode(v) => Debug::fmt(v, f),
            ValkyrieValue::UTF8String(v) => Debug::fmt(v, f),
            ValkyrieValue::Bytes(v) => Debug::fmt(v, f),
            ValkyrieValue::Html(v) => Debug::fmt(v, f),
            ValkyrieValue::NDArray(v) => Debug::fmt(v, f),
            ValkyrieValue::Image(v) => Debug::fmt(v, f),
            #[cfg(feature = "polars")]
            ValkyrieValue::DataFrame(v) => Debug::fmt(v, f),
            ValkyrieValue::List(v) => Debug::fmt(v, f),
            ValkyrieValue::Dict(v) => Debug::fmt(v, f),
            ValkyrieValue::Class(v) => Debug::fmt(v, f),
            ValkyrieValue::Variant(v) => Debug::fmt(v, f),
        }
    }
}
