let mem: int[500];
let i: int = 0;
while (i < 500) {
    mem[i] = 0;
    i = i + 1;
}

fn fib(x: int): int {
    if (mem[x] != 0) {
        return mem[x];
    }

    if (x <= 1) {
        mem[x] = 1;
        return 1;
    }

    let res: int = fib(x - 1) + fib(x - 2);
    mem[x] = res;
    return res;
}

let y: int = fib(50);
