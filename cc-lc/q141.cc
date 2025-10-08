/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */

#include "listnode.hpp"
class Solution {
public:
    bool hasCycle(ListNode *head) {
        if (head == nullptr || head->next == nullptr) {
            return false;
        }
        auto fast = head->next->next;
        auto slow = head->next;

        while (fast != nullptr && slow != nullptr && fast != slow) {
            fast = fast->next;
            if (fast) {
                fast = fast->next;
            } else {
                return false;
            }
            slow = slow->next;
        }

        if (fast == nullptr || slow == nullptr) {
            return false;
        } else {
            return true;
        }
    }
};