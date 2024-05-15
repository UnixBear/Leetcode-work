#include <iostream>
#include <vector>
#include <unordered_set>
using namespace std;

class Solution {
public:
    bool isValidSudoku(vector<vector<char>>& board) {
        vector<unordered_set<int>> rows(9);
        vector<unordered_set<int>> cols(9);
        vector<unordered_set<int>> subBoxes(9);
        for (int i = 0; i < 9; ++i) {
            for (int j = 0; j < 9; ++j) {
                if (board[i][j] != '.') {
                    int num = board[i][j] - '0'; //converts char to int
                    int boxIndex = (i / 3) * 3 + j / 3;
                    if (rows[i].find(num) != rows[i].end() ||
                        cols[j].find(num) != cols[j].end() ||
                        subBoxes[boxIndex].find(num) != subBoxes[boxIndex].end()) {
                            return false;
                        }
                    rows[i].insert(num);
                    cols[j].insert(num);
                    subBoxes[boxIndex].insert(num);

                }
            }
        }
        return true;
    }
};
int main(int argc, char *argv[])
{
    Solution sol; 
    vector<vector<char>> boardTwo = {
    {'8', '3', '.', '.', '7', '.', '.', '.', '.'},
    {'6', '.', '.', '1', '9', '5', '.', '.', '.'},
    {'.', '9', '8', '.', '.', '.', '.', '6', '.'},
    {'8', '.', '.', '.', '6', '.', '.', '.', '3'},
    {'4', '.', '.', '8', '.', '3', '.', '.', '1'},
    {'7', '.', '.', '.', '2', '.', '.', '.', '6'},
    {'.', '6', '.', '.', '.', '.', '2', '8', '.'},
    {'.', '.', '.', '4', '1', '9', '.', '.', '5'},
    {'.', '.', '.', '.', '8', '.', '.', '7', '9'}
};
    vector<vector<char>> boardOne = {
    {'5', '3', '.', '.', '7', '.', '.', '.', '.'},
    {'6', '.', '.', '1', '9', '5', '.', '.', '.'},
    {'.', '9', '8', '.', '.', '.', '.', '6', '.'},
    {'8', '.', '.', '.', '6', '.', '.', '.', '3'},
    {'4', '.', '.', '8', '.', '3', '.', '.', '1'},
    {'7', '.', '.', '.', '2', '.', '.', '.', '6'},
    {'.', '6', '.', '.', '.', '.', '2', '8', '.'},
    {'.', '.', '.', '4', '1', '9', '.', '.', '5'},
    {'.', '.', '.', '.', '8', '.', '.', '7', '9'}
};
    cout << "testing checker, should true(1) then false(0)." << endl;
    cout << sol.isValidSudoku(boardOne) << endl;
    cout << sol.isValidSudoku(boardTwo) << endl;
}
