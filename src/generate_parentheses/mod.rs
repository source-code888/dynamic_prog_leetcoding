//! # Generate Parentheses
//! LeetCode URL: https://leetcode.com/problems/generate-parentheses/description/?envType=problem-list-v2&envId=dynamic-programming
//! ## Problem description:
//! Given n pairs of parentheses, write a function to generate all combinations
//! of well-formed parentheses.
pub struct Solution;

impl Solution {
    pub fn generate_parentheses(n: i32) -> Vec<String> {
        if n < 1 || n > 8 {
            return vec![];
        }
        Self::combine_parentheses((1i32, false), (0i32, false), n, String::from("("))
    }

    fn combine_parentheses(l: (i32, bool), r: (i32, bool), m: i32, buff: String) -> Vec<String> {
        if l.0 > m || r.0 > m || r.0 > l.0 {
            return vec![];
        }
        let mut buff = buff;
        let l_count = if l.1 {
            buff.push('(');
            l.0 + 1
        } else {
            l.0
        };
        let r_count = if r.1 {
            buff.push(')');
            r.0 + 1
        } else {
            r.0
        };
        let mut combinations = if l_count == r_count && l_count == m {
            vec![buff.clone()]
        } else {
            vec![]
        };
        let mut left =
            Self::combine_parentheses((l_count, true), (r_count, false), m, buff.clone());
        let mut right = Self::combine_parentheses((l_count, false), (r_count, true), m, buff);
        combinations.append(&mut left);
        combinations.append(&mut right);
        combinations
    }
}

mod generate_parentheses_v2 {
    pub fn generate_parentheses(n: i32) -> Vec<String> {
        _generate_parentheses(0, 0, n, &mut String::new())
    }

    fn _generate_parentheses(l: i32, r: i32, m: i32, s: &mut String) -> Vec<String> {
        if l == r && l == m {
            return vec![s.clone()];
        }
        let mut result = Vec::<String>::new();
        if l < m {
            s.push('(');
            result.append(&mut _generate_parentheses(l + 1, r, m, s));
            s.pop();
        }

        if r < l {
            s.push(')');
            result.append(&mut _generate_parentheses(l, r + 1, m, s));
            s.pop();
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::generate_parentheses(3),
            ["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::generate_parentheses(1), ["()"]);
    }

    #[test]
    fn test3() {
        assert_eq!(
            generate_parentheses_v2::generate_parentheses(2),
            ["(())", "()()"]
        );
    }
}
