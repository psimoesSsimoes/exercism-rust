pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut vec: Vec<Vec<u32>> = vec![];
        let mut row: u32 = 0;
        while row < row_count {
            let mut i: u32 = 0;
            let mut inner_vec: Vec<u32> = vec![];
            while i <= row {
                inner_vec.push(binomial_cof(row, i));
                i += 1;
            }
            vec.push(inner_vec);
            row += 1;
        }

        PascalsTriangle { rows: vec }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.to_vec()
    }
}

fn binomial_cof(n: u32, k: u32) -> u32 {
    let mut res: u32 = 1;
    let mut k = k;

    if k > n - k {
        k = n - k;
    }

    let mut count = 0;
    while count < k {
        res *= n - count;
        res /= count + 1;
        count += 1;
    }
    res
}
