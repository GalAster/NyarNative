#![feature(trivial_bounds)]
#![feature(allocator_api)]
#![feature(never_type)]
#![feature(unboxed_closures)]
#![feature(iter_from_generator)]
#![feature(generators)]
#![feature(lazy_cell)]

extern crate core;

mod builtin;
mod codegen;
mod collection;
mod functions;
mod modifiers;
// #[cfg(test)]
mod definitions;
mod encoding;
mod singletons;
pub mod testing;
pub mod third_party;
mod types;
mod utils;
mod validation;
mod values;

pub use self::{
    builtin::{
        images::ValkyrieImage,
        result::{ValkyrieFailure, ValkyrieSuccess},
        texts::{StringID, ValkyrieString},
        TokenType,
    },
    collection::{
        dict::ValkyrieDict,
        list::{ValkyrieList, ValkyrieOrdinal},
    },
    definitions::{
        classes::ValkyrieStructure, enumerates::ValkyrieEnumerate, interfaces::ValkyrieInterface, names::ValkyrieName,
    },
    functions::{ValkyrieFunction, ValkyrieFunctionType, ValkyrieMonomorphicFunction},
    modifiers::{FeatureType, InitializeType, MutableType},
    singletons::ids::{ValkyrieID, ValkyrieUniverse},
    types::{
        atomic_type::ValkyrieAtomicType, class_type::ValkyrieClassType, literal_type::ValkyrieLiteralType,
        union_type::ValkyrieUnionType, variant_type::ValkyrieVariantType, ValkyrieType,
    },
    values::ValkyrieValue,
};
pub use nyar_error::{
    Failure, FileCache, FileID, NyarError as ValkyrieError, Result as ValkyrieResult, RuntimeError, Success, SyntaxError,
};
pub use nyar_number::{Num, NyarReal as ValkyrieNumber, One, Zero};
pub use shredder::Gc;
pub use valkyrie_ast::ValkyrieOperator;
pub use valkyrie_parser::{ProgramContext, StringFormatterBuilder};
