#include"aoc.h"
#include<ranges>
#include<algorithm>
#include<iostream>
#include<cmath>
#include<sstream>
#include<numeric>
#include<queue>
#include<memory>
#include<map>
#include<unordered_map>
#include<set>
#include<unordered_set>
#include<functional>
#include<random>
#include<cstring>
#include<cassert>

using namespace std;

namespace {
    int perfect_hash(string nodes) {
        return r::fold_left(nodes, 0, [](int acc, char c) {
            return 26 * acc + (c - 'a');
        });
    }

    int node_id(string nodes, int lookup[], vector<int>& hash_nodes) {
        auto node_id = perfect_hash(nodes);

        if (lookup[node_id] == -1) {
            hash_nodes.push_back(node_id);
            lookup[node_id] = hash_nodes.size() - 1;
        }
        return lookup[node_id];
    }

    // Karger's algorithm
    pair<vector<pair<int, int>>, int> parse_input_for_karger(vector<string>& input) {
        vector<pair<int, int>> edges;
        vector<int> hash_nodes;

        int lookup[26 * 26 * 26];
        memset(lookup, -1, sizeof(lookup));
        

        for(auto line: input) {
            auto nodes = aoc::split(line, " ");
            auto start = node_id(nodes[0].substr(0, nodes[0].size() - 1), lookup, hash_nodes);

            for(auto node: nodes | rv::drop(1)) {
                auto next = node_id(node, lookup, hash_nodes);
                edges.push_back({start, next});
            }
        }

        return {edges, hash_nodes.size()};
    }

    int find_set(int node, vector<int>& parent) {
        if (node == parent[node]) return node;
        return parent[node] = find_set(parent[node], parent);
    }

    void union_set(int u, int v, vector<int>& parent, vector<int>& rank_set) {
        u = find_set(u, parent);
        v = find_set(v, parent);

        if (u != v) {
            if (rank_set[u] < rank_set[v]) {
                swap(u, v);
            }

            parent[v] = u;

            if (rank_set[u] == rank_set[v]) rank_set[u]++;
        }
    }

    pair<int, int> karger_min_cut(vector<pair<int, int>>& edges, int v) {
        vector<int> parent(v), rank_set(v, 0);
        int vertices = v;

        for(int i = 0; i < v; i++) {
            parent[i] = i;
        }

        random_device device;
        mt19937 seed(device());
        uniform_int_distribution<mt19937::result_type> rand(0, edges.size() - 1);

        while(vertices > 2) {
            int i = rand(seed);

            int setA = find_set(edges[i].first, parent);
            int setB = find_set(edges[i].second, parent);

            if (setA != setB) {
                union_set(edges[i].first, edges[i].second, parent, rank_set);
                vertices--;
            }
        }

        int cut = 0;
        for(int i = 0; i < edges.size(); i++) {
            int setA = find_set(edges[i].first, parent);
            int setB = find_set(edges[i].second, parent);
            if (setA != setB) cut++;
        }

        unordered_map<int, int> counter;
        for(int i = 0; i < v; i++) {
            counter[parent[i]]++;
        }

        return {cut, r::fold_left(counter | rv::values, 1, multiplies<>())};
    }

    int solve_with_karger(vector<pair<int, int>>& edges, int v) {
        auto [cut, ans] = karger_min_cut(edges, v);
        if (cut == 3) {
            return ans;
        }
        return solve_with_karger(edges, v);
    }

    // Simplified Edmonds–Karp algorithm
    using edges_and_nodes = pair<vector<int>, vector<pair<int, int>>>;
    edges_and_nodes parse_input_for_edmond(vector<string>& input) {
        vector<vector<int>> neighbours(1552, vector<int>());
        vector<int> hash_nodes;

        int lookup[26 * 26 * 26];
        memset(lookup, -1, sizeof(lookup));

        for(auto line: input) {
            auto nodes = aoc::split(line, " ");
            auto start = node_id(nodes[0].substr(0, nodes[0].size() - 1), lookup, hash_nodes);

            for(auto node: nodes | rv::drop(1)) {
                auto next = node_id(node, lookup, hash_nodes);
                neighbours[start].push_back(next);
                neighbours[next].push_back(start);
            }
        }

        vector<int> edges;
        vector<pair<int, int>> nodes;

        for(auto list: neighbours) {
            auto start = edges.size();
            auto end = start + list.size();

            edges.insert(edges.end(), list.begin(), list.end());
            nodes.push_back({start, end});
        }

        return {edges, nodes};
    }

    auto generate_neighbours(edges_and_nodes& input, int node) {
        auto [edges, nodes] = input;
        auto [start, end] = nodes[node];
        return rv::iota(start, end) | rv::transform([&](auto&& edge) -> pair<int, int> {
            return {edge, edges[edge]};
        });
    }

    int find_furthest(edges_and_nodes& input, int start) {
        queue<int> q;
        q.push(start);

        vector<char> seen(input.second.size(), 'f');
        seen[start] = 't';

        int furthest = start;
        while(!q.empty()) {
            auto now = q.front(); q.pop();
            furthest = now;

            for(auto [_, next]: generate_neighbours(input, now)) {
                if (seen[next] == 'f') {
                    seen[next] = 't';
                    q.push(next);
                }
            }
        }

        return furthest;
    }

    int simplified_max_flow(edges_and_nodes& input, int start, int end) {
        auto [edges, nodes] = input;

        queue<pair<int, int>> q;
        vector<pair<int, int>> path;
        vector<char> used(edges.size(), 'f');

        int size = 0;
        int loop = 4;

        while(loop--) {
            size = 0;
            q.push({start, numeric_limits<int>::max()});

            vector<char> seen(nodes.size(), 'f');
            seen[start] = 't';

            while(!q.empty()) {
                auto [now, parent] = q.front(); q.pop();
                size++;

                if (now == end) {
                    while(parent != numeric_limits<int>::max()) {
                        auto [edge, next] = path[parent];
                        used[edge] = 't';
                        parent = next;
                    }

                    break;
                }

                for(auto [edge, next]: generate_neighbours(input, now)) {
                    if (used[edge] == 'f' && seen[next] == 'f') {
                        seen[next] = 't';
                        q.push({next, path.size()});
                        path.push_back({edge, parent});
                    }
                } 
            }

            q = queue<pair<int, int>>();
            path.clear();
        }

        return size;
    }

    int solve_with_simplified_max_flow(edges_and_nodes& input) {
        int start = find_furthest(input, 815);
        int end = find_furthest(input, start);
        int size = simplified_max_flow(input, start, end);

        return size * (input.second.size() - size);
    }
}

void aoc::day_25(vector<string>& input) {
    // auto [edge_list, node_count] = parse_input_for_karger(input);
    // auto part1 = solve_with_karger(edge_list, node_count);

    auto edges_and_nodes = parse_input_for_edmond(input);
    auto part1 = solve_with_simplified_max_flow(edges_and_nodes);

    assert(part1 == 602151);

    // cout << "PART I  : " << part1 << endl;
}
