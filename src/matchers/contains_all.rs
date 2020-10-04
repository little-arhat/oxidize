use super::Matcher;
use std::fmt::Debug;

pub struct ContainsAll<'a, T>(pub &'a [T]);

impl<T: Debug + PartialEq> Matcher<Vec<T>> for ContainsAll<'_, T> {
    fn matches(&self, lhs: &Vec<T>) -> bool {
        self.0.iter().all(|ri| lhs.iter().any(|li| *li == *ri))
    }

    fn fail_msg(&self, lhs: &Vec<T>) -> String {
        format!("expected {:?} to contain all from {:?}", lhs, self.0)
    }

    fn negated_fail_msg(&self, lhs: &Vec<T>) -> String {
        format!("expected {:?} to not contain all from {:?}", lhs, self.0)
    }
}

#[cfg(test)]
mod test {
    mod vec_t_contains_all_t {
        use super::super::super::super::dsl::*;

        #[test]
        fn test_does_not_contain_everything_does_not_match() {
            expect(vec![1, 2]).to_not(contain_all(&[1, 2, 3]));
        }

        #[test]
        fn test_contains_all_with_vector_matches() {
            expect(vec![1, 2]).to(contain_all(&[1, 2]));
        }

        #[test]
        #[should_panic(expected = "expected [1, 2] to contain all from [1, 2, 3]")]
        fn test_contains_all_with_vector_fails_with_message() {
            expect(vec![1, 2]).to(contain_all(&[1, 2, 3]));
        }

        #[test]
        #[should_panic(expected = "expected [1, 2, 3] to not contain all from [1, 2]")]
        fn test_negated_contains_with_vector_fails_with_message() {
            expect(vec![1, 2, 3]).to_not(contain_all(&[1, 2]));
        }
    }
}
