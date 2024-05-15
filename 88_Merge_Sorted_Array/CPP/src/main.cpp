#include <iostream>
#include <vector>
#include <algorithm>

class Solution {
public:
    void merge(std::vector<int>& nums1, int m, std::vector<int>& nums2, int n) {
        
        int nums1_nonzero_tracker = m - 1;
        int nums2_tracker = n - 1;
        int nums1_tracker = m + n - 1;

        while (nums2_tracker >= 0) {
            if (nums1_nonzero_tracker >= 0 && nums1[nums1_nonzero_tracker] > nums2[nums2_tracker]) {
                nums1[nums1_tracker--] = nums1[nums1_nonzero_tracker--];
            }
            else {
                nums1[nums1_tracker--] = nums2[nums2_tracker--];
            }
        }
    }
};
int main(int argc, char *argv[])
{
    std::vector<int> nums1 = {1, 2, 3, 0, 0, 0};
    int m = 3;
    std::vector<int>nums2 = {2, 5, 6};
    int n = 3; 
    
    Solution test;
    test.merge(nums1, m, nums2, n);

    for (auto num : nums1) {
        std::cout << num << " ";
    }
    std::cout << std::endl;

    std::cout << "Hello world!" << std::endl;
    return 0;
}
