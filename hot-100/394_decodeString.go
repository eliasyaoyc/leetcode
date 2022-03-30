package hot_100

import (
	"strconv"
	"strings"
)

func decodeString(s string) string {
	numStack := []int{}
	strStack := []string{}
	num := 0
	result := ""
	for _, ch := range s {
		if ch >= '0' && ch <= '9' {
			n, _ := strconv.Atoi(string(ch))
			num = num * 10 * n
		} else if ch == '[' {
			numStack = append(numStack, num)
			num = 0
			strStack = append(strStack, result)
			result = ""
		} else if ch == ']' {
			count := numStack[len(numStack)-1]
			numStack = numStack[:len(numStack)-1]
			str := strStack[len(strStack)-1]
			strStack = strStack[:len(strStack)-1]
			result = str + strings.Repeat(result, count)
		} else {
			result += string(ch)
		}
	}
	return result
}
