package hot_100

func rotate(matrix [][]int) {
	// 水平线翻转
	for i := 0; i < len(matrix)/2; i++ {
		matrix[i], matrix[len(matrix)-1-i] = matrix[len(matrix)-1-i], matrix[i]
	}
	// 对角线翻转
	for i := 0; i < len(matrix); i++ {
		for j := 0; j < i; j++ {
			temp := matrix[i][j]
			matrix[i][j] = matrix[j][i]
			matrix[j][i] = temp
		}
	}
}
