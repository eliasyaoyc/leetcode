//! 明确 base case -> 明确[状态] -> 明确[选择] -> 定义 dp 数组/函数的含义
//! ```
//! /// 初始化 base case
//! let mut dp: Vec<Vec<u32>> = vec![];
//! /// 进行状态转移
//! for 状态1 in 0..dp.len(){
//!   for 状态2 in 0..dp.len(){
//!      dp[状态1][状态2] = 求最值(选择1,选择2);
//!  }
//! }
//! ```
//!
//! 关于子序列相关算法总结：
//! 1. 把 `dp[i][j]` 中的  `i` 当作 `a` 数组中的一个字符，   `j` 当作 `b` 数组中的一个字符
//! 2. 动态方程 常规来说  `a[i] == b[j]  dp[i][j] = dp[i-1][j-1] + 1` 含义就是 如果`a`中的第`i`个字符 与 `b` 中的第`j`个字符一样，
//!   则`dp[i][j]` 就是`dp`中 上一个字符中的值 + 1;  如果不等  `dp[i][j] = max(dp[i - 1][j], dp[i][j - 1])` 取最大.
//! 上述适用于两个数组或者两个字符串计算子序列问题
//!
//! 若只有一个数组或者一个字符串计算回文串，
//! 只需要把 `dp[i][j]` 中  `i` 当作这个数组的 第 `i` 个位置，`j` 当作这个数组的 第 `j` 个位置，如下
//! ```text
//! i               j
//! 1 2 3 4 5 6 7 8 9
//!
//! dp[i+1][j-1] 则表示
//!   i           j
//! 1 2 3 4 5 6 7 8 9
//! ```
//!
//! 关于 背包问题 （01 背包和 完全背包）
//! ⚠️ 如果使用一维数组来解决的话，物品遍历放在外层，背包放在内层，并且内层倒序遍历
//! 01 背包：只有一个 选一个或者不选：比如 有N件物品和一个最多能背重量为 W 的背包，第 i 件物品的重量是 weight[i], 得到的价值是 value[i]，每件物品只能用一次，求哪些物品装入背包李物品价值总和最大
//!
//! 完全背包：无数个   选几个或者不选
mod _509_fib;
mod _322_coin_change;
mod _300_length_of_lis;
mod _53_max_sub_array;
mod _494_find_target_sum_ways;
mod _931_min_falling_path_sum;
mod _516_longest_palindrome_subseq;
mod _1143_longest_common_subsequence;
mod _718_find_length;
mod _674_find_length_of_lcis;
mod _392_is_subsequence;
mod _647_count_substrings;
mod _583_min_distance;
mod _1035_max_uncrossed_lines;
mod _115_num_distinct;
mod _70_climb_stairs;
mod _746_min_cost_climbing_stairs;
mod _62_unique_paths;
mod _63_unique_paths_with_obstacles;
mod _416_can_partition;
mod _343_integer_break;
mod _96_num_trees;
mod _1049_last_stone_weight_ii;