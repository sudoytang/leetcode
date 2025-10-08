#include "listnode.hpp"
#include <unordered_set>
class Solution {
public:
    ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
        std::unordered_set<ListNode*> set;
        for (auto p = headA; p != nullptr; p = p->next) {
            set.insert(p);
        }
        for (auto p = headB; p != nullptr; p = p->next) {
            if (!set.insert(p).second) {
                // already exists
                return p;
            }
        }
        return nullptr;
    }
};