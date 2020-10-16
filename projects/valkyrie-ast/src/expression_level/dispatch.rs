use super::*;

impl Default for ExpressionType {
    fn default() -> Self {
        Self::Placeholder
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ExpressionNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        self.body.pretty(theme)
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ExpressionType {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            Self::Placeholder => unreachable!(),
            Self::Slot(node) => node.pretty(theme),
            Self::Symbol(node) => node.pretty(theme),
            Self::Number(node) => node.pretty(theme),
            Self::Text(node) => node.pretty(theme),
            Self::String(node) => node.pretty(theme),
            Self::Prefix(node) => node.pretty(theme),
            Self::Binary(node) => node.pretty(theme),
            Self::Suffix(node) => node.pretty(theme),
            Self::Table(node) => node.pretty(theme),
            Self::Apply(node) => node.pretty(theme),
            Self::ApplyDot(node) => node.pretty(theme),
            Self::LambdaCall(node) => node.pretty(theme),
            Self::LambdaDot(node) => node.pretty(theme),
            Self::Subscript(node) => node.pretty(theme),
            Self::GenericCall(node) => node.pretty(theme),
            Self::New(node) => node.pretty(theme),
            Self::Resume(node) => node.pretty(theme),
            Self::If(node) => node.pretty(theme),
            Self::IfLet(node) => node.pretty(theme),
            Self::Switch(node) => node.pretty(theme),
        }
    }
}

impl ExpressionType {
    pub fn span(&self) -> Range<u32> {
        match self {
            Self::Placeholder => unreachable!(),
            Self::Slot(node) => node.span.clone(),
            Self::Symbol(node) => node.span.clone(),
            Self::Number(node) => node.span.clone(),
            Self::Text(node) => node.span.clone(),
            Self::String(node) => node.span.clone(),
            Self::New(node) => node.span.clone(),
            Self::Prefix(node) => node.span.clone(),
            Self::Binary(node) => node.span.clone(),
            Self::Suffix(node) => node.span.clone(),
            Self::Table(node) => node.span.clone(),
            Self::Apply(node) => node.span.clone(),
            Self::ApplyDot(node) => node.span.clone(),
            Self::LambdaCall(node) => node.span.clone(),
            Self::LambdaDot(node) => node.span.clone(),
            Self::Subscript(node) => node.span.clone(),
            Self::GenericCall(node) => node.span.clone(),
            Self::Resume(node) => node.span.clone(),
            Self::If(node) => node.span.clone(),
            Self::IfLet(node) => node.span.clone(),
            Self::Switch(node) => node.span.clone(),
        }
    }
}
