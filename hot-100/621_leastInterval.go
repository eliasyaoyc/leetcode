package hot_100

func leastInterval(tasks []byte, n int) int {
	if n == 0 {
		return len(tasks)
	}
	m := map[byte]int{}
	for _, t := range tasks {
		m[t]++
	}
	maxSize, maxCount := 0, 0
	for _, c := range m {
		if c > maxSize {
			maxSize, maxCount = c, 1
		} else if c == maxSize {
			maxCount++
		}
	}
	if time := (maxSize-1)*(n+1) + maxCount; time > len(tasks) {
		return time
	}
	return len(tasks)
}
