fn test0(n: int, k: int): int {
    println("Z Value: ", z, " k value: ", k);

    return test1(n);
}

let x: int = 7;
let y: float = 6;
let z: float = x * y;

fn test1(n: int): int {
    return n + y;
}

println("x: ", x);
println("y: ", y);
println("z: ", z);

println(test0(5, 0));