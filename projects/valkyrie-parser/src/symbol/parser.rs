use super::*;


impl FromStr for ValkyrieIdentifier {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, Self::parse)
    }
}

pub static IDENTIFIER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?ux)
    ^(?P<regular>(?:\p{XID_Start}|_)\p{XID_Continue}*)
|   ^`(?P<escaped>(?:\\.|[^`])*)`
    ").unwrap()
});

impl ValkyrieIdentifier {
    /// - regular: `\p{XID_Start}|_)\p{XID_Continue}*`
    /// - escaped: ``` `(\.|[^`])*` ```
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&IDENTIFIER, "IDENTIFIER")?;
        let id = ValkyrieIdentifier {
            name: m.as_str().to_string(),
            range: state.away_from(input),
        };
        state.finish(id)
    }
}

#[test]
pub fn test() {
    let input = "`_he\\u2222llo`";
    let id = ValkyrieIdentifier::from_str(input).unwrap();
    println!("{}", id)
}
