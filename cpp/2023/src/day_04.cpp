#include"aoc.h"
#include<ranges>
#include<algorithm>
#include<iostream>
#include<unordered_map>
#include<unordered_set>
#include<cmath>
#include<cstring>
#include<cassert>

using namespace std;

namespace {
    struct FenwickTreeX {
        vector<int> bit1;
        vector<int> bit2;
        int n;

        FenwickTreeX(int n) {
            this->n = n + 1;
            bit1.assign(n + 1, 0);
            bit2.assign(n + 1, 0);
        }

        void add(vector<int> &bit, int idx, int val) {
            for (++idx; idx < n; idx += idx & -idx)
                bit[idx] += val;
        }

        int sum(vector<int> &bit, int idx) {
            int ret = 0;
            for (++idx; idx > 0; idx -= idx & -idx)
                ret += bit[idx];
            return ret;
        }

        void range_add(int l, int r, int val) {
            add(bit1, l, val);
            add(bit1, r+1, -val);
            add(bit2, l, val*(l-1));
            add(bit2, r+1, -val*r);
        }

        int prefix_sum(int idx) {
            return sum(bit1, idx)*idx -  sum(bit2, idx);
        }

        int range_sum(int l, int r) {
            return prefix_sum(r) - prefix_sum(l - 1);
        }
    };

    struct card_parser {
        string_view buffer;

        // private
        int i = 0, mode = 0, point = 0;
        bool numbers[100];

        int extract_number() {
            return r::fold_left(
                buffer | rv::drop(i) | rv::take(2),
                0,
                [](int acc, char now) {
                    if (now == ' ') return acc;
                    return acc * 10 + (now - '0');
                }
            );
        }

        void add_number(int mode, int num) {
            if (mode == 0) numbers[num] = true;
            else if (numbers[num]) point++;
        }

        int get_point() {
            memset(numbers, false, sizeof(numbers));

            i += 10;
            while(i < buffer.size()) {
                add_number(mode, extract_number());
                i += 3;

                if (buffer[i] == '|') {
                    mode = 1;
                    i += 2;
                }
            }

            return point;
        }
    };

    int parse_input(string_view input) {
        card_parser parser{input};
        return parser.get_point();
    }
}

void aoc::day_04(vector<string>& input) {
    auto const winning_number_count = input | rv::transform(parse_input);

    auto const part1 = r::fold_left(
        winning_number_count | rv::transform([](auto&& point) {
            return static_cast<u64>(1) << (point - 1);
        }), 0, plus<>()
    );

    FenwickTreeX tree(input.size() + 5);
    tree.range_add(1, input.size() + 1, 1);

    for(size_t i = 0; i < winning_number_count.size(); i++) {
        auto x = tree.range_sum(i+1, i+1);
        tree.range_add(i + 2, i + winning_number_count[i] + 1, x);
    }

    auto const part2 = tree.range_sum(1, input.size());

    assert(part1 == 18653);
    assert(part2 == 5921508);

    // cout << "PART I  : " << part1 << endl;
    // cout << "PART II : " << part2 << endl;
}