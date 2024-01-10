#include <iostream>
#include <algorithm>
#include <vector>

class Solution {
public:
    int removeElement(std::vector<int>& nums, int val) {
        //remove() moves all elements that match val to the end of the array and returns an 
        //iterator that will start at the part of the array where the elements matching
        //val are
        //erase() is deletes everything from the iterator to the end 
        nums.erase(std::remove(nums.begin(), nums.end(), val), nums.end());
        return nums.size();
    }
};

int main(int argc, char *argv[])
{
    std::vector<int> testvec = {1,2,3,3,3,4,5};
    std::cout << "The test vector is {1,2,3,3,3,4,5} and we're removing 3 from it. \n Expected response should be 4" << std::endl;
    Solution test;
    int answer = test.removeElement(testvec, 3);
    std::cout << answer;
}
