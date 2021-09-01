// bit-set solution to save memory

#[allow(dead_code)]
pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    use std::collections::HashSet;

    if s.len() < 10 {
        return vec![];
    }

    let mut cur: u32 = 0;
    let mut found: HashSet<u32> = HashSet::new();
    let mut repeat: HashSet<u32> = HashSet::new();
    let mut count: u32 = 0;

    for ch in s.chars() {
        // same as:
        //     let cur = &s[i..i + 10];
        // but the cur is bits

        // shift 2 because 0~3 takes max 2 bits
        cur <<= 2;
        match ch {
            // bitwise or: set the last 2 bits, preserve others
            'A' => cur |= 0_u32,
            'C' => cur |= 1_u32,
            'G' => cur |= 2_u32,
            'T' => cur |= 3_u32,
            _ => unreachable!(),
        }
        if count < 9 {
            count += 1;
            continue;
        }
        // mask high 12-bits, leaving only 10 letters (20 bits)
        cur &= 0b0000_0000_0000_1111_1111_1111_1111_1111;
        if !found.insert(cur) {
            repeat.insert(cur);
        }
    }
    // convert bits back to letters
    repeat
        .iter()
        .map(|&code| {
            let mut substr = String::new();
            let mut code = code;
            for _ in 0..10 {
                // take the first 2 bits each time
                substr.push(match code & 0b0000_0000_0000_1100_0000_0000_0000_0000 {
                    0b0000_0000_0000_0000_0000_0000_0000_0000 => 'A',
                    0b0000_0000_0000_0100_0000_0000_0000_0000 => 'C',
                    0b0000_0000_0000_1000_0000_0000_0000_0000 => 'G',
                    0b0000_0000_0000_1100_0000_0000_0000_0000 => 'T',
                    _ => unreachable!(),
                });
                code <<= 2;
            }
            substr
        })
        .collect()
}

#[test]
fn run() {
    let ans = find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".into());
    assert_eq!(ans.len(), 2);

    let ans = find_repeated_dna_sequences("AAAAAAAAAAAAA".into());
    assert_eq!(ans.len(), 1);
}
