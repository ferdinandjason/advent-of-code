#include"aoc.h"
#include<ranges>
#include<algorithm>
#include<unordered_map>
#include<unordered_set>
#include<iostream>
#include<cassert>

using namespace std;

namespace {
    constexpr pair<int, int> d[8] = {
        {-1, -1}, {-1, 0}, {-1, 1},
        {0, -1}, {0, 1},
        {1, -1}, {1, 0}, {1, 1},
    };

    pair<int, int> get_parts(vector<string> &input, int x, int y, vector<string>& seen) {
        int n = input.size(), m = input[0].size();
        auto const number = [&](int x, int y) {
            int si = y;
            while(si >= 0 & isdigit(input[x][si])) si--;

            int ret = 0;
            for(int i = si + 1; i < m && isdigit(input[x][i]); i++) {
                seen[x][i] = 1;
                ret = ret * 10 + (input[x][i] - '0');
            }

            return ret;
        };


        vector<int> gears;

        #pragma GCC unroll 8
        for(int i = 0; i < 8; i++) {
            int nx = x + d[i].first;
            int ny = y + d[i].second;

            if (nx < 0 || nx >= n) continue;
            if (ny < 0 || ny >= m) continue;

            if (input[nx][ny] == '.') continue;

            if (isdigit(input[nx][ny]) && !seen[nx][ny]) {
                gears.push_back(number(nx, ny));
            }
        }

        int size = gears.size();

        return r::fold_left(
            gears, pair<int, int>{0, 1}, [&](pair<int, int> acc, int a) -> pair<int, int> {
                return {acc.first + a, acc.second * a * (size == 2)};
            }
        );
    }
}

void aoc::day_03(vector<string>& input) {
    int n = input.size(), m = input[0].size();

    vector<pair<bool, pair<int, int>>> gears;
    vector<string> seen(m, string(n, 0));
    for(int i = 0; i < n; i++) {
        for(int j = 0; j < m; j++) {
            if (!isdigit(input[i][j]) && input[i][j] != '.') {
                gears.push_back({input[i][j] == '*', get_parts(input, i, j, seen)});
            }
        }
    }

    auto const part1 = r::fold_left(
        gears | rv::transform([](auto&& gear) { return gear.second.first; }),
        0, plus<>()
    );

    auto const part2 = r::fold_left(
        gears | 
            rv::filter([&](auto&& gear) { 
                return gear.first;
            }) |
            rv::transform([](auto&& gear) {
                return gear.second.second;
            }),
        0, plus<>()
    );

    assert(part1 == 509115);
    assert(part2 == 75220503);

    // cout << "PART I  : " << part1 << endl;
    // cout << "PART II : " << part2 << endl;
}