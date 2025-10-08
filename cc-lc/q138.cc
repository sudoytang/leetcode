/*
// Definition for a Node.
class Node {
public:
    int val;
    Node* next;
    Node* random;

    Node(int _val) {
        val = _val;
        next = NULL;
        random = NULL;
    }
};
*/
#include <cassert>
#include <cstdlib>
#include <unordered_map>
class Node {
public:
    int val;
    Node* next;
    Node* random;

    Node(int _val) {
        val = _val;
        next = NULL;
        random = NULL;
    }
};

class Solution {
public:
    Node* copyRandomList(Node* head) {
        std::unordered_map<Node*, Node*> mapping;
        for (auto p = head; p != nullptr; p = p->next) {
            mapping[p] = new Node(p->val);
        }
        for (auto p = head; p != nullptr; p = p->next) {
            auto mapped = mapping[p];
            assert(mapped != nullptr);
            mapped->next = mapping[p->next];
            mapped->random = mapping[p->random];
        }

        return mapping[head];
    }
};