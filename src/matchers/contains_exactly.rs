use super::Matcher;
use std::fmt::Debug;

pub struct ContainsExactly<'a, T>(pub &'a [T]);

impl<T: Debug + PartialEq> Matcher<Vec<T>> for ContainsExactly<'_, T> {
    fn matches(&self, lhs: &Vec<T>) -> bool {
        lhs.len() == self.0.len() && self.0.iter().all(|ri| lhs.iter().any(|li| *li == *ri))
    }

    fn fail_msg(&self, lhs: &Vec<T>) -> String {
        format!(
            "expected {:?} to contain only all elements from {:?}",
            lhs, self.0
        )
    }

    fn negated_fail_msg(&self, lhs: &Vec<T>) -> String {
        format!(
            "expected {:?} to not contain only all elements from {:?}",
            lhs, self.0
        )
    }
}

#[cfg(test)]
mod test {
    mod vec_t_contains_all_t {
        use super::super::super::super::dsl::*;

        #[test]
        fn test_does_not_contain_exactly_everything_does_not_match() {
            expect(vec![1, 2, 3]).to_not(contain_exactly(&[1, 2]));
        }

        #[test]
        fn test_contains_exactly_with_vector_matches() {
            expect(vec![1, 2]).to(contain_exactly(&[1, 2]));
        }

        #[test]
        #[should_panic(expected = "expected [1, 2, 3] to contain only all elements from [1, 2]")]
        fn test_contains_all_with_vector_fails_with_message() {
            expect(vec![1, 2, 3]).to(contain_exactly(&[1, 2]));
        }

        #[test]
        #[should_panic(
            expected = "expected [1, 2, 3] to not contain only all elements from [1, 2, 3]"
        )]
        fn test_negated_contains_with_vector_fails_with_message() {
            expect(vec![1, 2, 3]).to_not(contain_exactly(&[1, 2, 3]));
        }
    }
}
