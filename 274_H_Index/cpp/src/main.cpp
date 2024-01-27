#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

class Solution {
public:
    int hIndex(vector<int>& citations) {
        sort(citations.begin(), citations.end(), greater<int>());
        int tracker = 0;
        for (int i=0;i<citations.size(); ++i){
            int citation = citations[i];
            if (citation >= i+1) {
                tracker = i + 1;
            }
            else {
                break;
            }
        }
        return tracker;
    }
};


int main(int argc, char *argv[])
{
    vector<int> a = { 1, 45, 54, 71, 76, 12 };
    Solution sol;
    sol.hIndex(a);
}
