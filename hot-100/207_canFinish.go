package hot_100

// example  numCourses: 6, prerequisites:[[3,0],[3,1],[4,1],[4,2],[5,3],[5,4]]
//   0
//        3
//   1         5
//        5
//   2
func canFinish(numCourses int, prerequisites [][]int) bool {
	// 课号和对应的入度
	inDegree := map[int]int{}
	// 将所有的课程先放入
	for i := 0; i < numCourses; i++ {
		inDegree[i] = 0
	}
	// 当前课程和后续课程的依赖关系
	adj := map[int][]int{}

	// 初始化入度和依赖关系
	for i := range prerequisites {
		// (3,0) 想学 3 必须先学0
		cur := prerequisites[i][1]
		next := prerequisites[i][0]
		// 更新入度
		inDegree[next]++
		// 更新依赖关系
		if _, exist := adj[cur]; !exist {
			adj[cur] = []int{}
		}
		adj[cur] = append(adj[cur], next)
	}

	// 将入度为0 的课程放入队列，也就是 example 中的 0,1,2
	q := []int{}
	for i := range inDegree {
		if inDegree[i] == 0 {
			q = append(q, i)
		}
	}

	for len(q) != 0 {
		// 取出一个节点学习这个课程，然后更新 入度--，查看入度是否为0， 0 则加入队列
		nQ, p := pop(q)
		q = nQ
		if _, exist := adj[p]; !exist {
			continue
		}
		for _, v := range adj[p] {
			inDegree[v]--
			if inDegree[v] == 0 {
				q = append(q, v)
			}
		}
	}
	for i := range inDegree {
		if inDegree[i] != 0 {
			return false
		}
	}
	return true
}

func canFinish1(numCourses int, prerequisites [][]int) bool {
	var adj = make([][]int, numCourses)
	var inDegree = make([]int, numCourses)
	var result []int
	for _, info := range prerequisites {
		adj[info[1]] = append(adj[info[1]], info[0])
		inDegree[info[0]]++
	}
	q := []int{}
	for i := 0; i < numCourses; i++ {
		if inDegree[i] == 0 {
			q = append(q, i)
		}
	}

	for len(q) > 0 {
		u := q[0]
		q = q[1:]
		result = append(result, u)
		for _, v := range adj[u] {
			inDegree[v]--
			if inDegree[v] == 0 {
				q = append(q, v)
			}
		}
	}
	return len(result) == numCourses
}
