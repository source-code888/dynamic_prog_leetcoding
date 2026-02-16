
pub struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut row: Vec<i32> = vec![1; row_index as usize + 1];
        for r in 1..row_index as usize{
            let prev: Vec<i32> = row[0..r + 1].to_vec();
            for j in 1 .. r + 1 {
                row[j] = prev[j] + prev[j - 1];
            }
        }
        row
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        let row: Vec<i32> = Solution::get_row(3);
        assert_eq!(row, vec![1, 3, 3, 1]);
    }

    #[test]
    fn test_2() {
        let row: Vec<i32> = Solution::get_row(4);
        assert_eq!(row, vec![1, 4, 6, 4, 1]);
    }

}
