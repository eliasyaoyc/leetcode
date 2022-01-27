package hot_100

func permute(nums []int) [][]int {
	ans := [][]int{}
	visited := map[int]bool{}

	var dfs func(path []int)
	dfs = func(path []int) {
		if len(path) == len(nums) {
			temp := make([]int, len(path))
			// 这里拷贝的意思是：path变量存的是地址引用，结束当前递归时，将它加入ans后，还需要将这个 path 传给别的递归调用，它所指向的内存空间还需要继续被操作
			// 所以 ans 中的 path 内容会被改变，所以需要重新拷贝一份
			copy(temp, path)
			ans = append(ans, temp)
			return
		}
		for _, n := range nums {
			if visited[n] {
				continue
			}
			path = append(path, n)
			visited[n] = true
			dfs(path)
			path = path[:len(path)-1]
			visited[n] = false
		}
	}
	dfs([]int{})
	return ans
}
