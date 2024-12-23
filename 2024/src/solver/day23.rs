use fxhash::{FxHashMap, FxHashSet};
use itertools::{sorted, Itertools};

pub struct Input {
    computers: Vec<String>,
    graph: FxHashMap<usize, Vec<usize>>,
}

pub fn parse(input: &str) -> Input {
    let mut seen = FxHashMap::default();
    let mut computers = Vec::new();
    let mut graph = FxHashMap::default();

    input.lines().for_each(|line| {
        let (a, b) = line.split_once("-").unwrap();
        seen.entry(a).or_insert_with(|| {
            computers.push(a.to_owned());
            computers.len() - 1
        });
        seen.entry(b).or_insert_with(|| {
            computers.push(b.to_owned());
            computers.len() - 1
        });

        let idxa = seen[a];
        let idxb = seen[b];

        graph
            .entry(idxa)
            .and_modify(|v: &mut Vec<usize>| v.push(idxb))
            .or_insert(vec![idxb]);
        graph
            .entry(idxb)
            .and_modify(|v: &mut Vec<usize>| v.push(idxa))
            .or_insert(vec![idxa]);
    });

    Input { computers, graph }
}

pub fn solve(input: &Input) -> (usize, String) {
    (
        find_three_connected_computers(input),
        find_max_clique(input),
    )
}

fn find_three_connected_computers(input: &Input) -> usize {
    let mut lan_party_computers = FxHashSet::default();
    for (idx, _) in input.computers.iter().enumerate() {
        let mut path = Vec::new();
        dfs(input, idx, idx, 0, &mut path, &mut lan_party_computers, 3);
    }

    lan_party_computers
        .iter()
        .filter(|computers| {
            input.computers[computers.0].chars().nth(0).unwrap() == 't'
                || input.computers[computers.1].chars().nth(0).unwrap() == 't'
                || input.computers[computers.2].chars().nth(0).unwrap() == 't'
        })
        .count()
}

fn find_max_clique(input: &Input) -> String {
    let mut r = FxHashSet::default();
    let mut p = (0..input.computers.len()).collect::<FxHashSet<_>>();
    let mut x = FxHashSet::default();
    let mut cliques = Vec::new();

    bron_kerbosch(&mut r, &mut p, &mut x, &input.graph, &mut cliques);

    cliques
        .iter()
        .map(|idx| input.computers[*idx].clone())
        .sorted()
        .join(",")
}

fn bron_kerbosch(
    r: &mut FxHashSet<usize>,
    p: &mut FxHashSet<usize>,
    x: &mut FxHashSet<usize>,
    adj: &FxHashMap<usize, Vec<usize>>,
    cliques: &mut Vec<usize>,
) {
    if p.is_empty() && x.is_empty() {
        if cliques.len() < r.len() {
            *cliques = r.iter().cloned().collect::<Vec<_>>();
        }

        return;
    }

    for v in p.clone() {
        let mut nr = r.clone();
        nr.insert(v);

        let nv = adj[&v].iter().cloned().collect::<FxHashSet<_>>();

        let mut np = p.intersection(&nv).cloned().collect();
        let mut nx = x.intersection(&nv).cloned().collect();

        bron_kerbosch(&mut nr, &mut np, &mut nx, adj, cliques);

        p.remove(&v);
        x.insert(v);
    }
}

fn dfs(
    input: &Input,
    root: usize,
    u: usize,
    l: usize,
    path: &mut Vec<usize>,
    lan_party_computers: &mut FxHashSet<(usize, usize, usize)>,
    max_level: usize,
) {
    if l == max_level {
        if root == u {
            lan_party_computers.insert(
                sorted(path.iter())
                    .map(|v| *v)
                    .collect_tuple::<(usize, usize, usize)>()
                    .unwrap(),
            );
        }
        return;
    }

    for v in input.graph[&u].iter() {
        path.push(*v);
        dfs(input, root, *v, l + 1, path, lan_party_computers, max_level);
        path.pop();
    }
}
