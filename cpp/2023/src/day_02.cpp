#include"aoc.h"
#include<ranges>
#include<algorithm>
#include<iostream>
#include<vector>
#include<string_view>
#include<cassert>

using namespace std;

namespace {
    struct max_rgp_input_parser {
        string_view buffer;

        // public
        int id = 0;
        int r = 0, g = 0, b = 0;

        // private
        int i = 0;
        int r_ = 0, g_ = 0, b_ = 0;

        int extract_number() {
            auto count = 0;
            auto n = r::fold_left(
                buffer | 
                    rv::drop(i) |
                    rv::take_while([&](const char c) {
                        return isdigit(c);
                    }),
                0,
                [&](int acc, char now) {
                    count++;
                    return acc * 10 + (now - '0');
                }
            );

            i += count;
            return n;
        }

        int skip_color_name(char color_code) const {
            if (color_code == 'r') return 2;
            if (color_code == 'g') return 4;
            if (color_code == 'b') return 3;
            return 0;
        }

        void set_color_value(int num, char color_code) {
            if (color_code == 'r') r_ = num;
            if (color_code == 'g') g_ = num;
            if (color_code == 'b') b_ = num;
        }

        void count_max() {
            r = max(r, r_ );
            g = max(g, g_);
            b = max(b, b_);

            r_ = 0, g_ = 0, b_ = 0;
        }

        void parse() {
            i += 5;
            id = extract_number();
            i += 2;

            while(i < buffer.size()) {
                while(!isdigit(buffer[i])) i++;
                auto const num = extract_number();
                auto const cc = buffer[i + 1];
                i += 2;
                i += skip_color_name(cc);
                set_color_value(num, cc);

                if (buffer[i] == ',') i++;
                if (buffer[i] == ';') {
                    count_max();
                }
            }

            count_max();
        }

        tuple<int, int, int, int> get_max_color_with_id() {
            parse();
            return {id, r, g, b};
        }
    };

    tuple<int, int, int> parse_input(string& input) {
        max_rgp_input_parser parser{string_view{input}};
        auto [game_id, max_r, max_g, max_b] = parser.get_max_color_with_id();
        return {game_id, max_r * max_g * max_b, max_r <= 12 && max_g <= 13 && max_b <= 14};   
    }
}

void aoc::day_02(vector<string>& input) {
    auto parsed_input = input | rv::transform(parse_input);
    
    auto part1 = r::fold_left(
        parsed_input |
            rv::filter([](auto&& game) { return get<2>(game); }) |
            rv::transform( [](auto&& game) { return get<0>(game); }), 
        0, plus<>()
    );

    auto part2 = r::fold_left(
        parsed_input |
            rv::transform([](auto&& game) -> int { return get<1>(game); }), 
        0, plus<>()
    );

    assert(part1 == 2600);
    assert(part2 == 86036);

    // cout << "PART I  : " << part1 << endl;
    // cout << "PART II : " << part2 << endl;
}
