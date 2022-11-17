fn fib(x: int): int {
    if (x <= 2) {
        return x;
    }

    return fib(x - 1) + fib(x - 2);
}

let y: int = fib(10);

if (!(y > 50)) {
    y = 2022;
}

