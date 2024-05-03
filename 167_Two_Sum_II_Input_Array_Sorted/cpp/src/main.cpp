#include <iostream>
#include <vector>
using namespace std;

class Solution {
public:
    vector<int> twoSum(vector<int>& numbers, int target) {
        int left = 0;
        int right = numbers.size()-1;
        int sum;

        while (left < right) {
            sum = numbers[right]+numbers[left];
            if (sum == target) {
                return {left+1, right+1};
            }
            else if (sum > target) {
                right--;
            }
            else {
                left++;
            }
        }
        return {};
        
    }
};

int main(int argc, char *argv[])
{
    Solution test;
    vector<int> sampleVec = {2,7,11,15};
    int sampleTarget = 9;
    vector<int> answer = test.twoSum(sampleVec, sampleTarget);
    cout << answer[0] << " " << answer[1];
}
