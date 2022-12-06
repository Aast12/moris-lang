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

fn fib_iter(n: int): int {
    let acc: int = 0;
    let fst: int = 0;
    let snd: int = 1;

    for (index in 1:n) {
        acc = fst + snd;
        fst = snd;
        snd = acc;
    }

    return acc;
}

let test_n: int = 20;

let recursive_result: int = fib(test_n);
let iter_result: int = fib_iter(test_n);

println("Test for n = ", test_n);
println("Iterative fib: ", iter_result);
println("recursive_result: ", recursive_result);

