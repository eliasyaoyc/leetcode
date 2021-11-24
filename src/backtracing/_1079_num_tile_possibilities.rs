struct Solution;

impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        fn helper(title: &mut Vec<usize>, ret: &mut i32) {
            for i in 0..26 as usize {
                if title[i] == 0 {
                    continue;
                }
                *ret += 1;
                title[i] -= 1;
                helper(title, ret);
                title[i] += 1;
            }
        }
        let mut counts: Vec<usize> = vec![0; 26];
        for c in tiles.chars() {
            counts[(c as u8 - b'A') as usize] += 1;
        }
        let mut ret = 0_i32;
        helper(&mut counts, &mut ret);
        ret
    }
}