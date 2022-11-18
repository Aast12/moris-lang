let mem: float[500];
let i: int = 0;
while (i < 500) {
    mem[i] = - 1;
    i = i + 1;
}

fn fib(x: int): int {
    if (mem[x] != -1) {
        return mem[x];
    }

    if (x <= 1) {
        mem[x] = x;
        return x;
    }

    let res: int = fib(x - 1) + fib(x - 2);
    mem[x] = res;
    return res;
}

let y: int = fib(50);
