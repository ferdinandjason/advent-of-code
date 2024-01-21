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

    direction reverse_dir(direction dir) {
        if (dir == north) return south;
        if (dir == south) return north;
        if (dir == east) return west;
        if (dir == west) return east;
        return north; // impossible
    }

    struct state {
        point2d pos;
        direction dir;
        int streak;

        void operator = (const state &other) {
            this->pos = other.pos;
            this->dir = other.dir;
            this->streak = other.streak;
        }

        bool operator<(const state& other) const {
            auto value = [](state p) {
                return p.pos.x * 100000000 + p.pos.y * 100000 + p.dir * 1000 + p.streak;
            };
            return value(*this) < value(other);
        }

        bool operator == (const state &p) const {
            return this->pos == p.pos && this->dir == p.dir && this->streak == p.streak;
        }
    };

    struct state_hash {
        size_t operator()(const state& p) const {
            size_t seed = 0;
            aoc::hash_combine(seed, point2d_hash{}(p.pos));
            aoc::hash_combine(seed, static_cast<int>(p.dir));
            aoc::hash_combine(seed, p.streak);
            return seed;
        }
    };

    // -------------- PART 1 -------------- //
    int solve_part1(vector<string>& maps) {
        int n = maps.size(), m = maps[0].size();

        unordered_map<state, int, state_hash> distances;
        unordered_set<state, state_hash> seen;

        using queue_state = pair<int, state>;
        priority_queue<queue_state, vector<queue_state>, greater<queue_state>> pq;

        pq.push({0, state{{0, 0}, south, 0}});
        pq.push({0, state{{0, 0}, east, 0}});
        seen.insert(state{{0, 0}, south, 0});
        seen.insert(state{{0, 0}, east, 0});

        while(!pq.empty()) {
            auto [cost, node] = pq.top(); pq.pop();

            if (node.pos == point2d{n-1, m-1}) {
                return cost;
            }

            for(int dir = 0; dir < 4; dir++) {
                if(dir == reverse_dir(node.dir)) continue;

                point2d next = node.pos + dir_to_offset(static_cast<direction>(dir));
                if (next.x < 0 || next.x >= n || next.y < 0 || next.y >= m) continue;

                state next_state;
                if (dir != node.dir) next_state = state{next, static_cast<direction>(dir), 1};
                else if (node.streak < 3) next_state = state{next, static_cast<direction>(dir), node.streak + 1};
                else continue;

                if (distances.find(next_state) == distances.end()) distances[next_state] = numeric_limits<int>::max();

                if (cost + (maps[next.x][next.y] - '0') < distances[next_state]) {
                    distances[next_state] = cost + (maps[next.x][next.y] - '0');
                    pq.push({distances[next_state], next_state});
                }
            }
        }

        return -1;
    }

    // -------------- PART 2 -------------- //
    int solve_part2(vector<string>& maps) {
        int n = maps.size(), m = maps[0].size();

        unordered_map<state, int, state_hash> distances;
        unordered_set<state, state_hash> seen;

        using queue_state = pair<int, state>;
        priority_queue<queue_state, vector<queue_state>, greater<queue_state>> pq;

        pq.push({0, state{{0, 0}, south, 0}});
        pq.push({0, state{{0, 0}, east, 0}});
        seen.insert(state{{0, 0}, south, 0});
        seen.insert(state{{0, 0}, east, 0});

        while(!pq.empty()) {
            auto [cost, node] = pq.top(); pq.pop();

            if (node.pos == point2d{n-1, m-1} && node.streak >= 4) {
                return cost;
            }

            for(int dir = 0; dir < 4; dir++) {
                if(dir == reverse_dir(node.dir)) continue;

                point2d next = node.pos + dir_to_offset(static_cast<direction>(dir));
                if (next.x < 0 || next.x >= n || next.y < 0 || next.y >= m) continue;

                state next_state;
                if (dir != node.dir && node.streak >= 4) next_state = state{next, static_cast<direction>(dir), 1};
                else if (dir == node.dir && node.streak < 10) next_state = state{next, static_cast<direction>(dir), node.streak + 1};
                else continue;

                if (distances.find(next_state) == distances.end()) distances[next_state] = numeric_limits<int>::max();

                if (cost + (maps[next.x][next.y] - '0') < distances[next_state]) {
                    distances[next_state] = cost + (maps[next.x][next.y] - '0');
                    pq.push({distances[next_state], next_state});
                }
            }
        }

        return -1;
    }
}

void aoc::day_17(vector<string>& input) {
    auto part1 = solve_part1(input);
    auto part2 = solve_part2(input);

    assert(part1 == 1138);
    assert(part2 == 1312);

    // cout << "PART I  : " << part1 << endl;
    // cout << "PART II : " << part2 << endl;
}
