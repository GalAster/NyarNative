use super::*;
use crate::utils::build_match_terms;
impl crate::DotMatchCallNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<MatchCallNode> {
        let patterns = build_match_terms(&self.match_terms, ctx);
        let monadic = self.op_and_then.is_some();

        Ok(MatchCallNode {
            monadic,
            base: Default::default(),
            kind: self.kw_match.build(),
            patterns: PatternBlock { branches: vec![], span: Default::default() },
            span: self.span.clone(),
        })
    }
}
