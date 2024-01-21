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
    struct state_hash {
        size_t operator()(const pair<i64, pair<i64, i64>>& p) const {
            size_t seed = 0;
            aoc::hash_combine(seed, p.first);
            aoc::hash_combine(seed, p.second.first);
            aoc::hash_combine(seed, p.second.second);
            return seed;
        }
    };

    using state_umap = unordered_map<pair<i64, pair<i64, i64>>, i64, state_hash>;
    i64 solve(state_umap& memo, string& input, vector<int>& group, int i, int j, int group_count) {
        if (auto value = memo.find({group_count, {i, j}}); value != memo.end()) {
            return value->second;
        }

        if (i == input.size() && j == group.size()) return 1;

        i64 count = 0LL;
        if (input[i] == '#' || input[i] == '?') {
            // continue the pattern
            count += solve(memo, input, group, i+1, j, group_count + 1);
        }

        if (input[i] == '.' || input[i] == '?') {
            // stop the pattern
            if (group_count != 0 && group[j] != group_count) {
                return count;
            }
            count += solve(memo, input, group, i+1, j+(group_count != 0), 0);
        }

        return memo[{group_count, {i, j}}] = count;
    }

    vector<pair<string, vector<int>>> parse_input(vector<string>& input) {
        return input | rv::transform([](auto&& line) -> pair<string, vector<int>> {
            auto sep_pos = line.find(" ");
            return {
                line.substr(0, sep_pos),
                aoc::split(line.substr(sep_pos + 1), ",") | rv::transform([](auto&& num_str) {
                    return stoll(num_str);
                }) | to<vector<int>>()
            };
        }) | to<vector<pair<string, vector<int>>>>();
    }
}

void aoc::day_12(vector<string>& input) {
    auto parsed_input = parse_input(input);

    auto part1 = r::fold_left(
        parsed_input | rv::transform([](auto&& input) {
            state_umap memo; 
            auto [pattern, group] = input;
            pattern += ".";

            return solve(memo, pattern, group, 0, 0, 0);
        }), 0, plus<>()
    );

    auto part2 = r::fold_left(
        parsed_input | rv::transform([](auto&& input) {
            state_umap memo;
            auto [pattern, group] = input;

            string modified_pattern;
            vector<int> modified_group;
            for(int i = 0; i < 5; i++) {
                modified_pattern += pattern + "?";
                modified_group.insert(modified_group.end(), group.begin(), group.end());
            }

            modified_pattern.back() = '.';

            return solve(memo, modified_pattern, modified_group, 0, 0, 0);
        }), 0, plus<>()
    );

    assert(part1 == 7032);
    assert(part2 == 1493340882140);

    // cout << "PART I  : " << part1 << endl;
    // cout << "PART II : " << part2 << endl;
}
