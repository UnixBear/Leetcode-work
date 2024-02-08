#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> productExceptSelf(vector<int>& nums) {
        int product = 1;
        int zeroes = 0;
        vector<int> answers;
        for (int i: nums) {
            if (i != 0) {
                product *= i;
            }
            else {
                zeroes++;
            }
        }
        for (int i: nums) {
            if (zeroes > 1) answers.emplace_back(0);
            else if (zeroes == 1) answers.emplace_back(i == 0 ? product : 0);
            else answers.emplace_back(product/i);
        }
        return answers;
    }
};

int main(int argc, char *argv[])
{
    Solution tester;
    vector<int> testvec1 = {1,2,3,4};
    vector<int> answer1 = {24,12,8,6};
    vector<int> testvec2 = {-1,1,0,-3,3};


    cout << "the first test on the vector {1,2,3,4} should show true if it returns {24,12,8,6}" << endl;
    cout << (tester.productExceptSelf(testvec1) == answer1)<< endl;

}
