let x: int = 7;
let y: float = 6;
let z: float = x * y;

fn fibonacci(n: int): int {
    if (n <= 1) {
        return n;
    }

    return fibonacci(n - 2) + fibonacci(n - 2);
}