#include"aoc.h"
#include<ranges>
#include<algorithm>
#include<iostream>
#include<vector>
#include<cassert>
#include<cassert>

using namespace std;

namespace {
    constexpr array digits_str = {"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};

    constexpr auto not_digit = [](char c) {
        return !isdigit(c);
    };

    auto const find_first_digit = [](auto sv) -> u32 {
        while(true) {
            if (isdigit(sv[0])) return sv[0] - '0';

            #pragma GCC unroll 10
            for(auto i = 0; i < digits_str.size(); i++) {
                if (sv.starts_with(digits_str[i])) return i + 1;
            }
            sv = sv.substr(1);
        }
    };

    auto const find_second_digit = [](auto sv) -> u32 {
        while(true) {
            if (isdigit(sv.back())) return sv.back() - '0';

            #pragma GCC unroll 10
            for(auto i = 0; i < digits_str.size(); i++) {
                if (sv.ends_with(digits_str[i])) return i + 1;
            }
            sv = sv.substr(0, sv.length() - 1);
        }
    };
}

void aoc::day_01(vector<string>& input) {
    auto const part1 = r::fold_left(
        input |
        rv::transform(
            [](string_view str) -> int {
                auto const first = *r::find_if(str, [](auto&& c) {return isdigit(c);}) - '0';
                auto const second = *r::find_if(str | rv::reverse, [](auto&& c) {return isdigit(c);}) - '0';

                return 10 * first + second;
            }
        ), 0, plus<>()
    );

    auto const part2 = r::fold_left(
        input |
        rv::transform(
            [](string_view str) -> int {
                auto const first = find_first_digit(str);
                auto const second = find_second_digit(str);

                return 10 * first + second;

            }
        ), 0, plus<>()
    );

    assert(part1 == 54644);
    assert(part2 == 53348);

    // cout << "PART I  : " << part1 << endl;
    // cout << "PART II : " << part2 << endl;
}