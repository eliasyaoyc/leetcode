package hot_100

import s "sort"

func reconstructQueue(people [][]int) [][]int {
	s.Slice(people, func(i, j int) bool {
		if people[i][0] == people[j][0] {
			return people[i][1] < people[j][1]
		}
		return people[i][0] > people[j][0]
	})

	result := make([][]int, 0)
	for _, p := range people {
		result = append(result, p)
		copy(result[p[1]+1:], result[p[1]:])
		result[p[1]] = p
	}
	return result
}
