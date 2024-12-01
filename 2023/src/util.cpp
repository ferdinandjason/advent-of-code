#include"aoc.h"
#include<vector>
#include<string>
#include<ranges>
#include<algorithm>

namespace aoc {
    vector<string> split(string str, string delims = " "){
        vector<string> output;

        for (auto first = str.data(), second = str.data(), last = first + str.size(); second != last && first != last; first = second + 1) {
            second = find_first_of(first, last, cbegin(delims), cend(delims));

            if (first != second)
                output.emplace_back(first, second - first);
        }

        return output;
    }
}
