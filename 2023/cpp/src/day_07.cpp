#include"aoc.h"
#include<ranges>
#include<algorithm>
#include<iostream>
#include<unordered_map>
#include<map>
#include<sstream>
#include<cassert>

using namespace std;

namespace {
    unordered_map<char,int> values1 = {{'2', 0},{'3', 1},{'4', 2},{'5', 3},{'6', 4},{'7', 5},{'8', 6},{'9', 7},{'T', 8},{'J', 9},{'Q', 10},{'K', 11},{'A', 12}};
    unordered_map<char,int> values2 = {{'J', 0},{'2', 1},{'3', 2},{'4', 3},{'5', 4},{'6', 5},{'7', 6},{'8', 7},{'9', 8},{'T', 9},{'Q', 10},{'K', 11},{'A', 12}};

    int value1(string& card) {
        unordered_map<char, int> counter;
        for(auto c: card) counter[c]++;

        return r::max(counter | rv::values) - counter.size();
    }

    bool compare_values1(string& a, string& b) {
        for(int i = 0; i < a.size(); i++) {
            int aval2 = values1[a[i]];
            int bval2 = values1[b[i]];
            if (aval2 != bval2) {
                return aval2 < bval2;
            }
        }
        return false;
    }

    int value2(string& card) {
        unordered_map<char, int> counter;
        for(auto c: card) counter[c]++;

        bool is_joker = false;
        if (auto it = counter.find('J'); it != counter.end()) {
            auto joker_count = it->second; it->second = numeric_limits<int>::min();

            auto max_fcard = r::max_element(counter, [](auto&& a, auto&& b) {
                return a.second < b.second;
            });

            if (max_fcard->first == 'J') {
                max_fcard->second = joker_count;
            } else {
                max_fcard->second += joker_count;
                is_joker = true;
            }   
        }

        return r::max(counter | rv::values) - (counter.size() - is_joker);
    }

    bool compare_values2(string& a, string& b) {
        for(int i = 0; i < a.size(); i++) {
            int aval2 = values2[a[i]];
            int bval2 = values2[b[i]];
            if (aval2 != bval2) {
                return aval2 < bval2;
            }
        }
        return false;
    }
}

void aoc::day_07(vector<string>& input) {
    auto card_bid = input | rv::transform([](auto&& line) -> tuple<string, int, int, int> {
        stringstream ss(line);
        string card; int bid;

        ss >> card >> bid;

        return {card, value1(card), value2(card), bid};
    }) | to<vector<tuple<string, int, int, int>>>();

    r::sort(card_bid, [](auto&& a, auto&&b) {
        int aval = get<1>(a), bval = get<1>(b);
        if (aval == bval) {
            return compare_values1(get<0>(a), get<0>(b));
        }

        return aval < bval;
    });

    auto part1 = r::fold_left(
        card_bid |
        rv::enumerate | 
        rv::transform([](auto&& index_hand) -> i64 {
            auto [index, card_bid] = index_hand;
            return static_cast<i64>(index + 1) * static_cast<i64>(get<3>(card_bid));
        }),
        0, plus<>()
    );

    r::sort(card_bid, [](auto&& a, auto&&b) {
        int aval = get<2>(a), bval = get<2>(b);
        if (aval == bval) {
            return compare_values2(get<0>(a), get<0>(b));
        }

        return aval < bval;
    });

    auto part2 = r::fold_left(
        card_bid |
        rv::enumerate | 
        rv::transform([](auto&& index_hand) -> i64 {
            auto [index, card_bid] = index_hand;
            return static_cast<i64>(index + 1) * static_cast<i64>(get<3>(card_bid));
        }),
        0, plus<>()
    );

    assert(part1 == 250058342);
    assert(part2 == 250506580);

    // cout << "PART I  : " << part1 << endl;
    // cout << "PART II : " << part2 << endl;
}