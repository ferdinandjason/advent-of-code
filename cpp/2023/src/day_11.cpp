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
    using pii = pair<i64, i64>;

    i64 manhattan_distance(pii a, pii b) {
        return abs(a.first - b.first) + abs(a.second - b.second);
    }

    i64 solve(vector<string> maps, i64 scale) {
        int n = maps.size(), m = maps[0].size();
        
        auto x = maps | rv::transform([&](auto&& row) -> i64 {
            if (row.find('#') != string::npos) {
                return 1;
            }
            return scale;
        }) | to<vector<i64>>();

        auto y = rv::iota(0, m) | rv::transform([&](auto&& col) ->i64 {
            auto column = rv::iota(0, n) | rv::transform([&](auto&& row) {
                return maps[row][col];
            }) | to<string>();

            if (column.find('#') != string::npos) {
                return 1;
            }
            return scale;
        }) | to<vector<i64>>();

        for(int i = 1; i < x.size(); i++) x[i] += x[i - 1];
        for(int i = 1; i < y.size(); i++) y[i] += y[i - 1];

        vector<pii> stars;
        for(int i = 0; i < maps.size(); i++) {
            for(int j = 0; j < maps[i].size(); j++) {
                if (maps[i][j] == '#') {
                    stars.push_back({x[i], y[j]});
                }
            }
        }

        i64 sum = 0LL, count = stars.size();
        for(int i = 0; i < count; i++) {
            for(int j = i + 1; j < count; j++) {
                sum += manhattan_distance(stars[i], stars[j]);
            }
        }

        return sum;
    }
}

void aoc::day_11(vector<string>& input) {
    auto part1 = solve(input, 2);
    auto part2 = solve(input, 1000000);

    assert(part1 == 9627977);
    assert(part2 == 644248339497);

    // cout << "PART I  : " << part1 << endl;
    // cout << "PART II : " << part2 << endl;
}
