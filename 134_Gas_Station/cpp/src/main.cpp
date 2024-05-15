#include <iostream>
#include <vector>
#include <numeric>

using namespace std;

class Solution {
public:
    int canCompleteCircuit(vector<int>& gas, vector<int>& cost) {
        if (accumulate(cost.begin(), cost.end(),0) > accumulate(gas.begin(),gas.end(), 0)) {
            return -1;
        }
        int totalGas = 0;
        int totalCost = 0;
        int tracker = 0;
        for (int i = 0; i < static_cast<int>(gas.size()); ++i){
            totalGas += gas[i];
            totalCost += cost[i];
            if (totalCost > totalGas) {
                totalCost = 0;
                totalGas = 0;
                tracker = (i + 1) % gas.size();
            }
        }
        return tracker;
    }
};


int main(int argc, char *argv[])
{
    vector<int> gas = {1,2,3,4,5};
    vector<int> cost = {3,4,5,1,2};
    cout << "testing the vectors [1,2,3,4,5] [3,4,5,1,2], should return 3." << endl;
    Solution sol;
    cout << "answer is: " << sol.canCompleteCircuit(gas, cost);
}
