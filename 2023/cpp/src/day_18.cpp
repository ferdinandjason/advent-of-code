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
    struct plan {
        char dir;
        i64 num;

        string hex;

        char dir_p2;
        i64 num_p2;
    };

    static unordered_map<char, pair<int, int>> offset_udlr = {
        {'U', {-1, 0}},
        {'D', {1, 0}},
        {'L', {0, -1}},
        {'R', {0, 1}},
    };

    static unordered_map<char, pair<int, int>> offset_3120 = {
        {'3', {-1, 0}},
        {'1', {1, 0}},
        {'2', {0, -1}},
        {'0', {0, 1}},
    };

    vector<plan> parse_input(vector<string>& input) {
        return input | rv::transform([](auto&& instruction) -> plan {
            char dir; i64 num; char hex[10];
            sscanf(instruction.c_str(), "%c %lld (%s)", &dir, &num, hex);

            auto temp_hex = string(hex).substr(1, 6);
            
            return {
                dir, 
                num, 
                temp_hex, 
                temp_hex.substr(5, 1)[0],
                stoll("0x" + temp_hex.substr(0, 5), nullptr, 16),
            };
        }) | to<vector<plan>>();
    }

    pair<vector<pair<i64, i64>>, i64>  generate_points_part1(vector<plan> input) {
        vector<pair<i64, i64>> points = {{0, 0}};
        auto perimeter = i64(0);
        for(auto curr: input) {
            points.push_back({
                points.back().first + offset_udlr[curr.dir].first * curr.num,
                points.back().second + offset_udlr[curr.dir].second * curr.num
            });

            perimeter += curr.num;
        }

        return {points, perimeter};
    }
    
    pair<vector<pair<i64, i64>>, i64> generate_points_part2(vector<plan> input) {
        vector<pair<i64, i64>> points = {{0, 0}};
        auto perimeter = i64(0);
        for(auto curr: input) {
            points.push_back({
                points.back().first + offset_3120[curr.dir_p2].first * curr.num_p2,
                points.back().second + offset_3120[curr.dir_p2].second * curr.num_p2
            });

            perimeter += curr.num_p2;
        }

        return {points, perimeter};
    }

    i64 calculate_area(pair<vector<pair<i64, i64>>, i64> points_and_perimeter) {
        auto [points, perimeter] = points_and_perimeter;
        
        auto area = r::fold_left(
            points | rv::slide(2) | rv::transform([](auto&& pairwise_xy) -> i64 {
                auto [x1, y1] = pairwise_xy[0];
                auto [x2, y2] = pairwise_xy[1];

                return (x1 * y2) - (x2 * y1);
            }), 0, plus<>()
        );

        if (area < 0) {
            area = -area;
        }

        return area / 2 + perimeter / 2 + 1;
    }
}

void aoc::day_18(vector<string>& input) {
    auto parsed_input = parse_input(input);

    auto part1 = calculate_area(generate_points_part1(parsed_input));
    auto part2 = calculate_area(generate_points_part2(parsed_input));

    assert(part1 == 50746);
    assert(part2 == 70086216556038);

    // cout << "PART I  : " << part1 << endl;
    // cout << "PART II : " << part2 << endl;
}
