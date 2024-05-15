#include <iostream>
#include <algorithm>
#include <vector>

using namespace std;

class Solution {
public:
    int jump(vector<int>& nums) {
        size_t destination = nums.size();
        int currentReach = 0;
        int nextReach = 0;
        int jumps = 0;
        if (destination <= 1) {
            return 0;
        }
        for (size_t index = 0; index < destination; ++index) {
            nextReach = max(nextReach, static_cast<int>(index) + nums[index]);
            if (index == currentReach){
                jumps++;
                currentReach = nextReach;
                if (currentReach >= destination-1) {
                    return jumps;
            }
            }

        }
        return jumps;
    }
};

int main(int argc, char *argv[])
{
    Solution tester;
    vector<int> test = {2,3,1,1,4};
    int answer = tester.jump(test);
    cout << "Testing the vector [2,3,1,1,4] which should yield 2." << endl;
    cout << "The answer is: " << answer;
}
