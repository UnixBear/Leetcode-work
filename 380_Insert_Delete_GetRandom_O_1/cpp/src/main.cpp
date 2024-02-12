#include <iostream>
#include <vector>
#include <unordered_map>
using namespace std;

class RandomizedSet {
public:
    vector<int> values;
    unordered_map<int,int> indexMap;

    RandomizedSet() {
        
    }
    bool search(int val) {
        if(indexMap.find(val)!= indexMap.end()) {
            return true;
        }
        else {
            return false;
        }
    }
    
    bool insert(int val) {
        if(search(val)) {
            return false;
        }
        else {
            values.push_back(val);
            indexMap[val] = values.size()-1;
            return true;
        }
    }
    
    bool remove(int val) {
        if(!search(val)) {
            return false;
        }
        else {
            auto it = indexMap.find(val);
            values[it->second] = values.back();
            values.pop_back();
            indexMap[values[it->second]]=it->second;
            indexMap.erase(val);
            return true;

        }
        
    }
    
    int getRandom() {
        return values[rand()%values.size()];
    }
};

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * RandomizedSet* obj = new RandomizedSet();
 * bool param_1 = obj->insert(val);
 * bool param_2 = obj->remove(val);
 * int param_3 = obj->getRandom();
 */

int main(int argc, char *argv[])
{
    std::cout << "Hello world!" << std::endl;
}
