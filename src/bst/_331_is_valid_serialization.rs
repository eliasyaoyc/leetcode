struct Solution {}

impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let words: Vec<_> = preorder.split(',').collect();
        let mut ret = vec![];
        for x in words {
            ret.push(x);
            while let Some(&[y, "#", "#"]) = ret.get(ret.len() - 3..ret.len()) {
                if y == "#" {
                    break;
                } else {
                    ret.pop();
                    ret.pop();
                    ret.pop();
                    ret.push("#");
                }
            }
        }
        &ret == &["#"]
    }

    pub fn is_valid_serialization_02(preorder: String) -> bool {
        let words: Vec<_> = preorder.split(',').collect();
        let rest = words.iter().fold(1, |w, i| {
            if w == 0 {
                return -1;
            }
            let mut temp = w - 1;
            if *i != "#" {
                temp += 2;
                dbg!(temp);
            }
            dbg!(temp);
            temp
        });
        rest == 0
    }
}

#[test]
fn test() {
    let ret = Solution::is_valid_serialization_02("#,#,3,5,#".to_string());
    println!("{}", ret);
}