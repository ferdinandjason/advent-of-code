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

    vector<i64> count_tiles(vector<string>& maps, point2d start, set<int> steps) {
        int n = maps.size(), m = maps[0].size();
        int max_steps = (*steps.rbegin());

        array<point2d, 4> offset = {point2d{-1, 0}, {0, -1}, {1, 0}, {0, 1}};

        unordered_map<point2d, int, point2d_hash> seen;

        queue<pair<point2d, int>> q;
        q.push({start, 0});
        seen[start] = 0;

        while(!q.empty()) {
            auto [pos, count] = q.front(); q.pop();

            for(int i = 0; i < 4; i++) {
                auto next = pos + offset[i];

                if (next.x < 0 || next.x >= n || next.y < 0 || next.y >= m) continue;
                if (maps[next.x][next.y] == '#') continue;
                if (count + 1 > max_steps) continue;
                if (seen.find(next) != seen.end()) continue;

                seen[next] = count + 1;
                q.push({next, count + 1});
            }
        }

        vector<int> count_tiles_counter(max_steps + 1);
        r::for_each(seen | rv::values, [&](auto&& count) {
            count_tiles_counter[count]++;
        });

        for(int i = 2; i < count_tiles_counter.size(); i++) {
            count_tiles_counter[i] += count_tiles_counter[i-2];
        }

        return rv::transform(steps, [&](auto&& step) -> i64 {
            return static_cast<i64>(count_tiles_counter[step]);
        }) | to<vector<i64>>();
    }
    
    i64 count_tiles_in_expanded_maps(vector<string>& maps, i64 n = 202300) {
        auto full = count_tiles(maps, {65, 65}, {131, 132});

        auto rmc = count_tiles(maps, {65, 0},   {130})[0];
        auto lmc = count_tiles(maps, {65, 130}, {130})[0];
        auto tmc = count_tiles(maps, {130, 65}, {130})[0];
        auto bmc = count_tiles(maps, {0, 65},   {130})[0];
        

        auto tr = count_tiles(maps, {130, 0},   {64, 195});
        auto tl = count_tiles(maps, {130, 130}, {64, 195});
        auto bl = count_tiles(maps, {0, 130},   {64, 195});
        auto br = count_tiles(maps, {0, 0},     {64, 195});

        i64 fullc = (n * n * full[1]) + ((n - 1) * (n - 1) * full[0]);
        i64 trc = n * tr[0] + (n - 1) * tr[1];
        i64 tlc = n * tl[0] + (n - 1) * tl[1];
        i64 blc = n * bl[0] + (n - 1) * bl[1];
        i64 brc = n * br[0] + (n - 1) * br[1];

        return fullc + trc + tlc + blc + brc + rmc + lmc + tmc + bmc;
    }
}

void aoc::day_21(vector<string>& input) {
    auto part1 = count_tiles(input, {65, 65}, {64})[0];
    auto part2 = count_tiles_in_expanded_maps(input);

    assert(part1 == 3722);
    assert(part2 == 614864614526014);

    // cout << "PART I  : " << part1 << endl;
    // cout << "PART II : " << part2 << endl;
}
