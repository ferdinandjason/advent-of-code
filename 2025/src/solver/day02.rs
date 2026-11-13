pub fn parse(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')
        .into_iter()
        .map(|s| {
            let (a, b) = s.split_once('-').unwrap();
            (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap())
        })
        .collect()
}

pub fn solve(input: &Vec<(u64, u64)>) -> (u64, u64) {
    let p1 = input
        .iter()
        .map(|(start, end)| {
            let (ss, ee) = (start.to_string(), end.to_string());
            let (sn, en) = (ss.len(), ee.len());
            let mut sum = 0;

            if sn % 2 == 0 {
                let st = (&ss[0..sn/2]).parse::<u64>().unwrap();
                let et = if en % 2 == 0 {
                    (&ee[0..en/2]).parse::<u64>().unwrap()
                } else {
                    10_u64.pow((en as u32 + 1u32) / 2)
                };


                for i in st..=(et+1) {
                    let di = i.ilog10() + 1;
                    let ni = i * 10_u64.pow(di) + i;
                    if ni >= *start && ni <= *end {
                        sum += ni;
                    }
                }
            } else {
                let st = 10_u64.pow((sn as u32 + 1u32) / 2 - 1);
                let et = if en % 2 == 0 {
                    (&ee[0..en/2]).parse::<u64>().unwrap()
                } else {
                    10_u64.pow((en as u32 + 1u32) / 2)
                };


                for i in st..=(et+1) {
                    let di = i.ilog10() + 1;
                    let ni = i * 10_u64.pow(di) + i;
                    if ni >= *start && ni <= *end {
                        sum += ni;
                    }
                }
            }

            sum
        })
        .sum::<u64>();

    let p2 = input
        .iter()
        .map(|(start, end)| {
            let mut sum = 0;
            for num in *start..=*end {
                let s = num.to_string();
                for nl in 1..=((num.ilog10() + 1) / 2) {
                    let mut chunks = s.as_bytes().chunks(nl as usize);
                    if let Some(first) = chunks.next() {
                        if chunks.all(|chunk| chunk == first) {
                            sum += num;
                            break;
                        }
                    }
                }
            }

            sum
        })
        .sum::<u64>();

    (p1, p2)
}
