#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

class Solution {
public:
    bool canJump(vector<int>& nums) {
        int maxReach = nums[0];
        int destination = nums.size() - 1;

        for (int i = 1; i <= destination; ++i) {
            if (i > maxReach) {
                return false;
            }
            maxReach = max(maxReach, i + nums[i]);
            if (maxReach >= destination) {
                return true;
            }
        }
        return maxReach >= destination;
    }
};

int main(int argc, char *argv[])
{
    vector<int> testOne = {2,3,1,1,4}, testTwo ={3,2,1,0,4};
    Solution test;
    cout << "Testing the vector {2,3,1,1,4}, should return true. \n" << boolalpha << test.canJump(testOne) << endl;
    cout << "Testing the vector {3,2,1,0,4}, should return false. \n" << boolalpha << test.canJump(testTwo) << endl;

}
