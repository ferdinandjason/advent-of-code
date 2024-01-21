#include"aoc.h"
#include<string>
#include<iostream>

using namespace std;

int main(int argc, char* argv[]) {
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);

    auto arg = argv[1];
    if (string(arg) == "bench") {
        auto count = stoi(string(argv[2]));
        aoc::bench(count);
        return 0;
    }

    auto day = stoi(argv[1]);
    aoc::do_aoc(day);

    return 0;
}
