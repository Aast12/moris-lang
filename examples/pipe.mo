fn step0(x: int): int {
    return x + 3;
}

fn step1(x: int): int {
    return x * 2;
}

fn step2(x: int): int {
    return x / 16;
}

let input:int = 10;

let res: int = (input + 2) |> step0 |> step1 |> step2;

println("Piped result", res);