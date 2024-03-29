struct Solution;

impl Solution {
    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        fn helper(s: &str) -> Vec<String> {
            let mut ret = vec![];
            if s == "0" || !s.starts_with('0') {
                ret.push(s.to_string());
            }
            for i in 1..s.len() {
                let x = format!("{}.{}", s.get(..i).unwrap(), s.get(i..).unwrap());
                let v = x.split('.').collect::<Vec<_>>();
                if (v[0] == "0" || !v[0].starts_with('0')) && !v[1].ends_with('0') {
                    ret.push(x);
                }
            }
            ret
        }

        let mut ret = vec![];

        for i in 2..s.len() - 1 {
            for x in helper(s.get(1..i).unwrap()) {
                for y in helper(s.get(i..s.len() - 1).unwrap()) {
                    ret.push(format!("({}, {})", x, y));
                }
            }
        }
        ret
    }
}