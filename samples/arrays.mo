let matrix: float[10][10];

let i: int = 0; 
let j: int = 0;
while (i < 10) {
    j = 0;
    while (j < 10) {
        matrix[i][j] = i * j * 5;
        println(i, ", ", j, " -> ", matrix[i][j]);
        j = j + 1;
    }
    i = i + 1;
}

println("RES", matrix[2][3]);

let row: int[10] = matrix[2];
println("ROW", row[3]);
row[3] = 77;
println("MAT", matrix[2][3]);
