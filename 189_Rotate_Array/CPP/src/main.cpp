#include <iostream>
#include <vector>
#include <algorithm>

class Solution {
public:
    void reverse(std::vector<int>& nums, int start, int end){
        while (start < end) {
            std::swap(nums[start], nums[end]);
            start++;
            end--;
        }
    }
    void manual_rotate(std::vector<int>& nums, int k) {
        int n = nums.size();
        k = k % n;

        reverse(nums, 0, n-1);
        reverse(nums, 0, k-1);
        reverse(nums, k, n-1);

    }
    void rotate(std::vector<int>& nums, int k) {
        k = k % nums.size();
        std::rotate(nums.begin(), nums.end()-k, nums.end());
    }
};
int main(int argc, char *argv[])
{
    std::vector<int> test = {1,2,3,4,5};
    int k = 3;
    Solution temp;
    std::cout << "First test using algorithm::rotate, should be easy and correct." << std::endl;
    temp.rotate(test, k);
    for (auto n : test) {
        std::cout << n << ' ';
    }
    std::cout << "\n";
    test = {1,2,3,4,5};
    std::cout << "Now testing custom function involving std::swap" << std::endl;
    temp.manual_rotate(test, k);
    for (auto n : test) {
        std::cout << n << ' ';
    }
}
