use std::collections::HashMap;

// =====================
// 1️⃣  BWT
// =====================

fn bwt(text: &str) -> String {
    let mut s = text.to_string();
    if !s.ends_with('$') {
        s.push('$'); // 终止符
    }

    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();

    // 生成所有旋转
    let mut rotations: Vec<String> = (0..n)
        .map(|i| {
            let mut rot = String::new();
            for j in 0..n {
                rot.push(chars[(i + j) % n]);
            }
            rot
        })
        .collect();

    rotations.sort();

    // 取最后一列
    rotations
        .iter()
        .map(|r| r.chars().last().unwrap())
        .collect()
}

// =====================
// 1️⃣  build SA
// =====================

fn build_suffix_array(text: &str) -> Vec<usize>{
    let mut s =  text.to_string();
    if !s.ends_with('$') {
        s.push('$'); // 终止符
    }

    let chars = s.chars().collect::<Vec<char>>();
    let n = chars.len();
    let mut suffixes = (0..n).map(|i|{
        let suffix =chars[i..].iter().collect();
        (suffix, i)
    }).collect::<Vec<(String, usize)>>();
    
    suffixes.sort_by(|a,b| a.0.cmp(&b.0));
    suffixes.iter().map(|(_,idx)| *idx).collect()
}

// =====================
// 1️⃣  build bwt
// =====================

fn build_bwt(text: &str, sa:&Vec<usize>) -> Vec<char>{
    let mut s = text.to_string();
    if !s.ends_with('$') {
        s.push('$'); // 终止符
    }
    let chars = s.chars().collect::<Vec<char>>();
    let n = chars.len();
    sa.iter().map(|&i|{
        if i==0{
            chars[n-1]
        } else {
            chars[i-1]
        }
    }).collect()

}



// =====================
// 2️⃣  逆 BWT
// =====================

fn inverse_bwt(bwt: &str) -> String {
    let n = bwt.len();
    let mut table: Vec<String> = vec![String::new(); n];

    for _ in 0..n {
        for (i, c) in bwt.chars().enumerate() {
            table[i] = format!("{}{}", c, table[i]);
        }
        table.sort();
    }

    for row in table {
        if row.ends_with('$') {
            return row;
        }
    }

    String::new()
}

// =====================
// 3️⃣  FM-index
// =====================

struct FMIndex {
    bwt: Vec<char>,
    c_table: HashMap<char, usize>,
    occ_table: Vec<HashMap<char, usize>>,
    sa: Vec<usize>, // 可选：存储后缀数组以支持位置恢复
}

impl FMIndex {
    fn new(text: &str) -> Self {
        // let bwt_string = bwt(text);
        
        // let bwt_chars: Vec<char> = bwt_string.chars().collect();
        let sa = build_suffix_array(text);
        let bwt_chars = build_bwt(text, &sa);

        // 构建 C 表（每个字符在排序后文本中起始位置）
        let mut counts: HashMap<char, usize> = HashMap::new();
        for &c in &bwt_chars {
            *counts.entry(c).or_insert(0) += 1;
        }

        let mut keys: Vec<char> = counts.keys().cloned().collect();
        keys.sort();

        let mut c_table = HashMap::new();
        let mut total = 0;
        for k in keys {
            c_table.insert(k, total);
            total += counts[&k];
        }

        // 构建 Occ 表
        let mut occ_table = Vec::new();
        let mut current: HashMap<char, usize> = HashMap::new();

        for &c in &bwt_chars {
            *current.entry(c).or_insert(0) += 1;
            occ_table.push(current.clone());
        }

        FMIndex {
            bwt: bwt_chars,
            c_table,
            occ_table,
            sa, 
        }
    }

    // 统计某字符在 [0..pos] 出现次数
    fn occ(&self, c: char, pos: isize) -> usize {
        if pos < 0 {
            return 0;
        }
        self.occ_table[pos as usize]
            .get(&c)
            .cloned()
            .unwrap_or(0)
    }

    // backward search
    fn search(&self, pattern: &str) -> Option<Vec<usize>> {
        let mut l = 0;
        let mut r = self.bwt.len() - 1;

        for c in pattern.chars().rev() {
            if !self.c_table.contains_key(&c) {
                return None;
            }

            let c_start = self.c_table[&c];

            l = c_start + self.occ(c, l as isize - 1);
            r = c_start + self.occ(c, r as isize) - 1;

            if l > r {
                return None;
            }
        }

        // Some((l, r))
        Some(self.sa[l..=r].to_vec())
    }
}

// =====================
// 4️⃣  主函数测试
// =====================

fn main() {
    let text: &str = "banana";

    println!("Original: {}", text);

    let suffixes = build_suffix_array(text);

    let bwt_chars = build_bwt(text, &suffixes);
    let bwt_str = bwt(text);
    println!("BWT: {}", bwt_str);

    let restored = inverse_bwt(&bwt_str);
    println!("Inverse BWT: {}", restored);

    let fm = FMIndex::new(text);

    let pattern = "a";
    match fm.search(pattern) {
        Some(positions) => {
            println!("Pattern '{}' found at positions:", pattern);
            for pos in positions {
                println!("{}", pos);
            }
        },
        None => println!("Pattern '{}' not found", pattern),
    }
}
