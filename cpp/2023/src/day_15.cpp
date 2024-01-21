#include"aoc.h"
#include<ranges>
#include<algorithm>
#include<iostream>
#include<cmath>
#include<sstream>
#include<numeric>
#include<list>
#include<cassert>

using namespace std;

namespace {
    void hash_it(int& seed, char const val) {
        seed += static_cast<int>(val);
        seed *= 17;
        seed %= 256;
    }

    int hash_label(string label) {
        int seed = 0;
        for(auto c: label) {
            hash_it(seed, c);
        }

        return seed;
    }

    vector<string> parse_input(string input) {
        return aoc::split(input, ",");
    }

    using hash_table = array<list<pair<string, int>>, 256>;
    hash_table generate_table(vector<string>& commands) {
        hash_table table;

        for(auto command: commands) {
            if (command.back() == '-') {
                auto label = command.substr(0, command.size() - 1);
                int hash = hash_label(label);

                table[hash].remove_if([&](auto&& value) {
                    return value.first == label;
                });

                continue;
            }

            auto sep = command.find('=');

            auto focal = stoi(command.substr(sep + 1));
            auto label = command.substr(0, sep);

            int hash = hash_label(label);

            auto now = &table[hash];
            auto now_iterator = now->begin();
            
            auto found = false;
            while(now_iterator != now->end()) {
                auto [now_label, now_focal] = (*now_iterator);
                if (now_label == label) {
                    now_iterator->second = focal;
                    found = true;
                    break;
                }
                now_iterator++;
            }

            if (!found) {
                now->push_back({label, focal});
            }
        };

        // for(int i = 0; i < 256; i++) {
        //     cout << i << ": ";
        //     for(auto x: table[i]) {
        //         cout << "(" << x.first << " " << x.second << ") ";
        //     }
        //     cout << endl;
        // }

        return move(table);
    }
}

void aoc::day_15(vector<string>& input) {
    auto commands = parse_input(input[0]);

    auto part1 = r::fold_left(
        rv::iota(0, static_cast<int>(commands.size())) | rv::transform([&](auto&& i) -> int {
            return r::fold_left(static_cast<string>(commands[i]), 0, [](int hash, char c) -> int {
                hash_it(hash, c);
                return hash;
            }); 
        }), 0, plus<>()
    );

    auto table = generate_table(commands);

    auto part2 = r::fold_left(
        rv::iota(0, 256) | rv::transform([&](auto&& i) {
            auto now = &table[i];
            auto now_iterator = now->begin();
            auto ret = 0, count = 1;

            while(now_iterator != now->end()) {
                ret += (i + 1) * count * now_iterator->second;

                now_iterator++; count++;
            }

            return ret;
        }), 0, plus<>()
    );

    assert(part1 == 510388);
    assert(part2 == 291774);

    // cout << "PART I  : " << part1 << endl;
    // cout << "PART II : " << part2 << endl;
}
