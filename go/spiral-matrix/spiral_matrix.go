package spiralmatrix

func SpiralMatrix(size int) [][]int {

	spiralMatrix := make([][]int, size)
	for i := range spiralMatrix {
		spiralMatrix[i] = make([]int, size)
	}

	top := 0
	bottom := size - 1
	left := 0
	right := size - 1
	number := 1
	for top <= bottom && left <= right {
		// top row from left to right
		for i := left; i <= right; i++ {
			spiralMatrix[top][i] = number
			number++
		}
		top++

		// right column from top to bottom
		for i := top; i <= bottom; i++ {
			spiralMatrix[i][right] = number
			number++
		}
		right--

		// bottom row from right to left
		if top <= bottom {
			for i := right; i >= left; i-- {
				spiralMatrix[bottom][i] = number
				number++
			}
			bottom--
		}

		// left column from bottom to top
		if left <= right {
			for i := bottom; i >= top; i-- {
				spiralMatrix[i][left] = number
				number++
			}
			left++
		}
	}

	return spiralMatrix
}
