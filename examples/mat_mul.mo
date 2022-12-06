let n: int = 4;
let m: int = 3;
let p: int = 3;

fn mat_mul(mat_a: float[4][3], mat_b: float[3][3], dest: float[4][3]): void {
    let sum: float = 0;

    for (i in 0:n) {
        for (j in 0:p) {
            sum = 0;
            for (k in 0:m) {
                sum = sum + mat_a[i][k] * mat_b[k][j];        
            }
            dest[i][j] = sum;
        }   
    }
}

let A: float[4][3];
let B: float[3][3];
let C: float[4][3];

random_fill(A, 2, 5);
random_fill(B, 2, 10);

mat_mul(A, B, C);

println("Matrix A:");
for (i in 0:n) {
    for (j in 0:m) {
        print(A[i][j], " ");
    }
    println();
}

println("Matrix B:");
for (i in 0:m) {
    for (j in 0:p) {
        print(B[i][j], " ");
    }
    println();
}

println("Product:");
for (i in 0:n) {
    for (j in 0:p) {
        print(C[i][j], ", ");
    }
    println();
}