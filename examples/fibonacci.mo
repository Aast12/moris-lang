let x: int = 7;
let y: float = 6;
let z: float = x * y;

fn fibonacci(n: int): int {
    if (n <= 0) {
        return 0;
    }
    if (n <= 2) {
        return n;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}

let result: int = fibonacci(10);

println(result);