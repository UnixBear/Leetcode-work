#include <iostream>
#include <regex>
#include <string>
class Solution {
public:
    bool isPalindrome_obsolete(std::string s) {
        std::regex pattern("[\\W_]+");
        std::string result = std::regex_replace(s, pattern, "");
        std::transform(result.begin(), result.end(), result.begin(),
            [](unsigned char c){return std::tolower(c);});
        std::string revResult = result;
        std::reverse(revResult.begin(), revResult.end());
        if (result == revResult) {
            return true;
        }
        else {
            return false;
        }
    }
    bool isPalindrome(std::string s) {
        std::string filtered;
        filtered.reserve(s.size()); //avoiding reallocations
        for (char character : s) {
            if (std::isalnum(character)) {
                filtered.push_back(std::tolower(character));
            }
        }
        int start = 0;
        int end = filtered.length() - 1;
        while(start < end) {
            if (filtered[start] != filtered[end]){
                return false;
            }
            ++start;
            --end;
        }
        return true;
    }
};
int main(int argc, char *argv[])
{
    Solution test;
    std::string testString = "A man, a plan, a canal: Panama";
    std::cout << test.isPalindrome(testString);

}
