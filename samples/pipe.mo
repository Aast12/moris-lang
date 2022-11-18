fn step0(x: int): int {
    return x + 3;
}

fn step1(x: int): int {
    let i: int = 0;
    let acc: int = 0;
    while (i < x) {
        acc = x * i;
        i = i + 1;
    }

    return acc;
}

fn step2(x: int): int {
    return x / 16;
}

let input:int = 1220;

let res: int = (input + 2) |> step0 |> step1 |> step2;