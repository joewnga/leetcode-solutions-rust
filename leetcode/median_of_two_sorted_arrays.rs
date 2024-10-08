/// 
/// Problem: Median of Two Sorted Arrays
/// 
/// There are two sorted arrays nums1 and nums2 of size m and n respectively.
/// 
/// Find the median of the two sorted arrays. The overall run time complexity
/// should be O(log (m+n)).
/// 
/// You may assume nums1 and nums2 cannot be both empty.
/// 
/// Example 1:
/// 
/// nums1 = [1, 3]
/// nums2 = [2]
/// 
/// The median is 2.0

/// Solution 1:
/// Time complexity: 0(n log n)
/// Space complexity: O(m + n)

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
         let mut numbers = [nums1, nums2].concat();
    numbers.sort();
    let numbers_length = numbers.len();
    let median_index = numbers_length / 2;

    if numbers_length % 2 == 0 {
        let lower = median_index - 1;
        let upper = median_index;
        return (numbers[lower] as f64 + numbers[upper] as f64) / 2.0;
    } else {
        return numbers[median_index] as f64;
    }
        
    }
}

/// Solution 2:
/// Time complexity: O(m + n)
/// Space complexity: O(m + n)

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut merged = Vec::new();  

        
        let mut i = 0; 
        let mut j = 0;
        
        
        while i < nums1.len() && j < nums2.len() {
            if nums1[i] < nums2[j] {
                merged.push(nums1[i]);
                i += 1;
            } else {
                merged.push(nums2[j]);
                j += 1;
            }
        }
        
        
        while i < nums1.len() {
            merged.push(nums1[i]);
            i += 1;
        }
        
        
        while j < nums2.len() {
            merged.push(nums2[j]);
            j += 1;
        }
        
        
        let len = merged.len();
        if len % 2 == 0 {
            return (merged[len / 2 - 1] as f64 + merged[len / 2] as f64) / 2.0;
        } else {
            return merged[len / 2] as f64;
        }
        
    }
}