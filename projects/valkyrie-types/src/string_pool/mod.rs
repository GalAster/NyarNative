use lasso::{Spur, ThreadedRodeo};
use std::{
    fmt::{Debug, Display, Formatter, Write},
    ops::Range,
    str::{pattern::Pattern, Split},
    sync::{Arc, LazyLock},
};

pub mod identifier;
mod name_path;
mod string_id;
mod string_pool;
pub mod variable;

pub static STRING_POOL: LazyLock<StringPool> = std::sync::LazyLock::new(|| StringPool::default());

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct NamePath {
    key: Spur,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileName {
    key: Spur,
}

impl Default for FileName {
    fn default() -> Self {
        Self { key: STRING_POOL.encode_static("") }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct Location {
    file: FileName,
    start: u32,
    end: u32,
}

impl FileName {
    pub fn with_range(&self, range: Range<u32>) -> Location {
        Location { file: *self, start: range.start, end: range.end }
    }
}

impl Location {
    pub fn with_range(self, range: &Range<u32>) -> Self {
        Self { file: self.file, start: range.start, end: range.end }
    }
}

pub struct StringPool {
    pool: Arc<ThreadedRodeo<Spur>>,
}
