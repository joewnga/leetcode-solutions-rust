/// 
/// Problem: Insert Interval
/// 
/// You are given an array of **non-overlapping** intervals, where `intervals[i] = [start_i, end_i]`
/// and a new interval `[start, end]`. Insert this new interval into `intervals`, ensuring the final output
/// is also **non-overlapping**.
///
/// **Example 1:**
/// ```plaintext
/// Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
/// Output: [[1,5],[6,9]]
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
/// Output: [[1,2],[3,10],[12,16]]
/// Explanation: The new interval [4,8] merges with [3,5], [6,7], and [8,10].
/// ```
///
/// **Constraints:**
/// - `0 <= intervals.length <= 10^4`
/// - `intervals[i].length == 2`
/// - `0 <= start_i <= end_i <= 10^4`
/// - `0 <= start <= end <= 10^4`
///
/// # Solution:
/// - **Time Complexity:** `O(n)`
/// - **Space Complexity:** `O(n)`
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let (mut new_start, mut new_end) = (new_interval[0], new_interval[1]);
        let mut inserted = false;

        for interval in intervals.iter() {
            let (start, end) = (interval[0], interval[1]);

            if end < new_start {
                result.push(interval.clone());
            } else if start > new_end {
                if !inserted {
                    result.push(vec![new_start, new_end]);
                    inserted = true;
                }
                result.push(interval.clone());
            } else {
                new_start = new_start.min(start);
                new_end = new_end.max(end);
            }
        }

        if !inserted {
            result.push(vec![new_start, new_end]);
        }

        result
    }
}