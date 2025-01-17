/// 
/// Problem: Next Permutation
/// 
/// A permutation of an array of integers is an arrangement of its members into a sequence or linear order.
/// - The next permutation of an array of integers is the next lexicographically greater permutation.
/// - If no greater permutation is possible, rearrange it into the lowest possible order (sorted in ascending order).
///
/// The replacement must be done **in-place** with only constant extra memory.
///
/// Example 1:
/// Input: nums = [1,2,3]
/// Output: [1,3,2]
///
/// Example 2:
/// Input: nums = [3,2,1]
/// Output: [1,2,3]
///
/// Example 3:
/// Input: nums = [1,1,5]
/// Output: [1,5,1]
///
/// Constraints:
/// - `1 <= nums.length <= 100`
/// - `0 <= nums[i] <= 100`
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        if n < 2 {
            return;
        }

        let mut i = n as isize - 2;
        while i >= 0 && nums[i as usize] >= nums[i as usize + 1] {
            i -= 1;
        }

        if i >= 0 {
            let mut j = n as isize - 1;
            while nums[j as usize] <= nums[i as usize] {
                j -= 1;
            }
            nums.swap(i as usize, j as usize);
        }

        nums[i as usize + 1..].reverse();
    }
}
