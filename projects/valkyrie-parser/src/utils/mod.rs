use crate::{
    helpers::ProgramState, AnnotationTermMixNode, AnnotationTermNode, MatchTermsNode, ModifierAheadNode, TupleTermsNode,
};
use nyar_error::Result;
use valkyrie_ast::{ArgumentsList, AttributeList, ModifierList, PatternBranch};

pub(crate) fn build_arguments(this: &Option<TupleTermsNode>, ctx: &mut ProgramState) -> Result<ArgumentsList> {
    match this {
        Some(s) => s.build(ctx),
        None => Ok(ArgumentsList { terms: vec![] }),
    }
}

pub(crate) fn build_modifier_ahead(this: &[ModifierAheadNode], ctx: &mut ProgramState) -> ModifierList {
    ModifierList { terms: this.iter().map(|s| s.build(ctx)).collect() }
}

pub(crate) fn build_annotation_terms(this: &[AnnotationTermNode], ctx: &mut ProgramState) -> Result<AttributeList> {
    let mut terms = vec![];
    for term in this {
        match term.build(ctx) {
            Ok(o) => terms.push(o),
            Err(e) => ctx.add_error(e),
        }
    }
    Ok(AttributeList { terms: terms.into_iter().map(|v| v.terms).flatten().collect() })
}

pub(crate) fn build_annotation_terms_mix(this: &[AnnotationTermMixNode], ctx: &mut ProgramState) -> Result<AttributeList> {
    let mut terms = vec![];
    for term in this {
        match term.build(ctx) {
            Ok(o) => terms.push(o),
            Err(e) => ctx.add_error(e),
        }
    }
    Ok(AttributeList { terms: terms.into_iter().map(|v| v.terms).flatten().collect() })
}
pub(crate) fn build_match_terms(this: &[MatchTermsNode], ctx: &mut ProgramState) -> Vec<PatternBranch> {
    let mut terms = Vec::with_capacity(this.len());
    for term in this {
        match term.build(ctx) {
            Ok(o) => terms.extend(o),
            Err(e) => ctx.add_error(e),
        }
    }
    terms
}
