struct Solution;

impl Solution {
    pub fn respace(dictionary: Vec<String>, sentence: String) -> i32 {
        // dp[i] 代表 在 sentence i 这个位置匹配数最大的值
        let mut dp = vec![0; sentence.len() + 1];
        for i in 1..=sentence.len() {
            dp[i] = dp[i - 1];
            for j in 0..dictionary.len() {
                if i < dictionary[j].len() {
                    continue;
                }
                if &sentence.as_str()[i - dictionary[j].len()..i] == dictionary[j] {
                    dp[i] = (dp[i - dictionary[j].len()] + dictionary[j].len()).max(dp[i]);
                }
            }
        }
        sentence.len() as i32 - dp[sentence.len()] as i32
    }
}

#[test]
fn test(){
    let ret = Solution::respace(vec!["looked".into(),"just".into(),"like".into(),"her".into(),"brother".into()],"jesslookedjustliketimherbrother".into());
    println!("{}",ret);
}