use super::*;

impl FunctionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FunctionType::Macro => "macro",
            FunctionType::Micro => "micro",
        }
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for FunctionType {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.keyword(self.as_str())
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for FunctionDeclaration {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(4);
        for m in &self.modifiers {
            terms.push(allocator.keyword(m.name.clone()));
            terms.push(allocator.space());
        }
        terms.push(allocator.keyword(self.r#type.as_str()));
        terms.push(allocator.space());
        terms.push(self.namepath.build(allocator));
        terms.push(self.generic.build(allocator));
        terms.push(self.arguments.build(allocator));
        if let Some(ret) = &self.r#return {
            terms.push(allocator.text(": "));
            terms.push(ret.build(allocator));
        }
        terms.push(self.body.build(allocator));
        allocator.concat(terms)
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for FunctionBody {
    /// ```vk
    /// # inline style
    /// { ... }
    ///
    /// # block style
    /// {
    ///    ...
    /// }
    /// ```
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let statements = match &self.statements {
            Some(s) => s.as_slice(),
            None => {
                return allocator.nil();
            }
        };

        let mut terms = Vec::with_capacity(9);
        terms.push(allocator.space());
        terms.push(allocator.text("{"));
        terms.push(allocator.hardline());
        terms.push(allocator.intersperse(statements, allocator.hardline()).indent(4));
        if let Some(s) = self.last() {
            if s.end_semicolon {
                terms.push(allocator.text(";"));
            }
        }
        terms.push(allocator.hardline());
        terms.push(allocator.text("}"));
        allocator.concat(terms)
    }
}
#[cfg(feature = "pretty-print")]
impl<K: PrettyPrint, V: PrettyPrint, D: PrettyPrint> PrettyPrint for ArgumentTermNode<K, V, D> {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(3);
        terms.push(self.key.build(allocator));
        if let Some(value) = &self.value {
            terms.push(allocator.text(": "));
            terms.push(value.build(allocator));
        }
        if let Some(default) = &self.default {
            terms.push(allocator.text(" = "));
            terms.push(default.build(allocator));
        }
        allocator.concat(terms)
    }
}
