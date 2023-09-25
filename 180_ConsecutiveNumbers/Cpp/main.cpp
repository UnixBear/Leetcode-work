#include <iostream>
#include <vector>
#include <set>

std::set<int> find_consecutive_triple_or_more(const std::vector<int>& vec) {
    std::set<int> results;
    for (size_t i = 0; i < vec.size() - 2; ++i) {
        if (vec[i] + 1 == vec[i+1] && vec[i] + 2 == vec[i+2]) {
            results.insert(vec[i]);
            // Check for more consecutive numbers in sequence
            size_t offset = 3;
            while (i + offset < vec.size() && vec[i] + offset == vec[i+offset]) {
                results.insert(vec[i+offset-1]);
                ++offset;
            }
        }
    }
    return results;
}

int main() {
    std::vector<int> vec = {1, 2, 3, 4, 5, 6, 8, 9, 10, 11, 13, 14};
    std::set<int> result = find_consecutive_triple_or_more(vec);
    
    if(!result.empty()) {
        std::cout << "Found 3 or more consecutive values starting with: ";
        for (const int& val : result) {
            std::cout << val << " ";
        }
        std::cout << std::endl;
    } else {
        std::cout << "No 3 or more consecutive values found." << std::endl;
    }

    return 0;
}