//! # Pascal's Triangle
//! LeetCode URL: https://leetcode.com/problems/pascals-triangle/description/?envType=problem-list-v2&envId=dynamic-programming
//! ## Problem description:
//! Given an integer **numRows**, return the first **numRows** of Pascal's triangle.
//! ## Constraints:
//! - 1 <= **numRows** <= 30

pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        if num_rows < 1 || num_rows > 30 {
            return vec![];
        }
        if num_rows == 1 {
            return vec![vec![1]];
        }
        let mut tr: Vec<Vec<i32>> = vec![vec![1], vec![1, 1]];
        for i in 3 ..= num_rows as usize {
            let mut row: Vec<i32> = vec![1; i];
            let prev: &[i32] = &tr[i - 2];
            for j in 1..prev.len() {
                row[j] = prev[j] + prev[j - 1];
            }
            tr.push(row);
        }
        tr
    }
}

mod pascal_triangle_rec {

    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        if num_rows < 1 || num_rows > 30 {
            return vec![];
        }
        _generate(num_rows as usize)
    }

    fn _generate(row: usize) -> Vec<Vec<i32>> {
        if row == 1 {
            return vec![vec![1]];
        }
        let mut tr = _generate(row - 1);
        let mut r: Vec<i32> = vec![1; row];
        let p: &[i32] = &tr[row - 2];
        for i in 1..p.len() {
            r[i] = p[i - 1] + p[i];
        }
        tr.push(r);
        tr
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::generate(1), vec![vec![1]]);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            pascal_triangle_rec::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        )
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::generate(3),
            vec![vec![1], vec![1, 1], vec![1, 2, 1],]
        )
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::generate(4),
            vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1],]
        )
    }
}
