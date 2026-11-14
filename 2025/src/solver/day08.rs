use itertools::Itertools;

struct UnionFind {
    n: usize,
    parent: Vec<usize>,
    size: Vec<usize>,
    last: Option<(usize, usize)>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            n,
            parent: (0..n).collect(),
            size: vec![1; n],
            last: None,
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            self.parent[i] = self.find(self.parent[i]);
        }
        self.parent[i]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut root_a = self.find(a);
        let mut root_b = self.find(b);

        if root_a == root_b {
            return false;
        }

        if self.size[root_a] < self.size[root_b] {
            std::mem::swap(&mut root_a, &mut root_b);
        }
        self.parent[root_b] = root_a;
        self.size[root_a] += self.size[root_b];

        if self.size[root_a] == self.n && self.last == None {
            self.last = Some((a, b));
        }

        true
    }
}

pub struct Input {
    n: usize,
    node: Vec<(i64, i64, i64)>,
    pairs: Vec<(i64, usize, usize)>,
}

pub fn parse(input: &str) -> Input {
    let node: Vec<(i64, i64, i64)> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();
    let n = node.len();

    let mut pairs: Vec<(i64, usize, usize)> = (0..n)
        .flat_map(|i| ((i + 1)..n).map(move |j| (i, j)))
        .map(|(i, j)| {
            let (x1, y1, z1) = node[i];
            let (x2, y2, z2) = node[j];

            ((x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2), i, j)
        })
        .collect();

    pairs.sort_unstable();

    Input {
        n,
        node: node.clone(),
        pairs,
    }
}

pub fn solve(input: &Input) -> (i64, i64) {
    let mut uf = UnionFind::new(input.n);
    for &(_, i, j) in input.pairs.iter().take(1000) {
        uf.union(i, j);
    }

    let mut sizes: Vec<_> = (0..input.n)
        .filter(|&i| uf.parent[i] == i)
        .map(|i| uf.size[i] as i64)
        .collect();

    sizes.sort_unstable_by(|a, b| b.cmp(a));

    for &(_, i, j) in input.pairs.iter().skip(1000) {
        if uf.last != None {
            break;
        }
        uf.union(i, j);
    }

    (
        sizes.iter().take(3).product(),
        input.node[uf.last.unwrap().0].0 * input.node[uf.last.unwrap().1].0,
    )
}
