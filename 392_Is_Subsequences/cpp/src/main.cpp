#include <iostream>
#include <string>
#include <algorithm>
using namespace std;

class Solution {
public:
    bool isSubsequence(string s, string t) {
        int position_s = 0, position_t = 0; //keeps track of position for sliding window
        while (position_t < t.length() && position_s < s.length()) { // makes sure we don't overshoot
            if (s[position_s] == t[position_t]) {
                position_s++; //we want to increment s whenever we find something 
            }
            position_t++; //we always increment t since we're transversing it
        }
        return position_s == s.length(); //if equal, all chars passed
    }
    bool isSubsequence_outdated(string s, string t) {
        auto t_iterator = t.begin();
        for (char ch : s) {
            t_iterator = find(t_iterator, t.end(), ch);
            if (t_iterator == t.end()) {
                return false;
            }
            ++ t_iterator;
        }
        return true;
    }
};
int main(int argc, char *argv[])
{  
    Solution test;
    std::string s = "ace";
    std::string t = "abcde";

    if (test.isSubsequence(s, t)) {
        std::cout << s << " is a subsequence of " << t << std::endl;
    } else {
        std::cout << s << " is not a subsequence of " << t << std::endl;
    }

    return 0;
}
