use super::*;

impl ThisParser for CallNode<SubscriptNode> {
    #[track_caller]
    fn parse(_: ParseState) -> ParseResult<Self> {
        unreachable!()
    }

    fn as_lisp(&self) -> Lisp {
        let mut lisp = Lisp::new(3);
        if self.rest.index0 {
            lisp += Lisp::keyword("call/subscript0");
        }
        else {
            lisp += Lisp::keyword("call/subscript1");
        }
        lisp += self.base.as_lisp();
        lisp += self.rest.as_lisp();
        lisp
    }
}

impl ThisParser for SubscriptNode {
    /// `[` ~ `]` | `[` [term](SubscriptTermNode::parse) ( ~ `,` ~ [term](SubscriptTermNode::parse))* `,`? `]`
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, (index0, terms)) = input.begin_choice().choose(parse_index1).choose(parse_index0).end_choice()?;
        state.finish(SubscriptNode { index0, terms: terms.body, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut lisp = Lisp::new(self.terms.len() + 2);
        lisp += Lisp::keyword(self.method());
        for term in &self.terms {
            lisp += term.as_lisp();
        }
        lisp
    }
}

#[inline]
fn parse_index_fake(input: ParseState) -> ParseResult<(bool, BracketPair<SubscriptTermNode>)> {
    let pat = BracketPattern::new("{", "}");
    pat.consume(input, ignore, SubscriptTermNode::parse).map_inner(|s| (false, s))
}

#[inline]
fn parse_index1(input: ParseState) -> ParseResult<(bool, BracketPair<SubscriptTermNode>)> {
    let pat = BracketPattern::new("[", "]");
    pat.consume(input, ignore, SubscriptTermNode::parse).map_inner(|s| (false, s))
}

#[inline]
fn parse_index0(input: ParseState) -> ParseResult<(bool, BracketPair<SubscriptTermNode>)> {
    let pat = BracketPattern::new("⁅", "⁆");
    pat.consume(input, ignore, SubscriptTermNode::parse).map_inner(|s| (true, s))
}

impl ThisParser for SubscriptTermNode {
    /// [range](SubscriptTermNode::parse_ranged) | [index](SubscriptTermNode::parse_single)
    fn parse(input: ParseState) -> ParseResult<Self> {
        input
            .begin_choice()
            .choose(|s| SubscriptSliceNode::parse(s).map_into())
            .choose(|s| ExpressionNode::parse(s).map_into())
            .end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        match self {
            SubscriptTermNode::Index(v) => v.as_lisp(),
            SubscriptTermNode::Slice(v) => v.as_lisp(),
        }
    }
}

impl ThisParser for SubscriptSliceNode {
    /// [start](ExpressionBody::parse)? ~ `:` ~ [end](ExpressionBody::parse)? (~ `:` ~ [step](ExpressionBody::parse)?)?
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, start) = input.match_optional(ExpressionNode::parse)?;
        let (state, _) = state.skip(ignore).match_char(':')?;
        let (state, end) = state.skip(ignore).match_optional(ExpressionNode::parse)?;
        let (state, step) = state.match_optional(maybe_step)?;
        state.finish(Self { start, end, step: step.flatten(), span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut lisp = Lisp::new(4);
        lisp += Lisp::symbol("range");
        match &self.start {
            None => lisp += Lisp::keyword("null"),
            Some(s) => lisp += s.as_lisp(),
        }
        match &self.end {
            None => lisp += Lisp::keyword("null"),
            Some(s) => lisp += s.as_lisp(),
        }
        match &self.step {
            None => lisp += Lisp::keyword("null"),
            Some(s) => lisp += s.as_lisp(),
        }
        lisp
    }
}

/// `~ : ~ step?`
fn maybe_step(input: ParseState) -> ParseResult<Option<ExpressionNode>> {
    let (state, _) = input.skip(ignore).match_char(':')?;
    let (state, term) = state.skip(ignore).match_optional(ExpressionNode::parse)?;
    state.finish(term)
}
