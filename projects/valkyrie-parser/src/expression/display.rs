use super::*;
use crate::symbol::ValkyrieIdentifier;
use lispify::{Lisp, LispSymbol, Lispify};
use std::fmt::Write;

impl Display for ValkyrieOperatorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValkyrieOperatorKind::Not => f.write_char('!'),
            ValkyrieOperatorKind::Concat => f.write_str("++"),
            ValkyrieOperatorKind::Positive => f.write_char('+'),
            ValkyrieOperatorKind::Negative => f.write_char('-'),
            ValkyrieOperatorKind::Plus => f.write_char('+'),
            ValkyrieOperatorKind::Minus => f.write_char('-'),
            ValkyrieOperatorKind::Multiply => f.write_char('*'),
            ValkyrieOperatorKind::Divide => f.write_char('/'),
            ValkyrieOperatorKind::Power => f.write_char('^'),
            ValkyrieOperatorKind::Unwrap => f.write_char('!'),
            ValkyrieOperatorKind::Raise => f.write_char('?'),
            ValkyrieOperatorKind::Celsius => f.write_char('℃'),
            ValkyrieOperatorKind::Fahrenheit => f.write_char('℉'),
            ValkyrieOperatorKind::Transpose => f.write_char('ᵀ'),
            ValkyrieOperatorKind::Transjugate => f.write_char('ᴴ'),
            ValkyrieOperatorKind::Hermitian => f.write_str("Hermitian"),
            ValkyrieOperatorKind::Unbox => f.write_char('*'),
            ValkyrieOperatorKind::Unpack => f.write_str("**"),
            ValkyrieOperatorKind::Greater => f.write_char('>'),
            ValkyrieOperatorKind::MuchGreater => f.write_char('≫'),
            ValkyrieOperatorKind::VeryMuchGreater => f.write_char('⋙'),
            ValkyrieOperatorKind::Less => f.write_char('<'),
            ValkyrieOperatorKind::MuchLess => f.write_char('≪'),
            ValkyrieOperatorKind::VeryMuchLess => f.write_char('⋘'),
            ValkyrieOperatorKind::Belongs(v) => match v {
                true => f.write_char('∈'),
                false => f.write_char('∉'),
            },
            ValkyrieOperatorKind::Equal(v) => match v {
                true => f.write_char('≖'),
                false => f.write_char('≠'),
            },
            ValkyrieOperatorKind::StrictlyEqual(v) => match v {
                true => f.write_char('≡'),
                false => f.write_char('≢'),
            },
            ValkyrieOperatorKind::Inverse => f.write_char('i'),
            ValkyrieOperatorKind::Surd(v) => match v {
                3 => f.write_char('∛'),
                4 => f.write_char('∜'),
                _ => f.write_char('√'),
            },
            ValkyrieOperatorKind::DivideByDecimalPower(v) => match v {
                3 => f.write_char('‰'),
                4 => f.write_char('‱'),
                _ => f.write_char('%'),
            },
        }
    }
}

impl Display for ValkyrieOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl Lispify for ValkyrieOperatorKind {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        Lisp::Operator(self.to_string())
    }
}

impl Lispify for ValkyrieOperator {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        self.kind.lispify()
    }
}

impl Lispify for ValkyrieExpression {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        match self {
            ValkyrieExpression::Placeholder => Lisp::Keyword("placeholder".into()),
            ValkyrieExpression::Prefix(v) => v.lispify().into(),
            ValkyrieExpression::Binary(v) => v.lispify().into(),
            ValkyrieExpression::Suffix(v) => v.lispify().into(),
            ValkyrieExpression::Number(v) => v.lispify().into(),
            ValkyrieExpression::Symbol(v) => v.lispify().into(),
            ValkyrieExpression::String(v) => v.lispify().into(),
        }
    }
}

impl Lispify for ValkyrieNamepath {
    type Output = LispSymbol;

    fn lispify(&self) -> Self::Output {
        let mut terms = self.names.iter().map(|s| s.name.clone());
        let first = terms.next().unwrap_or_default();

        LispSymbol { name: first, path: terms.collect() }
    }
}

impl Lispify for ValkyrieIdentifier {
    type Output = LispSymbol;

    fn lispify(&self) -> Self::Output {
        LispSymbol { name: self.name.clone(), path: vec![] }
    }
}

impl Lispify for ValkyrieBinary {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        Lisp::operator(self.operator.to_string(), &[self.lhs.lispify(), self.rhs.lispify()])
    }
}
impl Lispify for ValkyrieUnary {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        Lisp::operator(self.operator.to_string(), &[self.body.lispify()])
    }
}
