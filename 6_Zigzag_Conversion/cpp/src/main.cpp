#include <iostream>
#include <string>

using namespace std;

class Solution {
public:
    string convert(string s, int numRows) {
        if (numRows == 1 || s.size() <= numRows) {
            return s;
        }

        string result = "";
        int cycleLen = 2 * numRows - 2;

        for (int row = 0; row < numRows; row++) {
            for (int j = row; j < s.size(); j += cycleLen) {
                result += s[j]; 
                if (row > 0 && row < numRows - 1) {
                    int diagIndex = j + cycleLen - 2 * row;
                    if (diagIndex < s.size()) {
                        result += s[diagIndex];
                    }
                }
            }
        }

        return result;
    }
};



int main(int argc, char *argv[])
{
    cout << "Hello world!" << endl;
    Solution test;
    cout << test.convert("testing", 3);
}