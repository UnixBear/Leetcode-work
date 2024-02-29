#include <iostream>
#include <array>
#include <tuple>
#include <string>

using namespace std;

class Solution {
public:
    string intToRoman(int num) {
        string answer = "";
        array<tuple<int,string>, 13> numerals = {{
            {1000, "M"}, {900, "CM"}, {500, "D"}, {400, "CD"},
            {100, "C"}, {90, "XC"}, {50, "L"}, {40, "XL"},
            {10, "X"}, {9, "IX"}, {5, "V"}, {4, "IV"}, {1, "I"}
        }};
        for (const auto& numeral : numerals) {
            int value = get<0>(numeral);
            string symbol = get<1>(numeral);
            while (num >= value) {
                answer += symbol;
                num -= value;
            }
        }
        return answer;
    }
};

int main(int argc, char *argv[])
{
    Solution sol;
    cout << "testing integer to roman function on the numbers 3, 58, 1994 and we should get III, LVIII, MCMXCIV." << endl;
    cout << "3: " << sol.intToRoman(3) << endl << "58: " << sol.intToRoman(58) << endl << "1994: " << sol.intToRoman(1994) << endl;
}
