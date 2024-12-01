#include"aoc.h"
#include<ranges>
#include<algorithm>
#include<iostream>
#include<cmath>
#include<sstream>
#include<numeric>
#include<queue>
#include<memory>
#include<map>
#include<unordered_map>
#include<set>
#include<unordered_set>
#include<functional>
#include<cassert>

using namespace std;

namespace {
    struct point3d {
        double x = 0.0, y = 0.0, z = 0.0;
    };

    point3d operator-(const point3d& p1, const point3d& p2) {
        return {
            p1.x - p2.x,
            p1.y - p2.y,
            p1.z - p2.z,
        };
    }

    struct particle {
        point3d p, v;
    };


    particle parse_particle(string& line) {
        auto nums = aoc::extract_numbers<double>(line);

        return {
            {nums[0], nums[1], nums[2]},
            {nums[3+0], nums[3+1], nums[3+2]},
        };
    }

    vector<particle> parse_input(vector<string>& input) {
        return input | rv::transform(parse_particle) | to<vector<particle>>();
    }

    bool is_future(const particle& p, double x, double y) {
        auto dy = y - p.p.y;
        auto dx = x - p.p.x;

        return ((dy >= 0) == (p.v.y >= 0)) && ((dx >= 0) == (p.v.x >= 0));
    }

    bool check_overlap(const particle& p1, const particle& p2, double mini, double maxi) {
        double a1 = p1.v.y / p1.v.x;
        double b1 = -1;
        double c1 = p1.p.y - (a1 * p1.p.x);

        double a2 = p2.v.y / p2.v.x;
        double b2 = -1;
        double c2 = p2.p.y -(a2 * p2.p.x);

        double x = (b1 * c2 -  b2 * c1) / (a1 * b2 - a2 * b1);
        double y = (a2 * c1 -  a1 * c2) / (a1 * b2 - a2 * b1);

        bool is_future_p1 = is_future(p1, x, y);
        bool is_future_p2 = is_future(p2, x, y);

        return x >= mini && x <= maxi && y >= mini && y <= maxi && is_future_p1 && is_future_p2;
    }

    int solve_part1(const vector<particle>& particles, double mini, double maxi) {
        int n = static_cast<int>(particles.size());

        int answer = 0;
        for(int i = 0; i < n; i++) {
            for(int j = i + 1; j < n; j++) {
                answer += check_overlap(particles[i], particles[j], mini, maxi);
            }
        }

        return answer;
    }

    constexpr double EPS = 1e-9;
    constexpr int INF = -1;

    int gauss (vector<vector<double>> a, vector<double> &ans) {
        int n = (int) a.size();
        int m = (int) a[0].size() - 1;

        vector<int> where (m, -1);
        for (int col=0, row=0; col<m && row<n; ++col) {
            int sel = row;
            for (int i=row; i<n; ++i)
                if (abs (a[i][col]) > abs (a[sel][col]))
                    sel = i;
            if (abs (a[sel][col]) < EPS)
                continue;
            for (int i=col; i<=m; ++i)
                swap (a[sel][i], a[row][i]);
            where[col] = row;

            for (int i=0; i<n; ++i)
                if (i != row) {
                    double c = a[i][col] / a[row][col];
                    for (int j=col; j<=m; ++j)
                        a[i][j] -= a[row][j] * c;
                }
            ++row;
        }

        ans.assign (m, 0);
        for (int i=0; i<m; ++i)
            if (where[i] != -1)
                ans[i] = a[where[i]][m] / a[where[i]][i];
        for (int i=0; i<n; ++i) {
            double sum = 0;
            for (int j=0; j<m; ++j)
                sum += ans[j] * a[i][j];
            if (abs (sum - a[i][m]) > EPS)
                return 0;
        }

        for (int i=0; i<m; ++i)
            if (where[i] == -1)
                return INF;
        return 1;
    }

    i64 solve_part2(vector<particle> p) {
        auto p01 = p[0].p - p[1].p;
        auto v01 = p[0].v - p[1].v;
        auto p02 = p[0].p - p[2].p;
        auto v02 = p[0].v - p[2].v;

        auto cross = [&](point3d a, point3d b) -> point3d {
            i64 a1 = a.x, b1 = b.x;
            i64 a2 = a.y, b2 = b.y;
            i64 a3 = a.z, b3 = b.z;

            return {
                static_cast<double>(a2 * b3 - a3 * b2), 
                static_cast<double>(a3 * b1 - a1 * b3), 
                static_cast<double>(a1 * b2 - a2 * b1)
            };
        };

        auto cross0 = cross(p[0].p, p[0].v);
        auto cross1 = cross(p[1].p, p[1].v);
        auto cross2 = cross(p[2].p, p[2].v);

        vector<vector<double>> inputs = {
            {     0, -p01.z,  p01.y,      0, -v01.z,  v01.y, cross1.x - cross0.x},
            { p01.z,      0, -p01.x,  v01.z,      0, -v01.x, cross1.y - cross0.y},
            {-p01.y,  p01.x,      0, -v01.y,  v01.x,      0, cross1.z - cross0.z},
            {     0, -p02.z,  p02.y,      0, -v02.z,  v02.y, cross2.x - cross0.x},
            { p02.z,      0, -p02.x,  v02.z,      0, -v02.x, cross2.y - cross0.y},
            {-p02.y,  p02.x,      0, -v02.y,  v02.x,      0, cross2.z - cross0.z},
        };

        vector<double> ans;

        if (gauss(inputs, ans) != INF) {
            return static_cast<i64>(round(ans[3])) + static_cast<i64>(round(ans[4])) + static_cast<i64>(round(ans[5]));
        }
        return -1;
    }
}

void aoc::day_24(vector<string>& input) {
    auto particles = parse_input(input);

    auto part1 = solve_part1(particles, 200000000000000.0, 400000000000000.0);
    auto part2 = solve_part2(particles);

    assert(part1 == 26657);
    assert(part2 == 828418331313365);
}
