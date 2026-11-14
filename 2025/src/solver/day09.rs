pub fn parse(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|s| {
            let (a, b) = s.split_once(',').unwrap();
            (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
        })
        .collect()
}

pub fn solve(input: &Vec<(i64, i64)>) -> (i64, i64) {
    let p1 = (0..input.len())
        .flat_map(|i| ((i + 1)..input.len()).map(move |j| (i, j)))
        .map(|(i, j)| {
            let dx = (input[i].0 - input[j].0).abs() + 1;
            let dy = (input[i].1 - input[j].1).abs() + 1;

            dx * dy
        })
        .max()
        .unwrap();

    let n = input.len();
    let edges: Vec<((i64, i64), (i64, i64))> =
        (0..n).map(|i| (input[i], input[(i + 1) % n])).collect();

    let p2 = (0..input.len())
        .flat_map(|i| ((i + 1)..input.len()).map(move |j| (i, j)))
        .filter(|(i, j)| {
            let x_min = input[*i].0.min(input[*j].0);
            let x_max = input[*i].0.max(input[*j].0);
            let y_min = input[*i].1.min(input[*j].1);
            let y_max = input[*i].1.max(input[*j].1);

            !cuts_interior(x_min, x_max, y_min, y_max, &edges)
                && center_inside(x_min, x_max, y_min, y_max, &edges)
        })
        .map(|(i, j)| {
            let dx = (input[i].0 - input[j].0).abs() + 1;
            let dy = (input[i].1 - input[j].1).abs() + 1;

            dx * dy
        })
        .max()
        .unwrap();

    (p1, p2)
}

fn cuts_interior(
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    edges: &[((i64, i64), (i64, i64))],
) -> bool {
    for &((x1, y1), (x2, y2)) in edges {
        if y1 == y2 {
            let (ex_min, ex_max) = (x1.min(x2), x1.max(x2));
            if y_min < y1 && y1 < y_max && ex_max > x_min && ex_min < x_max {
                return true;
            }
        } else {
            let (ey_min, ey_max) = (y1.min(y2), y1.max(y2));
            if x_min < x1 && x1 < x_max && ey_max > y_min && ey_min < y_max {
                return true;
            }
        }
    }
    false
}

fn center_inside(
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    edges: &[((i64, i64), (i64, i64))],
) -> bool {
    let cx2 = x_min + x_max;
    let cy2 = y_min + y_max;
    let mut crossings = 0;

    for &((x1, y1), (x2, y2)) in edges {
        if y1 == y2 {
            let (ex_min, ex_max) = (x1.min(x2) * 2, x1.max(x2) * 2);
            if cy2 == y1 * 2 && ex_min <= cx2 && cx2 <= ex_max {
                return true;
            }
        } else {
            let ex2 = x1 * 2;
            let (ey_min, ey_max) = (y1.min(y2) * 2, y1.max(y2) * 2);

            if cx2 == ex2 && ey_min <= cy2 && cy2 <= ey_max {
                return true;
            }

            if ex2 > cx2 && ey_min <= cy2 && cy2 < ey_max {
                crossings += 1;
            }
        }
    }

    crossings % 2 == 1
}
