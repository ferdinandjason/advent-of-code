pub fn parse(input: &str) -> Vec<FileBlock> {
    let mut id = 0;
    let mut sign = 0;

    input
        .chars()
        .map(|raw| {
            let x = raw as u8 - b'0';

            sign ^= 1;
            match sign {
                1 => {
                    id += 1;
                    FileBlock {
                        id: Some(id - 1),
                        qty: x as u64,
                        gaps: None,
                    }
                }
                0 => FileBlock {
                    id: None,
                    qty: x as u64,
                    gaps: None,
                },
                _ => unimplemented!(),
            }
        })
        .collect::<Vec<_>>()
}

pub fn solve(input: &[FileBlock]) -> (u64, u64) {
    (part1(input), part2(input))
}

#[derive(Debug, Clone)]
pub struct FileBlock {
    id: Option<u64>,
    qty: u64,

    gaps: Option<Vec<FileBlock>>,
}

fn part1(input: &[FileBlock]) -> u64 {
    let mut fbs = input.iter().cloned().collect::<Vec<_>>();

    let (mut i, mut j) = (0, fbs.len() - 1);
    while i <= j {
        if fbs[i].id == None {
            let mut gaps = Vec::new();
            while fbs[i].qty > 0 {
                if fbs[i].qty >= fbs[j].qty {
                    gaps.push(FileBlock {
                        id: fbs[j].id,
                        qty: fbs[j].qty,
                        gaps: None,
                    });

                    fbs[i].qty -= fbs[j].qty;
                    fbs[j].qty = 0;
                    j -= 2;
                } else {
                    gaps.push(FileBlock {
                        id: fbs[j].id,
                        qty: fbs[i].qty,
                        gaps: None,
                    });

                    fbs[j].qty -= fbs[i].qty;
                    fbs[i].qty = 0;
                }
            }
            fbs[i].gaps = Some(gaps);
        }
        i += 1;
    }

    calculate_checksum(&fbs, j)
}

fn part2(input: &[FileBlock]) -> u64 {
    let mut fbs = input.iter().cloned().collect::<Vec<_>>();
    for i in (0..fbs.len()).rev() {
        if let Some(id) = fbs[i].id {
            for j in 0..i {
                if fbs[j].id == None && fbs[j].qty >= fbs[i].qty {
                    if fbs[j].gaps.is_none() {
                        fbs[j].gaps = Some(Vec::new());
                    }

                    let fb = FileBlock {
                        id: Some(id),
                        qty: fbs[i].qty,
                        gaps: None,
                    };

                    fbs[j].gaps.as_mut().unwrap().push(fb);
                    fbs[j].qty -= fbs[i].qty;
                    fbs[i].id = None;

                    break;
                }
            }
        }
    }

    calculate_checksum(&fbs, fbs.len() - 1)
}

fn checksum_block(id: u64, a: u64, n: u64) -> u64 {
    id * n * (2 * a + (n - 1)) / 2
}

fn calculate_checksum(fbs: &Vec<FileBlock>, n: usize) -> u64 {
    let (mut pos, mut checksum) = (0, 0);
    for i in 0..=n {
        if let Some(id) = fbs[i].id {
            if fbs[i].qty == 0 {
                continue;
            }

            checksum += checksum_block(id, pos, fbs[i].qty);
            pos += fbs[i].qty;
        } else {
            if let Some(gaps) = &fbs[i].gaps {
                for j in 0..gaps.len() {
                    checksum += checksum_block(gaps[j].id.unwrap(), pos, gaps[j].qty);
                    pos += gaps[j].qty;
                }
            }
            pos += fbs[i].qty;
        }
    }

    checksum
}
