package hot_100

/*
因为字符串中的字符全是小写字母，可以用长度为26的数组记录字母出现的次数
设n = len(s), m = len(p)。记录p字符串的字母频次p_cnt，和s字符串前m个字母频次s_cnt
若p_cnt和s_cnt相等，则找到第一个异位词索引 0
继续遍历s字符串索引为[m, n)的字母，在s_cnt中每次增加一个新字母，去除一个旧字母
判断p_cnt和s_cnt是否相等，相等则在返回值res中新增异位词索引 i - m + 1
*/
func findAnagrams(s string, p string) []int {
	var ans []int
	l1, l2 := len(s), len(p)
	if l1 < l2 {
		return ans
	}
	var scount, pcount [26]int
	for i, ch := range p {
		pcount[ch-'a']++
		scount[s[i]-'a']++
	}
	if scount == pcount {
		ans = append(ans, 0)
	}
	for i := l2; i < l1; i++ {
		scount[s[i]-'a']++
		scount[s[i-l2]-'a']--
		if scount == pcount {
			ans = append(ans, i-l2+1)
		}
	}
	return ans
}
