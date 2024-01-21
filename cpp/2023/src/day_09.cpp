#include"aoc.h"
#include<ranges>
#include<algorithm>
#include<iostream>
#include<cmath>
#include<sstream>
#include<numeric>
#include<cassert>

using namespace std;

namespace {
    // all of the input is enough with binom 21
    vector<i64> compute_binom_21() {
        vector<i64> ret;
        ret.push_back(static_cast<i64>(1));

        for(int k = 1, n = 21; k <= n; k++) {
            ret.push_back(
                ret.back() * (n + 1 - k) / k
            );
        }

        return move(ret);
    }
}

void aoc::day_09(vector<string>& input) {
    auto parsed_input = input | rv::transform([](auto&& line) {
        return aoc::extract_numbers<i64>(line);
    }) | to<vector<vector<i64>>>();

    auto const binom22 = compute_binom_21();

    auto const sign = [](int k) {
        if (k % 2) return -1;
        return 1;
    };

    auto answer = r::fold_left(
        parsed_input | rv::transform([&](auto&& input) -> pair<i64, i64> {
            i64 part1 = 0LL, part2 = 0LL;
            for(auto [index, value]: input | rv::enumerate) {
                part1 += (value * binom22[index] * sign(index));
                part2 += value * binom22[index + 1] * sign(index + 1);
            }

            return {part1, part2 * -1};
        }),
        make_pair(0LL, 0LL),
        [](pair<i64, i64> a, pair<i64, i64> b) -> pair<i64, i64> {
            return {a.first + b.first, a.second + b.second};
        }
    );

    assert(answer.first == 1987402313);
    assert(answer.second == 900);

    // cout << "PART I  : " << answer.first << endl;
    // cout << "PART II : " << answer.second << endl;
}