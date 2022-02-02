package hot_100

func exist(board [][]byte, word string) bool {
	m, n := len(board), len(board[0])
	used := make([][]bool, m)
	for i := 0; i < m; i++ {
		used[i] = make([]bool, n)
	}

	var canFind func(r, c, i int) bool
	canFind = func(r, c, i int) bool {
		if i == len(word) {
			return true
		}
		if r < 0 || r >= m || c < 0 || c >= n {
			return false
		}
		if used[r][c] || board[r][c] != word[i] {
			return false
		}
		used[r][c] = true
		canFindRest := canFind(r+1, c, i+1) || canFind(r-1, c, i+1) ||
			canFind(r, c+1, i+1) || canFind(r, c-1, i+1)

		if canFindRest {
			return true
		} else {
			used[r][c] = false
			return false
		}
	}

	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			if board[i][j] == word[0] && canFind(i, j, 0) {
				return true
			}
		}
	}
	return false
}
