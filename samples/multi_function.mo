fn fibonacci3(n: int): int {
    if (n <= 1) {
        return n;
    }

    return fibonacci2(n - 2) + fibonacci3(n - 2);
}

let x: int = 7;
let y: float = 6;
let z: float = x * y;

fn fibonacci(n: int): int {
    if (n <= 1) {
        return n;
    }

    return fibonacci(n - 2) + fibonacci(n - 2);
}

let q: bool = false;

fn fibonacci2(n: int): int {
    if (n <= 1) {
        return n;
    }

    return fibonacci2(n - 2) + fibonacci(n - 2);
}

let res: int = fibonacci3(10);