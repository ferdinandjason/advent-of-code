#include"aoc.h"
#include<ranges>
#include<algorithm>
#include<iostream>
#include<cmath>
#include<sstream>
#include<numeric>
#include<queue>
#include<unordered_map>
#include<unordered_set>
#include<cassert>

using namespace std;

namespace {
    struct rule {
        char part;
        bool less_than;
        int value;
        string to;
    };

    rule parse_rules(string& rules) {
        auto separator = rules.find(":");
        if (separator == string::npos) {
            return {'\0', false, 0, rules};
        }

        auto to = rules.substr(separator + 1);
        auto condition = rules.substr(0, separator);

        bool less_than = condition.find('<') != string::npos;
        auto lhs_and_rhs = aoc::split(condition, (less_than)? "<" : ">");
        return {
            lhs_and_rhs.front()[0],
            less_than,
            stoi(lhs_and_rhs.back()),
            to,
        };
    }

    using workflow_umap = unordered_map<string, vector<rule>>;
    workflow_umap::value_type parse_workflow(string& line) {
        auto name_and_rule_sep = line.find('{');
        string name = line.substr(0, name_and_rule_sep);
        string rules_string = line.substr(name_and_rule_sep + 1, line.size() - name.size() - 2);

        return {
            name,
            aoc::split(rules_string, ",") | rv::transform(parse_rules) | to<vector<rule>>()
        };
    }

    struct part {
        int x, m, a, s;

        int operator[](char chr) const {
            if (chr == 'x') return x;
            if (chr == 'm') return m;
            if (chr == 'a') return a;
            if (chr == 's') return s;

            return 0;
        }
    };

    part parse_part(string& line) {
        line = line.substr(1, line.size() - 2);
        auto values = aoc::split(line, ",") | rv::transform([](auto&& line) {
            return stoi(line.substr(line.find("=") + 1));
        }) | to<vector<int>>();

        return {
            values[0],
            values[1],
            values[2],
            values[3],
        };
    }

    pair<workflow_umap, vector<part>> parse_input(vector<string> input) {
        auto iter = r::find_if(input, [](auto&& line) {return line.empty();});

        return {
            r::subrange(input.begin(), iter) | rv::transform(parse_workflow) | to<workflow_umap>(),
            r::subrange(next(iter), input.end()) | rv::transform(parse_part) | to<vector<part>>(),
        };
    }

    string process_rule(rule& rule, part& part) {
        if (!rule.part) {
            return rule.to;
        }

        if (rule.less_than && part[rule.part] < rule.value) {
            return rule.to;
        } else if (!rule.less_than && part[rule.part] > rule.value) {
            return rule.to;
        } else {
            return "";
        }
    }

    string process_workflow(vector<rule>& rules, part& part) {
        for(auto rule: rules) {
            auto result = process_rule(rule, part);
            if (!result.empty()) {
                return result;
            }
        }

        return "";
    }

    bool is_accepted(workflow_umap& workflow, part& part) {
        string verdict = "in";
        while(verdict != "A" && verdict != "R") {
            verdict = process_workflow(workflow[verdict], part);
        }
        return verdict == "A";
    }

    using interval = pair<i64, i64>;
    struct part_interval {
        interval x, m, a, s;

        interval& operator[](char chr) {
            if (chr == 'x') return x;
            if (chr == 'm') return m;
            if (chr == 'a') return a;
            if (chr == 's') return s;

            return x;
        }

        i64 ways() {
            i64 ways = 1;

            ways *= (x.second - x.first + 1);
            ways *= (m.second - m.first + 1);
            ways *= (a.second - a.first + 1);
            ways *= (s.second - s.first + 1);

            return ways;
        }

        void print() {
            cout << "x(" << x.first << "-" << x.second << "), ";
            cout << "m(" << m.first << "-" << m.second << "), ";
            cout << "a(" << a.first << "-" << a.second << "), ";
            cout << "s(" << s.first << "-" << s.second << ")";
            cout << endl;
        }
    };

    i64 count_all_possible_accepted(workflow_umap& workflows, const string& start, part_interval input) {
        const auto& current_rule = workflows[start];
        i64 ways = 0;
        for(auto rule: current_rule) {
            if (!rule.part) {
                if (rule.to == "A" || rule.to == "R") {
                    return ways + ((rule.to == "A") ? input.ways() : 0);
                }

                ways += count_all_possible_accepted(workflows, rule.to, input);
                continue;
            }

            auto& itv = input[rule.part];
            interval yes, no;
            if (rule.less_than && rule.value > itv.first && rule.value <= itv.second) {
                yes = {itv.first, rule.value - 1};
                no = {rule.value, itv.second};

                itv = yes;
                ways += count_all_possible_accepted(workflows, rule.to, input);
                itv = no;
            } else if (!rule.less_than && rule.value >= itv.first && rule.value < itv.second) {
                yes = {rule.value + 1, itv.second};
                no = {itv.first, rule.value};

                itv = yes;
                ways += count_all_possible_accepted(workflows, rule.to, input);
                itv = no;
            }
        }

        return ways + ((start == "A") ? input.ways() : 0);
    }
}

void aoc::day_19(vector<string>& input) {
    auto [workflows, parts] = parse_input(input);

    auto part1 = r::fold_left(
        parts | rv::filter([&](auto&& part) { return is_accepted(workflows, part); }) | rv::transform([](auto&& part) { return part.x + part.m + part.a + part.s; }),
        0, plus<>()
    );

    auto part2_input = part_interval{
        {1, 4000},
        {1, 4000},
        {1, 4000},
        {1, 4000},
    };
    auto part2 = count_all_possible_accepted(workflows, "in", part2_input);

    assert(part1 == 449531);
    assert(part2 == 122756210763577);

    // cout << "PART I  : " << part1 << endl;
    // cout << "PART II : " << part2 << endl;
}
