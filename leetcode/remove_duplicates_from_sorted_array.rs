/// 
/// Problem: Remove Duplicates from Sorted Array
/// 
/// Given an integer array `nums` sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once.
/// The relative order of the elements should be kept the same.
///
/// Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the first part of the array.
/// Any extra elements after the unique elements can be set to any value.
///
/// Return `k` after placing the final result in the first `k` slots of `nums`.
///
/// Do not allocate extra space for another array. You must do this by modifying the input array in-place with O(1) extra memory.
///
/// Example 1:
/// Input: nums = [1,1,2]
/// Output: 2, nums = [1,2,_]
/// Explanation: Your function should return k = 2, with the first two elements of nums being 1 and 2 respectively.
/// It does not matter what you leave beyond the returned k (hence they are underscores).
///
/// Example 2:
/// Input: nums = [0,0,1,1,1,2,2,3,3,4]
/// Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]
///
/// Time complexity: O(n)
/// Space complexity: O(1)
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut write_index = 1;

        for read_index in 1..nums.len() {
            if nums[read_index] != nums[read_index - 1] {
                nums[write_index] = nums[read_index];
                write_index += 1;
            }
        }

        write_index as i32
    }
}
