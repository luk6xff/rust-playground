pub fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut res = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            res[i][j] = matrix[j][i];
        }
    }
    res
}