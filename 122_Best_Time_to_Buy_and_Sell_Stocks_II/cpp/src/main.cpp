#include <iostream>
#include <vector>

class Solution {
public:
    int maxProfit(std::vector<int>& prices) {
        int profit_total = 0;
        for(int i = 0; i < prices.size() - 1; ++i){
            if (prices[i] < prices[i+1]){
                profit_total += prices[i+1] - prices[i];
            }
        }
        return profit_total;
    }
};

int main(int argc, char *argv[])
{
    Solution sol;
    std::vector<int> test = {7,1,5,3,6,4};
    std::cout << "Correct answer should be: 7" << std::endl;
    std::cout << "Return value is: " << sol.maxProfit(test) << std::endl;
}
