#include <iostream>
#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    vector<string> testing(vector<string>& words, int maxWidth) {
        vector<string> res;
        vector<string> cur;
        int num_of_letters = 0;

        for (auto& word : words) {
            if (num_of_letters + word.size() + cur.size() > maxWidth) {
                if (cur.size() == 1) {
                    res.push_back(cur[0] + string(maxWidth, ' '));
                }
                else {
                    int total_spaces = maxWidth - num_of_letters;
                    int space_btwn_words = total_spaces / (cur.size() - 1);
                    int extra_spaces = total_spaces % (cur.size()-1);
                    for(int i = 0; i < extra_spaces, ++i) {
                        cur[i] += ' ';
                    }
                    // todo implement append spacesw
                }
            }
        }
        return res;
    }
};

int main(int argc, char *argv[])
{
    Solution sol;
    vector<string> words = {"string", "string"};
    vector<string> result = sol.testing(words, 10);
    cout << (11 / 2) << endl << (11%2);
    return 0;
}
