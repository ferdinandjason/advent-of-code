#include"aoc.h"
#include<ranges>
#include<algorithm>
#include<iostream>
#include<cmath>
#include<sstream>
#include<numeric>
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

    vector<point2d> get_neighbours(vector<string>& map, point2d& start) {
        static const unordered_map<char, pair<point2d, point2d>> char_to_offsets = {
            {'J', { point2d{0,-1}, point2d{-1,0} }},
            {'|', { point2d{-1,0}, point2d{1,0}}},
            {'-', { point2d{0,-1}, point2d{0,1}}},
            {'L', { point2d{-1,0}, point2d{0,1}}},
            {'7', { point2d{0,-1}, point2d{1,0}}},
            {'F', { point2d{1,0}, point2d{0,1}}},
            {'S', { point2d{-1,-1}, point2d{-1,-1}}},
            {',', { point2d{-1,-1}, point2d{-1,-1}}},
        };

        auto neighbours_offset = char_to_offsets.at(map[start.x][start.y]);
        return {
            start + neighbours_offset.first,
            start + neighbours_offset.second,
        };
    }

    char get_start_tile_char(vector<string>& maps, point2d& start) {
        int n = maps.size(), m = maps[0].size();

        vector<point2d> neigh;
        vector<point2d> dd = {{-1, 0}, {0, -1}, {0, 1}, {1, 0}};
        for(auto d: dd) {
            auto now = start + d;
            if (now.x < 0 || now.x >= n || now.y < 0 || now.y >= m) continue;
            
            for(auto next: get_neighbours(maps, now)) {
                if (next == start) {
                    neigh.push_back(start - now);
                }
            }
        }
        static const unordered_map<point2d, char, point2d_hash, point2d_equal> dist_to_tile{
            {{-2, -1}, 'J'}, {{0,-1}, '|'}, {{-1,0}, '-'},
            {{1,-1}, 'L'}, {{-2,1}, '7'}, {{1,2}, 'F'}
        };
        return dist_to_tile.at(neigh.front() + neigh.back());
    }

    tuple<point2d, vector<string>> parse_input(vector<string>& input) {
        point2d start;
        for(int i = 0; i < input.size(); i++) {
            for(int j = 0; j < input[i].size(); j++) {
                if (input[i][j] == 'S') {
                    start = {i, j};
                    goto end;
                }
            }
        }

        end:
        input[start.x][start.y] = get_start_tile_char(input, start);
        return {start, move(input)};
    }

    int solve_part1(vector<string>& maps, point2d& start) {
        unordered_set<point2d, point2d_hash, point2d_equal> seen;

        auto nexts = get_neighbours(maps, start);
        auto next1 = nexts.front();
        auto next2 = nexts.back();

        seen.insert(start);
        seen.insert(next1);
        seen.insert(next2);

        int distance = 1;
        while(next1 != next2) {
            for(auto nextt: get_neighbours(maps, next1)) {
                if (!seen.contains(nextt)) {
                    next1 = nextt;
                }
            }

            for(auto nextt: get_neighbours(maps, next2)) {
                if (!seen.contains(nextt)) {
                    next2 = nextt;
                }
            }

            seen.insert(next1);
            seen.insert(next2);

            distance++;
        }

        return distance;
    }

    int solve_part2(vector<string>& maps, point2d& start) {
        unordered_set<point2d, point2d_hash, point2d_equal> loop;
        int n = maps.size(), m = maps[0].size();

        auto const get_next = [&](point2d current) -> optional<point2d> {
            for(auto next: get_neighbours(maps, current)) {    
                if (!loop.contains(next)) {
                    return next;
                }
            }

            return {};
        };

        point2d current = start;
        do {
            loop.insert(current);
            auto next = get_next(current);
            current = (next) ? *next : start;
        } while(current != start);

        vector<string> new_maps(n, string(m, '.'));
        r::for_each(loop, [&](auto&& point) {
            new_maps[point.x][point.y] = maps[point.x][point.y];
        });

        return r::fold_left(
            new_maps | rv::transform([](string& row) {
                bool in = false;
                int count = 0;

                for(char tile: row) {
                    if (tile == '|' || tile == 'F' || tile == '7') in = !in;
                    else if (in && tile == '.') count++;
                }

                return count;
            }), 0, plus<>()
        );
    }
}

void aoc::day_10(vector<string>& input) {
    auto [start, maps] = parse_input(input);

    auto part1 = solve_part1(maps, start);
    auto part2 = solve_part2(maps, start);

    assert(part1 == 6947);
    assert(part2 == 273);

    // cout << "PART I  : " << part1 << endl;
    // cout << "PART II : " << part2 << endl;
}
