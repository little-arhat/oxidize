pub use self::contains::Contains;
pub use self::contains_only::ContainsOnly;
pub use self::equality::{Equal, GreaterThan, LessThan};
pub use self::length::Empty;
pub use self::option::{Nothing, Something};
pub use self::truthiness::{BeTrue, BeFalse};
pub use self::regex::MatchesRegex;

pub mod contains;
pub mod contains_only;
pub mod equality;
pub mod length;
pub mod option;
pub mod truthiness;
pub mod regex;

pub trait Matcher<Lhs> {
    fn matches(&self, lhs: &Lhs) -> bool;
    fn fail_msg(&self, lhs: &Lhs) -> String;
    fn negated_fail_msg(&self, lhs: &Lhs) -> String;
}

