#ifndef _AOC_H_
#define _AOC_H_

#pragma once

#include<vector>
#include<string>
#include<ranges>
#include<numeric>
#include"util.h"

using namespace std;

namespace aoc{
    void do_aoc(int day);
    void bench(int count);

    void day_01(vector<string>& input);
    void day_02(vector<string>& input);
    void day_03(vector<string>& input);
    void day_04(vector<string>& input);
    void day_05(vector<string>& input);
    void day_06(vector<string>& input);
    void day_07(vector<string>& input);
    void day_08(vector<string>& input);
    void day_09(vector<string>& input);
    void day_10(vector<string>& input);
    void day_11(vector<string>& input);
    void day_12(vector<string>& input);
    void day_13(vector<string>& input);
    void day_14(vector<string>& input);
    void day_15(vector<string>& input);
    void day_16(vector<string>& input);
    void day_17(vector<string>& input);
    void day_18(vector<string>& input);
    void day_19(vector<string>& input);
    void day_20(vector<string>& input);
    void day_21(vector<string>& input);
    void day_22(vector<string>& input);
    void day_23(vector<string>& input);
    void day_24(vector<string>& input);
    void day_25(vector<string>& input);

    vector<string> get_input(int day);
}

#endif