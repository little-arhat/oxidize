use std::fmt::Debug;
use super::Expectation;
use super::matchers::*;

/// Create an expectation with a value that can then be matched against.
pub fn expect<T: Debug>(lhs: T) -> Expectation<T> {
    Expectation(lhs)
}

pub fn equal<T: Debug>(rhs: T) -> Box<Equal<T>> {
    Box::new(Equal(rhs))
}

pub fn empty() -> Box<Empty> {
    Box::new(Empty)
}

pub fn contain<T: Debug>(rhs: T) -> Box<Contains<T>> {
    Box::new(Contains(rhs))
}

pub fn contain_only<T: Debug>(rhs: T) -> Box<ContainsOnly<T>> {
    Box::new(ContainsOnly(rhs))
}

pub fn greater_than<T: Debug>(rhs: T) -> Box<GreaterThan<T>> {
    Box::new(GreaterThan(rhs))
}

pub fn less_than<T: Debug>(rhs: T) -> Box<LessThan<T>> {
    Box::new(LessThan(rhs))
}

pub fn none() -> Box<Nothing> {
    Box::new(Nothing)
}

pub fn some() -> Box<Something> {
    Box::new(Something)
}

pub fn be_true() -> Box<BeTrue> {
    Box::new(BeTrue)
}

pub fn be_false() -> Box<BeFalse> {
    Box::new(BeFalse)
}

pub fn match_regex<'a>(rhs: &'a str) -> Box<MatchesRegex<'a>> {
    Box::new(MatchesRegex(rhs))
}
