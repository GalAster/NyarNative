use super::*;

impl PrettyPrint for ImportStatementNode {
    // fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    //     write!(f, "import ")?;
    //     Display::fmt(&self.r#type, f)?;
    //
    //     Ok(())
    // }

    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        todo!()
    }
}

impl PrettyPrint for ImportStatementType {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        match self {
            Self::Alias(node) => node.build(allocator),
            Self::Group(node) => node.build(allocator),
            Self::String(node) => node.build(allocator),
        }
    }
}

impl PrettyPrint for ImportTermNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        match self {
            Self::Alias(node) => node.build(allocator),
            Self::Group(node) => node.build(allocator),
        }
    }
}

impl PrettyPrint for ImportGroupNode {
    // fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    //     write!(f, "{} {{ {} }}", self.path, self.group.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "))
    // }

    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        todo!()
    }
}
impl PrettyPrint for ImportAliasNode {
    // fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    //     write!(f, "{} as {}", self.path, self.alias)
    // }

    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        todo!()
    }
}
impl From<ImportAliasNode> for ImportTermNode {
    fn from(value: ImportAliasNode) -> Self {
        Self::Alias(Box::new(value))
    }
}

impl From<ImportGroupNode> for ImportTermNode {
    fn from(value: ImportGroupNode) -> Self {
        Self::Group(Box::new(value))
    }
}
