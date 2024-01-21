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
#include<bitset>
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

    struct graph_input {
        vector<u64> directed;
        vector<u64> undirected;
        vector<vector<int>> weight; 
        int extra;
        int start;
        int end;
    };

    graph_input parse_input(vector<string>& maps) {
        int n = maps.size(), m = maps[0].size();

        maps[0][1] = '#';
        maps[n - 1][m - 2] = '#';

        auto start = point2d{1, 1};
        auto end = point2d{n - 2, m - 2};

        maps[start.x][start.y] = 'X';
        maps[end.x][end.y] = 'X';

        static const array<point2d, 4> offsets = {point2d{-1, 0},{0, -1},{1, 0},{0, 1}};

        unordered_map<point2d, int, point2d_hash> pois;
        pois[start] = 0;
        pois[end] = 1;

        for(int i = 0; i < n; ++i) {
            for(int j = 0; j < m; ++j) {
                if (maps[i][j] != '#') {
                    auto now = point2d{i, j};
                    auto neighbour_count = 0;
                    for(int k = 0; k < 4; k++) {
                        auto next = now + offsets[k];
                        if (maps[next.x][next.y] == '#') continue;
                        neighbour_count++;
                    }

                    if (neighbour_count > 2) {
                        maps[now.x][now.y] = 'X';
                        pois[now] = pois.size();
                    }
                }
            }
        }
        
        auto get_direction = [&](char chr) {
            switch(chr) {
                case '^': return 0;
                case '<': return 1;
                case 'v': return 2;
                case '>': return 3;
            }

            return -1;
        };

        int N = pois.size();

        vector<u64> directed(N, 0LL);
        vector<u64> undirected(N, 0LL);
        vector<vector<int>> weight(N, vector<int>(N, 0));

        for(auto [start, from]: pois) {
            queue<tuple<point2d, int, int>> q;
            q.push({start, 0, true});

            while(!q.empty()) {
                auto [now, cost, forward] = q.front(); q.pop();

                for(int k = 0; k < 4; k++) {
                    auto next = now + offsets[k];
                    
                    auto& cur = maps[next.x][next.y];
                    if (cur == '#') continue;
                    else if (cur == 'X') {
                        auto to = pois[next];
                        if (to == from) continue;

                        if (forward) {
                            directed[from] |= 1LL << to;
                        } else {
                            directed[to] |= 1LL << from;
                        }

                        undirected[from] |= 1LL << to;
                        undirected[to] |= 1LL << from;

                        weight[from][to] = cost + 1;
                        weight[to][from] = cost + 1;
                    } else if (cur == '.') {
                        q.push({next, cost + 1, forward});
                        maps[next.x][next.y] = '#';
                    } else {
                        auto same = k == get_direction(cur);
                        q.push({next, cost + 1, forward && same});
                        maps[next.x][next.y] = '#';
                    }
                }
            }
        }

        auto startp = countr_zero(undirected[0]);
        auto endp = countr_zero(undirected[1]);

        auto extra = 2 + weight[0][startp] + weight[1][endp];

        // auto mask = static_cast<u64>(0);
        // for(auto [i, edges]: undirected | rv::enumerate) {
        //     if (popcount(static_cast<u64>(edges)) < 4) {
        //         mask |= (1LL << i);
        //     }
        // }

        // for(int i = 0; i < undirected.size(); i++) {
        //     if (popcount(static_cast<u64>(undirected[i])) < 4) {
        //         undirected[i] = (undirected[i] & !mask) | directed[i];
        //     }
        // }

        return {
            directed,
            undirected,
            weight,
            extra,
            startp, endp,
        };
    }

    int solve_part1(graph_input& input) {
        vector<int> cost(36, 0);

        queue<u64> q;
        q.push(input.start);

        while(!q.empty()) {
            auto from = q.front(); q.pop();
            auto nodes = input.directed[from];
            while(nodes > 0) {
                auto to = countr_zero(nodes);
                auto mask = 1LL << to;
                nodes ^= mask;

                cost[to] = max(cost[to], cost[from] + input.weight[from][to]);
                q.push(to);
            }
        }

        return cost[input.end] + input.extra;
    }

    int dfs(graph_input& input, int from, u64 seen) {
        if (from == input.end) return 0;

        auto nodes = input.undirected[from] & ~seen;
        auto result = 0;

        while(nodes > 0) {
            auto to = countr_zero(nodes);
            auto mask = 1LL << to;
            nodes ^= mask;

            result = max(result, input.weight[from][to] + dfs(input, to, seen | mask));
        }

        return result;
    }

    int solve_part2(graph_input& input) {
        return dfs(input, input.start, 1LL << input.start) + input.extra;
    }
}

void aoc::day_23(vector<string>& input) {
    auto graph_input = parse_input(input);

    auto part1 = solve_part1(graph_input);
    auto part2 = solve_part2(graph_input);

    assert(part1 == 2094);
    assert(part2 == 6442);

    // cout << "PART I  : " << part1 << endl;
    // cout << "PART II : " << part2 << endl;
}
