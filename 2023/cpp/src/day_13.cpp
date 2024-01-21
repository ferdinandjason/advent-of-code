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
    int count_diff(string lhs, string rhs) {
        return  r::count_if(rv::iota(0, static_cast<int>(lhs.size())), [&](int i) -> bool {
            return lhs[i] != rhs[i];
        });
    }

    optional<int> find_reflective_vertical(vector<string>& map, bool include_smudge, int last_x) {
        int n = map.size();
        int m = map[0].size();

        auto xs = rv::iota(0, m) | to<vector<int>>();

        auto find_reflective = [&](int x) -> bool {
            if (x + 1 == last_x) return false;

            int mirror_size = min(x + 1, m - x - 1);
            int x1 = x, x2 = x1 + 1;

            string left, right;
            while(mirror_size--) {
                for(int y = 0; y < n; y++) {
                    left += map[y][x1];
                    right += map[y][x2];
                }

                x1--; x2++;
            }

            auto diff = count_diff(left, right);
            if (include_smudge) {
                return diff <= 1;
            }

            return diff == 0;
        };

        if (auto x = r::find_if(xs, find_reflective); x != xs.end() && *x != m - 1) {
            return *x + 1;
        }

        return {};
    } 

    optional<int> find_reflective_horizontal(vector<string>& map, bool include_smudge, int last_y) {
        int n = map.size();
        int m = map[0].size();

        auto ys = rv::iota(0, n) | to<vector<int>>();

        auto find_reflective = [&](int y) -> bool {
            if (y + 1 == last_y) return false;

            int mirror_size = min(y + 1, n - y - 1);
            int y1 = y, y2 = y + 1;
            
            string left, right;
            while(mirror_size--) {
                left += map[y1];
                right += map[y2];

                y1--; y2++;
            }

            auto diff = count_diff(left, right);
            if (include_smudge) {
                return diff <= 1;
            }

            return diff == 0;
        };

        if (auto y = r::find_if(ys, find_reflective); y != ys.end() && *y != n - 1) {
            return *y + 1;
        }

        return {};
    }

    pair<i64, i64> get_scores(vector<string>& map) {
        auto vertical = find_reflective_vertical(map, false, -1);
        if (vertical) {
            return {*vertical, -1};
        }

        auto horizontal = find_reflective_horizontal(map, false, -1);
        return {-1, *horizontal};
    }

    pair<i64, i64> get_scores_part2(vector<string>& map, pair<int, int> part1_scores) {
        auto vertical = find_reflective_vertical(map, true, part1_scores.first);
        if (vertical) {
            return {*vertical, 0};
        }

        auto horizontal = find_reflective_horizontal(map, true, part1_scores.second);
        return {0, *horizontal};
    }

    i64 get_final_scores(pair<i64, i64> raw_scores) {
        return raw_scores.second * 100 + raw_scores.first;
    }
}

void aoc::day_13(vector<string>& input) {
    auto maps = input | 
        rv::chunk_by([](auto&& str1, auto&& str2) { return str1.size() != 0 && str2.size() != 0; }) | 
        rv::transform([](auto&& map) { return map | to<vector<string>>(); }) | 
        rv::filter([](auto&& map) { return map.size() > 1; }) | 
        to<vector<vector<string>>>();

    auto scores_without_smudge = maps | rv::transform([](auto&& map) {
        return get_scores(map);
    }) | to<vector<pair<i64, i64>>>();

    auto part1_raw_scores = r::fold_left(scores_without_smudge, make_pair(0, 0), [](auto&& lhs, auto&& rhs) -> pair<i64, i64> {
        auto [lhs_ver, lhs_hor] = lhs;
        auto [rhs_ver, rhs_hor] = rhs;

        if (lhs_ver == -1) lhs_ver = 0;
        if (lhs_hor == -1) lhs_hor = 0;
        if (rhs_ver == -1) rhs_ver = 0;
        if (rhs_hor == -1) rhs_hor = 0;

        return {lhs_ver + rhs_ver, lhs_hor + rhs_hor};
    });

    auto part1 = get_final_scores(part1_raw_scores);

    auto part2_raw_scores = r::fold_left(
        maps | rv::enumerate | rv::transform([&](auto&& map_with_index) -> pair<int, int> {
            auto [index, map] = map_with_index;

            return get_scores_part2(map, scores_without_smudge[index]);
        }),
        make_pair(0, 0),
        [](auto&& lhs, auto&& rhs) -> pair<i64, i64> {
            return {lhs.first + rhs.first, lhs.second + rhs.second};
        }
    );

    auto part2 = get_final_scores(part2_raw_scores);

    assert(part1 == 31956);
    assert(part2 == 37617);

    // cout << "PART I  : " << part1 << endl;
    // cout << "PART II : " << part2 << endl;
}
