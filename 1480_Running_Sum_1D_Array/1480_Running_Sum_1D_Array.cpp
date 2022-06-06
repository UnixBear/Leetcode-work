#include <vector>
#include <iostream>
using namespace std;
class Solution {
public:
    vector<int> runningSum(vector<int>& nums) {
        for(int i=1; i < nums.size(); i++) {
            nums[i] += nums[i-1];
        }
        return nums;
    }
};

int main() {
    Solution test;
    vector<int> numVect {1,2,3,4};
    test.runningSum(numVect);
    for (auto i: numVect){
        cout << i << ' ';
    }
    return 0;
}