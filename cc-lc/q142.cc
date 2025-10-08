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
    ListNode *detectCycle(ListNode *head) {
        if (head == nullptr || head->next == nullptr) {
            return nullptr;
        }
        auto fast = head->next->next;
        auto slow = head->next;

        while (fast != nullptr && slow != nullptr && fast != slow) {
            fast = fast->next;
            if (fast) {
                fast = fast->next;
            } else {
                return nullptr;
            }
            slow = slow->next;
        }

        if (fast == nullptr || slow == nullptr) {
            return nullptr;
        } else {
            // there is a cycle.
            while (fast != head) {
                fast = fast->next;
                head = head->next;
            }
            return head;
        }
    }
};