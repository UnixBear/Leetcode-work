#include <iostream>
#include <vector>
#include <unordered_map>

class Solution {
public:
    int majorityElement(std::vector<int>& nums) {
        std::unordered_map<int,int> tracker;
        int largestKey = 0;
        for (int num : nums) {
            auto it = tracker.find(num);
            if (it != tracker.end()){
                it->second++;
            }
            else {
                tracker[num] = 1;
            }
            if (tracker[num] > tracker[largestKey]) {
                largestKey = num;
            }
        }
        return largestKey;
    }
};
int main(int argc, char *argv[])
{
    Solution test;
    std::vector<int> nums = {1,2,2,2,3,3,4};
    std::cout << "the largest key is: " << test.majorityElement(nums) << std::endl;
}
