use super::*;

/// `array⁅index0⁆, array[index1]`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubscriptCallNode {
    /// kind of
    pub kind: RangeKind,
    /// `array`
    pub base: ExpressionType,
    /// `array?[0]`
    pub monadic: bool,
    /// `array[1, 2:3]`
    pub terms: Vec<RangeTermNode>,
    /// The range of the node.
    pub span: Range<u32>,
}
impl ValkyrieNode for SubscriptCallNode {
    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}
impl SubscriptCallNode {
    /// Replace placeholder with actual expression
    pub fn with_base(self, base: ExpressionType) -> Self {
        Self { base, ..self }
    }
    /// The linked method name
    pub fn method(&self) -> &'static str {
        match self.kind {
            RangeKind::Ordinal => "subscript1",
            RangeKind::Offset => "subscript0",
        }
    }
}
