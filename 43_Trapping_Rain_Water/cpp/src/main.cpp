#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    int trap(vector<int>& height) {
        int* leftptr = height.data();
        int* rightptr = height.data() + height.size() - 1;
        int maxRight = 0, maxLeft = 0, answer = 0;
        while (leftptr < rightptr) {
            if (*leftptr < *rightptr) {
                if (*leftptr >= maxLeft) {
                    maxLeft = *leftptr;
                }
                else {
                    answer += maxLeft - *leftptr;
                }
                leftptr++;
            }
            else {
                if (*rightptr >= maxRight) {
                    maxRight = *rightptr;
                }
                else {
                    answer += maxRight - *rightptr;
                }
                rightptr--;
            }
        }
        return answer;
    }
};

int main(int argc, char *argv[])
{
    Solution sol;
    vector<int> test = {0,1,0,2,1,0,1,3,2,1,2,1};
    std::cout << "Hello world! We will test the set [0,1,0,2,1,0,1,3,2,1,2,1] and we should get 6" << std::endl;
    cout << "The answer is: [" << sol.trap(test) << "]" << endl;
}
