#include <iostream>
struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
public:
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        //head is used as a stand in node that we will ignore
        //after we get our linked list
        ListNode* head = new ListNode(0);
        ListNode* tail = head;
        int remainder = 0;
        while (l1 != nullptr || l2 != nullptr || remainder != 0) {
            //set up digits from linked lists to values if there
            //are values or 0 if the list points to null
            int digit1 = (l1 != nullptr) ? l1->val : 0;
            int digit2 = (l2 != nullptr) ? l2->val : 0;

            //artithmetic, generate sum and carry over for next operation
            int sum = digit1 + digit2 + remainder;
            int digit = sum % 10; 
            remainder = sum / 10;

            //create a new node with our digit and have it point to null
            ListNode* newNode = new ListNode(digit);

            //update tail pointer to new node's tail
            //update tail to be our new node
            tail->next = newNode;
            tail = tail->next;

            //update our current nodes to the next ones
            l1 = (l1 != nullptr) ? l1->next: nullptr;
            l2 = (l2 != nullptr) ? l2->next: nullptr;
        }
        
        //create a pointer to the list from our head for 
        //correct answer
        ListNode *answer = head->next;
        
        //head is no longer needed so it is deleted
        delete head;
        return answer;
    }
};


int main(int argc, char *argv[])
{
    
}
