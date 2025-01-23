/// 
/// Problem: Merge Intervals
/// 
/// Given an array of intervals where `intervals[i] = [start_i, end_i]`, 
/// merge all overlapping intervals and return an array of the **non-overlapping** intervals that cover all input intervals.
///
/// **Example 1:**
/// ```plaintext
/// Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
/// Output: [[1,6],[8,10],[15,18]]
/// Explanation: Intervals [1,3] and [2,6] overlap and are merged into [1,6].
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: intervals = [[1,4],[4,5]]
/// Output: [[1,5]]
/// Explanation: Intervals [1,4] and [4,5] overlap and are merged into [1,5].
/// ```
///
/// **Constraints:**
/// - `1 <= intervals.length <= 10^4`
/// - `intervals[i].length == 2`
/// - `0 <= start_i <= end_i <= 10^4`
///
/// # Solution:
/// - **Time Complexity:** `O(n log n)`
/// - **Space Complexity:** `O(n)`
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        
        if intervals.is_empty() {
            return vec![];
        }

        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut merged = Vec::new();
        merged.push(intervals[0].clone());

        for interval in intervals.iter().skip(1) {
            let last = merged.last_mut().unwrap();
            if interval[0] <= last[1] {
                last[1] = last[1].max(interval[1]);
            } else {
                merged.push(interval.clone());
            }
        }

        merged
        
    }
}