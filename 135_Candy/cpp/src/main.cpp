#include <iostream>
#include <vector>

using namespace std;

class Solution
{
public:
    int candy(vector<int> &ratings)
    {
        if (ratings.empty())
        {
            return 0;
        }

        int ret = 1, up = 0, down = 0, peak = 0;
        for (int i = 0; i < ratings.size() - 1; ++i)
        {
            int previous = ratings[i];
            int current = ratings[i + 1];
            if (previous < current)
            {
                up += 1;
                down = 0;
                peak = up;
                ret += 1 + up;
            }
            else if (previous == current)
            {
                up = down = peak = 0;
                ret += 1;
            }
            else
            {
                up = 0;
                down = down + 1;
                ret += 1 + down - (peak >= down);
            }
        }
        return ret;
    }
};

int main(int argc, char *argv[])
{
    Solution sol;
    vector<int> vec_one = {1, 0, 2};
    vector<int> vec_two = {1, 2, 2};
    cout << "Hello World! We'll test the candy function on two vectors [1,0,2] and [1,2,2] and their respective results should be 5 and 4." << endl;
    cout << "Results: [" << sol.candy(vec_one) << "] [" << sol.candy(vec_two) << "]";

    return 0;
}
