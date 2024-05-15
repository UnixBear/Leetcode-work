#include <iostream>
#include <vector>
#include <sstream>

using namespace std;

class Solution {
public:
    string reverseWords(string s) {
        istringstream iss(s);
        vector<string> words;
        string word;

        while (iss >> word) {
            words.push_back(word);
        }

        ostringstream oss;
        if (!words.empty()) {
            for (int i = words.size() - 1; i > 0; --i) {
                oss << words[i] << " ";
            }
            oss << words[0];
        }
        return oss.str();
    }
};
int main(int argc, char *argv[])
{
    std::cout << "Hello world!" << std::endl;
}
