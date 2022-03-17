package hot_100

func removeInvalidParentheses(s string) []string {
	var ret []string
	var dfs func(ans *[]string, start, lremove, rremove int, str string)
	dfs = func(ans *[]string, start, lremove, rremove int, str string) {
		if lremove == 0 && rremove == 0 {
			if isValid01(str) {
				*ans = append(*ans, str)
			}
			return
		}
		for i := start; i < len(str); i++ {
			if i != start && str[i] == str[i-1] {
				continue
			}
			if lremove+rremove > len(str)-i {
				return
			}
			if lremove > 0 && str[i] == '(' {
				dfs(ans, i, lremove-1, rremove, str[:i]+str[i+1:])
			}

			if rremove > 0 && str[i] == ')' {
				dfs(ans, i, lremove, rremove-1, str[:i]+str[i+1:])
			}
		}
	}
	lremove, rremove := 0, 0
	for _, ch := range s {
		if ch == '(' {
			lremove++
		} else if ch == ')' {
			if lremove == 0 {
				rremove++
			} else {
				lremove--
			}
		}
	}
	dfs(&ret, 0, lremove, rremove, s)
	return ret
}

func isValid01(str string) bool {
	cnt := 0
	for _, ch := range str {
		if ch == '(' {
			cnt++
		} else if ch == ')' {
			cnt--
			if cnt < 0 {
				return false
			}
		}
	}
	return cnt == 0
}
