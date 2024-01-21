#include"aoc.h"
#include<ranges>
#include<algorithm>
#include<iostream>
#include<unordered_map>
#include<queue>
#include<sstream>
#include<charconv>
#include<cassert>

using namespace std;

namespace {
    struct pair_hash {
        size_t operator()(const pair<u64, u64>& p) const {
            size_t seed = 0;
            aoc::hash_combine(seed, p.first);
            aoc::hash_combine(seed, p.second);
            return seed;
        }
    };
    
    using range_map = unordered_map<pair<u64, u64>, pair<u64, u64>, pair_hash>;

    u64 find(const range_map& maps, u64 src) {
        for (auto const& [source, dest] : maps) {
            if (src >= source.first && src <= source.second) {
                return dest.first + (src - source.first);
            }
        }
        return src;
    }

    vector<pair<u64, u64>> find_range(const range_map& maps, const vector<pair<u64 , u64>>& ranges) {
        queue<pair<u64, u64>> q;
        vector<pair<u64, u64>> result;

        for(int i = 0; i < ranges.size(); i++) q.push(ranges[i]);

        auto const is_not_intersect = [](pair<u64, u64> a, pair<u64, u64> b) {
            return (a.second < b.first) || (b.second < a.first);
        };

        while(!q.empty()) {
            auto curr = q.front(); q.pop();
            bool found = false;

            for (auto const& [source, dest] : maps) {
                if (is_not_intersect(source, curr)) continue;

                pair<u64, u64> instersect = {max(source.first, curr.first), min(source.second, curr.second)};

                if (curr.first >= source.first) {
                    if (curr.second > source.second) {
                        q.push({source.second + 1, curr.second});
                    } 
                } else {
                    if (curr.second >= source.first && curr.second <= source.second) {
                        q.push({curr.first, source.first - 1});
                    } else {
                        q.push({curr.first, source.first - 1});
                        q.push({source.second + 1, curr.second});
                    }
                }

                u64 diff_first = (instersect.first - source.first);
                u64 diff_second = (instersect.second - source.second);

                result.push_back({dest.first + diff_first, dest.second + diff_second});
                found = true;
                break;
            }

            if (!found) {
                result.push_back(curr);
            }
        }
        return result;
    }
}

void aoc::day_05(vector<string>& input) {
    auto const maps = input | rv::drop(2) | 
        rv::filter(
            [](string_view line) {
                return line.empty() || r::find_if(line, [](char ch) {return std::isdigit(ch); }) != line.end();
            }
        ) | 
        rv::chunk_by([](const string& str1, const string& str2)->bool { return str1.size() != 0 && str2.size() != 0; }) | 
        rv::transform([](auto&& input) -> vector<string_view> { return input | to<vector<string_view>>(); }) | 
        rv::transform(
            [](auto&& input_maps) -> range_map {
                stringstream ss;
                range_map maps;

                for(auto input: input_maps | rv::filter([](auto&& line){ return line.size() > 0;} )) {
                    u64 dstart, sstart, count;

                    ss << input;
                    ss >> dstart >> sstart >> count;

                    maps[{sstart, sstart + count - 1LL}] = {dstart, dstart + count - 1LL};
                    ss.clear();
                }

                return maps;
            }
        ) | to<vector<range_map>>();

    auto const seeds = aoc::extract_numbers<u64>(input[0]);

    auto const part1 = r::min(
        seeds | rv::transform([&](auto&& seed) { 
            auto temp = seed;
            for(int i = 0; i < maps.size(); i++) {
                temp = find(maps[i], temp);
            }

            return temp;
        })
    );

    auto seeds_range = seeds | rv::chunk(2) | rv::transform([](auto&& seed_and_count) -> pair<u64, u64> {
        return {seed_and_count[0], seed_and_count[0] + seed_and_count[1] - 1};
    }) | to<vector<pair<u64, u64>>>();

    r::for_each(maps, [&](auto map) {
        seeds_range = find_range(map, seeds_range);
    });

    auto part2 = r::min(seeds_range).first;

    assert(part1 == 1181555926);
    assert(part2 == 37806486);

    // cout << "PART I  : " << part1 << endl;
    // cout << "PART II : " << part2 << endl;
}