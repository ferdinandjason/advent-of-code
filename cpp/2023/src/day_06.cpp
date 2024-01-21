#include"aoc.h"
#include<ranges>
#include<algorithm>
#include<iostream>
#include<cmath>
#include<sstream>
#include<cassert>

using namespace std;

namespace {
    double discriminant(double t, double d) {
        return sqrt(t * t - 4 * d);
    }

    i64 way_to_win(pair<i64, i64> td) {
        auto [time, distance] = td;
        auto d = discriminant(time, distance);
        auto min_x = (static_cast<double>(time) - d) / 2.0;
        auto dx = ceil(min_x) - min_x;

        return (dx == 0)? static_cast<i64>(d) - 1 : static_cast<i64>(floor(d - dx)) + 1;
    }
}

void aoc::day_06(vector<string>& input) {
    auto time_distances = input | rv::transform([](auto&& line) -> pair<i64, i64> {
        stringstream ss(line);
        i64 time, distance;
        ss >> time >> distance;

        return {time, distance};
    }) | to<vector<pair<i64, i64>>>() ;

    auto part1 = r::fold_left(time_distances | rv::transform(way_to_win), 1, multiplies<>());

    stringstream stime, sdistance;
    for(auto& [t, d]: time_distances) {
        stime << t;
        sdistance << d;
    }

    auto part2 = way_to_win({stoll(stime.str()), stoll(sdistance.str())});

    assert(part1 == 449820);
    assert(part2 == 42250895);
    
    // cout << "PART I  : " << part1 << endl;
    // cout << "PART II : " << part2 << endl;
}