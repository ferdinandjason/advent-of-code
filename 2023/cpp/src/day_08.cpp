#include"aoc.h"
#include<ranges>
#include<algorithm>
#include<iostream>
#include<unordered_map>
#include<map>
#include<sstream>
#include<numeric>
#include<cassert>

using namespace std;

namespace {
    using node_map = unordered_map<int, pair<int, int>>;

    i64 cycle_count(string_view& path, int start, bool part2, node_map& tree) {
        int path_size = path.size();
        for(int i = 0;;i++) {
            auto now = path[i % path_size];

            if (part2 && start % 100 == 25)  return static_cast<i64>(i);
            if (start == 252525) return static_cast<i64>(i);

            if (now == 'L') start = tree[start].first;
            if (now == 'R') start = tree[start].second;
        }

        return -1;
    }

    int perfect_hash(string node) {
        return (node[0] - 'A') * 10000 + (node[1] - 'A') * 100 + (node[2] - 'A');
    }
}

void aoc::day_08(vector<string>& input) {
    auto path = string_view{input[0]};

    auto tree = input | 
        rv::drop(2) | 
        rv::transform([](auto&& line) -> node_map::value_type {
            stringstream ss(line);
            string start, left, right;

            ss >> start >> left >> right;
            return {perfect_hash(start), {perfect_hash(left), perfect_hash(right)}};
        }) | 
        to<node_map>();

    auto const part1 = cycle_count(path, 0, false, tree);

    auto const part2 = r::fold_left(
        tree | 
            rv::filter([](auto&& input) {
                auto [path, _] = input;
                return path % 100 == 0;
            }) | 
            rv::transform([&](auto&& start) {
                return cycle_count(path, start.first, true, tree);
            }),
        1, 
        [](i64 lhs, i64 rhs)->i64 {
            return lcm(lhs, rhs);
        }
    );

    assert(part1 == 18673);
    assert(part2 == 17972669116327);
    
    // cout << "PART I  : " << part1 << endl;
    // cout << "PART II : " << part2 << endl;
}