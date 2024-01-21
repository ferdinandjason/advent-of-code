#include"aoc.h"
#include<vector>
#include<functional>
#include<format>
#include<fstream>
#include<iostream>
#include<chrono>
#include<ranges>
#include<algorithm>
#include<cmath>

using namespace std;

void aoc::do_aoc(int day) {
    const static auto aoc = vector<function<void(vector<string>&)>>{
        day_01,
        day_02,
        day_03,
        day_04,
        day_05,
        day_06,
        day_07,
        day_08,
        day_09,
        day_10,
        day_11,
        day_12,
        day_13,
        day_14,
        day_15,
        day_16,
        day_17,
        day_18,
        day_19,
        day_20,
        day_21,
        day_22,
        day_23,
        day_24,
        day_25,
    };

    auto index = day - 1;
    if (index < aoc.size()) {
        auto input = aoc::get_input(day);
        auto t_start = chrono::high_resolution_clock::now();
        aoc[index](input);
        auto t_end = chrono::high_resolution_clock::now();
        
        printf("elapsed time = %lf ms\n", std::chrono::duration<double, std::milli>(t_end-t_start).count());
    }
}

void aoc::bench(int count) {
    const static auto aoc = vector<function<void(vector<string>&)>>{
        day_01,
        day_02,
        day_03,
        day_04,
        day_05,
        day_06,
        day_07,
        day_08,
        day_09,
        day_10,
        day_11,
        day_12,
        day_13,
        day_14,
        day_15,
        day_16,
        day_17,
        day_18,
        day_19,
        day_20,
        day_21,
        day_22,
        day_23,
        day_24,
        day_25,
    };

    auto global_mean_time = vector<double>();
    auto global_min_time = vector<double>();
    auto global_std_time = vector<double>();

    for(int i = 0; i < aoc.size(); i++) {
        printf("Benchmark Day %d ...", i + 1);
        
        auto input = aoc::get_input(i + 1);
        auto local_time = vector<double>();

        int count_temp = count;
        while(count_temp--) {
            auto temp_input = input;
            auto t_start = chrono::high_resolution_clock::now();
            aoc[i](temp_input);   
            auto t_end = chrono::high_resolution_clock::now();

            local_time.push_back(std::chrono::duration<double, std::milli>(t_end-t_start).count());
        }

        printf("done\n");
        
        auto mean = ranges::fold_left(local_time, 0, plus<>()) / static_cast<double>(local_time.size()); 
        auto min = ranges::min(local_time);
        auto std = sqrt(
            ranges::fold_left(
                local_time | rv::transform([&](auto&& x) {
                    return (x - mean) * (x - mean);
                }), 0, plus<>()
            ) / static_cast<double>(local_time.size())
        );

        global_mean_time.push_back(mean);
        global_min_time.push_back(min);
        global_std_time.push_back(std);
    }

    printf("| Day | Benchmark AVG (ms) | Benchmark Min (ms) | Benchmark Stdev (ms) | \n");
    printf("| --- | :-: | :-: | :-: |\n");
    for(int i = 0; i < aoc.size(); i++) {
        printf("| %2d | `%3.3lf` | `%3.3lf` | `%3.3lf` | \n", i + 1, global_mean_time[i], global_min_time[i], global_std_time[i]);
    }
    printf("| **Total** | `%3.3lf` | `%3.3lf` | - |\n", r::fold_left(global_mean_time, 0, plus<>()), r::fold_left(global_min_time, 0, plus<>()));
}

vector<string> aoc::get_input(int day) {
    vector<string> input;
    string path = format("./input/day_{:02}.txt", day), line;

    ifstream fs(path);
    while (getline(fs, line)) {
        input.push_back(line);
    }

    return input;
}
