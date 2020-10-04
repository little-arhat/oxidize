use super::Matcher;
use std::fmt::Debug;

pub struct ContainsOnly<T>(pub T);

impl<T: Debug + PartialEq> Matcher<Vec<T>> for ContainsOnly<T> {
    fn matches(&self, lhs: &Vec<T>) -> bool {
        lhs.len() == 1 && lhs.iter().any(|i| *i == self.0)
    }

    fn fail_msg(&self, lhs: &Vec<T>) -> String {
        format!("expected {:?} to contain only {:?}", lhs, self.0)
    }

    fn negated_fail_msg(&self, lhs: &Vec<T>) -> String {
        format!("expected {:?} not to contain only {:?}", lhs, self.0)
    }
}

#[cfg(test)]
mod test {
    mod vec_t_contains_t {
        use super::super::super::super::dsl::*;

        #[test]
        fn test_contains_more_with_vector_does_not_match() {
            expect(vec![1, 2, 3]).to_not(contain_only(1));
        }

        #[test]
        fn test_contains_only_with_vector_matches() {
            expect(vec![1]).to(contain_only(1));
        }

        #[test]
        #[should_panic(expected = "expected [1, 2, 3] to contain only 5")]
        fn test_contains_with_vector_fails_with_message() {
            expect(vec![1, 2, 3]).to(contain_only(5));
        }

        #[test]
        #[should_panic(expected = "expected [1, 2, 3] to contain only 2")]
        fn test_negated_contains_with_vector_fails_with_message() {
            expect(vec![1, 2, 3]).to(contain_only(2));
        }
    }
}
