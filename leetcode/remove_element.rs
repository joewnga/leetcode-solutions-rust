/// 
/// Problem: Remove Element
/// 
/// Given an integer array `nums` and an integer `val`, remove all occurrences of `val` in `nums` in-place.
/// The relative order of the elements may be changed.
///
/// Since it is impossible to change the length of the array in some languages, you must do this by modifying the input array in-place with O(1) extra memory.
///
/// Return `k` after placing the final result in the first `k` slots of `nums`.
///
/// Example 1:
/// Input: nums = [3,2,2,3], val = 3
/// Output: 2, nums = [2,2,_,_]
/// Explanation: Your function should return k = 2, with the first two elements of nums being 2.
/// It does not matter what you leave beyond the returned k (hence they are underscores).
///
/// Example 2:
/// Input: nums = [0,1,2,2,3,0,4,2], val = 2
/// Output: 5, nums = [0,1,4,0,3,_,_,_]
///
/// Time complexity: O(n)
/// Space complexity: O(1)
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut write_index = 0;

        for read_index in 0..nums.len() {
            if nums[read_index] != val {
                nums[write_index] = nums[read_index];
                write_index += 1;
            }
        }

        write_index as i32
    }
}
