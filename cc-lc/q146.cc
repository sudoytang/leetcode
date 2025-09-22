#include <list>
#include <unordered_map>
class LRUCache {
    struct Node {
        int k;
        int v;
    };
    int capacity {};
    int size = 0;
    std::list<Node> nodes {};
    std::unordered_map<int, std::list<Node>::iterator> map {};
public:
    LRUCache(int capacity) {
        this->capacity = capacity;
    }

    int get(int key) {
        if (auto it = map.find(key); it != map.end()) {
            // prepend this node
            nodes.splice(nodes.begin(), nodes, it->second);
            return it->second->v;
        } else {
            return -1;
        }
    }

    void put(int key, int value) {
        if (auto it = map.find(key); it != map.end()) {
            // prepend this node
            nodes.splice(nodes.begin(), nodes, it->second);
            it->second->v = value;
            return;
        }
        if (nodes.size() == capacity) {
            auto [k, v] = nodes.back();
            nodes.pop_back();
            map.erase(k);
        }
        nodes.push_front({key, value});
        map.insert({key, nodes.begin()});
    }
};

/**
 * Your LRUCache object will be instantiated and called as such:
 * LRUCache* obj = new LRUCache(capacity);
 * int param_1 = obj->get(key);
 * obj->put(key,value);
 */