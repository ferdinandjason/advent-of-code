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
#include<cassert>

using namespace std;

namespace {
    struct point2d {
        int x = 0, y = 0;
    };

    struct point2d_hash {
        size_t operator()(const point2d& p) const {
            size_t seed = 0;
            aoc::hash_combine(seed, p.x);
            aoc::hash_combine(seed, p.y);
            return seed;
        }
    };

    struct point2d_equal {
        bool operator()(const point2d& p1, const point2d& p2) const {
            return p1.x == p2.x && p1.y == p2.y;
        }
    };

    bool operator==(const point2d& p1, const point2d& p2) {
        return point2d_equal()(p1, p2);
    }

    struct point3d {
        int x = 0, y = 0, z = 0;

        point2d xy() {
            return {x, y};
        }
    };

    struct box {
        point3d u, v;
    };

    vector<box> parse_input(vector<string>& input) {
        auto boxs = input | rv::transform([](auto&& line) {
            auto nums = aoc::extract_numbers<int>(line);

            return box{
                {min(nums[0], nums[3]), min(nums[1], nums[4]), min(nums[2], nums[5])},
                {max(nums[0], nums[3]), max(nums[1], nums[4]), max(nums[2], nums[5])},
            };
        }) | to<vector<box>>();
  
        sort(boxs.begin(), boxs.end(), [](auto&& a, auto&& b) {
            return a.u.z < b.u.z;
        });

        return boxs;
    }

    vector<point3d> point_in_box(box b) {
        vector<point3d> result;

        int dx = b.v.x - b.u.x;
        int dy = b.v.y - b.u.y;
        int dz = b.v.z - b.u.z;

        if (dx != 0)
            for(int x = b.u.x; x <= b.v.x; x++) {
                result.push_back({x, b.u.y, b.u.z});
            }
        else if (dy != 0)
            for(int y = b.u.y; y <= b.v.y; y++) {
                result.push_back({b.u.x, y, b.u.z});
            }
        else if (dz != 0)
            for(int z = b.u.z; z <= b.v.z; z++) {
                result.push_back({b.u.x, b.u.y, z});
            }
        else result.push_back(b.u);

        return result;
    }

    pair<vector<unordered_set<int>>, vector<unordered_set<int>>> simulate(const vector<box>& boxs) {
        unordered_map<int, unordered_map<point2d, int, point2d_hash>> on_ground;

        vector<unordered_set<int>> below(boxs.size());
        vector<unordered_set<int>> upper(boxs.size());

        for(auto [i, box]: boxs | rv::enumerate) {
            auto z0 = box.u.z;
            auto point = point_in_box(box);

            while(z0 > 1) {
                bool can_fall = true;
                for(auto p3d: point) {
                    if (p3d.z == z0) {
                        auto below_cube = on_ground[z0 - 1].find(p3d.xy());
                        if (below_cube != on_ground[z0 - 1].end()) {
                            below[i].insert(below_cube->second);
                            upper[below_cube->second].insert(i);
                            
                            can_fall = false;
                        }
                    }
                }

                if (!can_fall) break;

                for(auto& p3d: point) {
                    p3d.z--;
                }
                z0--;
            }

            for(auto& p3d: point) {
                on_ground[p3d.z][p3d.xy()] = i;
            };            
        }

        return {below, upper};
    }

    vector<vector<int>> build_graph(int n, vector<unordered_set<int>>& upper) {
        vector<vector<int>> graph(n);
        for(auto [i, box_up]: upper | rv::enumerate) {
            for(auto box: box_up) {
                graph[i].push_back(box);
            }
        }

        return graph;
    }

    int required_box_count(vector<unordered_set<int>> below) {
        unordered_set<int> required_box_ids;

        for(auto supported_by: below) {
            if (supported_by.size() == 1) {
                required_box_ids.insert(*supported_by.begin());
            }
        }

        return required_box_ids.size();
    }
}

void aoc::day_22(vector<string>& input) {
    auto boxs = parse_input(input);
    auto [below, upper] = simulate(boxs);
    int n = boxs.size();

    auto part1 = n - required_box_count(below);
    
    auto graph = build_graph(n, upper);
    auto count = [&](int v) -> int {
        vector<int> indegree(n, 0);

        for(auto i: rv::iota(0, n)) {
            for(auto x: graph[i]) {
                indegree[x]++;
            }
        }

        int ret = 0;

        queue<int> q;
        q.push(v);
        while(!q.empty()) {
            auto u = q.front(); q.pop();
            ret++;

            for(auto x: graph[u]) {
                indegree[x]--;

                if (!indegree[x]) {
                    q.push(x);
                }
            }
        }

        return ret - 1;
    };

    auto part2 = r::fold_left(
        rv::iota(0, n) | rv::transform([&](auto&& i) { return count(i); }),
        0, plus<>()
    );

    assert(part1 == 424);
    assert(part2 == 55483);

    // cout << "PART I  : " << part1 << endl;
    // cout << "PART II : " << part2 << endl;
}
