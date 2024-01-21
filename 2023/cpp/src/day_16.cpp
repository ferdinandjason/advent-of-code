#include"aoc.h"
#include<ranges>
#include<algorithm>
#include<iostream>
#include<cmath>
#include<sstream>
#include<numeric>
#include<queue>
#include<unordered_map>
#include<unordered_set>
#include<cassert>

using namespace std;

namespace {
    struct point2d {
        int x, y;
    };

    point2d operator*(int k, const point2d& p) {
        return { k * p.x, k * p.y };
    }

    point2d operator+(const point2d& p1, const point2d& p2) {
        return {
            p1.x + p2.x,
            p1.y + p2.y
        };
    }

    point2d operator-(const point2d& p1, const point2d& p2) {
        return {
            p1.x - p2.x,
            p1.y - p2.y
        };
    }

    struct point2d_equal {
        bool operator()(const point2d& p1, const point2d& p2) const {
            return p1.x == p2.x && p1.y == p2.y;
        }
    };

    bool operator==(const point2d& p1, const point2d& p2) {
        return point2d_equal()(p1, p2);
    }

    struct point2d_hash {
        size_t operator()(const point2d& p) const {
            size_t seed = 0;
            aoc::hash_combine(seed, p.x);
            aoc::hash_combine(seed, p.y);
            return seed;
        }
    };

    enum direction {
        north = 0,
        west,
        south,
        east
    };

    point2d dir_to_offset(direction dir) {
        static const array<point2d, 4> offsets = {
            point2d{-1, 0},
            point2d{0, -1},
            point2d{1, 0},
            point2d{0, 1},
        };

        return offsets[dir];
    }

    vector<direction> char_dir_to_dirs(char from, direction dir_from) {
        static const unordered_map<char, array<vector<direction>, 4>> dirs_map = {
            {'|', 
                {vector<direction>
                    {north},        // north
                    {north, south}, // west
                    {south},        // south
                    {north, south}, // east
                }
            },
            {'-',
                {vector<direction>
                    {east, west}, // north
                    {west},       // west
                    {east, west}, // south
                    {east},       // east
                }
            },
            {'\\',
                {vector<direction>
                    {west},  // north
                    {north}, // west
                    {east},  // south
                    {south}, // east
                }
            },
            {'/',
                {vector<direction>
                    {east},  // north
                    {south}, // west
                    {west},  // south
                    {north}, // east
                }
            },
            {'.',
                {vector<direction>
                    {north}, // north
                    {west},  // west
                    {south}, // south
                    {east},  // east
                }
            },
        };

        return dirs_map.at(from)[dir_from];
    }

    using state = pair<point2d, direction>;
    struct state_hash {
        size_t operator()(const state& p) const {
            size_t seed = 0;
            aoc::hash_combine(seed, point2d_hash{}(p.first));
            aoc::hash_combine(seed, static_cast<int>(p.second));
            return seed;
        }
    };

    using memo_energized = unordered_map<point2d, int, point2d_hash>;

    bool is_in_edge(vector<string>& maps, point2d start) {
        auto [x, y] = start;
        return x == 0 || x == maps.size() || y == 0 || y == maps[0].size();
    }

    int simulate(vector<string>& maps, point2d start, direction dir, memo_energized& energized_count) {
        if (energized_count.find(start) != energized_count.end()) {
            return energized_count[start];
        }

        int n = maps.size(), m = maps[0].size();

        unordered_set<state, state_hash> seen;

        queue<state> q;
        q.push({start, dir});
        seen.insert({start, dir});

        vector<point2d> exit;

        while(!q.empty()) {
            auto [pos, dir] = q.front(); q.pop();
            auto next = pos + dir_to_offset(dir);

            if (is_in_edge(maps, pos)) {
                exit.push_back(pos);
            }

            if (next.x < 0 || next.x >= n || next.y < 0 || next.y >= m) continue;
            
            for(auto next_dir: char_dir_to_dirs(maps[next.x][next.y], dir)) {
                if (seen.contains({next, next_dir})) continue;

                seen.insert({next, next_dir});
                q.push({next, next_dir});
            }
        }

        auto energized = seen | rv::transform([](auto&& state) {return state.first;}) | to<unordered_set<point2d, point2d_hash>>();
        int count = energized.size();

        for(auto point: exit) {
            energized_count[point] = count;
        }
        return count;
    }

    vector<state> generate_start_state(vector<string>& maps) {
        int n = maps.size(), m = maps[0].size();
        vector<state> start_state;

        for(int i = 0; i < m; i++) {
            start_state.push_back({point2d{0, i}, south});
            start_state.push_back({point2d{n, i}, north});
        }

        for(int i = 1; i < n - 1; i++) {
            start_state.push_back({point2d{i, 0}, east});
            start_state.push_back({point2d{i, n - 1}, west});
        }

        return start_state;
    }
}

void aoc::day_16(vector<string>& input) {
    memo_energized energized_count;

    auto part1 = simulate(input, point2d{0, 0}, east, energized_count);

    auto part2 = r::max(
        generate_start_state(input) | rv::transform([&](auto&& state_input) {
            return simulate(input, state_input.first, state_input.second, energized_count);
        })
    );

    assert(part1 == 7434);
    assert(part2 == 8183);

    // cout << "PART I  : " << part1 << endl;
    // cout << "PART II : " << part2 << endl;
}
