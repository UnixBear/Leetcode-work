
#include <vector>
#include <iostream>
#include <string>
#include <map>

using namespace std;

class Solution 
{
    public:
        vector<int> twoSum(vector<int>& nums, int target) 
        {
            map<int,int> hmap;
            vector<int> hpairs;
            for (int i = 0; i < nums.size(); i++)
                {
                    int complement = target - nums[i];
                    if(hmap.find(complement) != hmap.end())
                    {
                        hpairs.push_back(hmap.find(complement)->second);
                        hpairs.push_back(i);
                        break;
                    }
                    hmap.insert(pair<int, int>(nums[i], i));
                }
            return hpairs;

        }
};


int main()
{
    Solution sol;
    vector<int>nums = {1,4,3,6,5,8};
    vector<int>pair = sol.twoSum(nums, 7);
    cout << pair[0] << pair[1];
    return 0;
}