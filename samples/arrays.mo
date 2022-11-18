let mem: int[10];
let i: int = 0;
while (i < 10) {
    mem[i] = i;
    i = i + 1;
}

fn array_mod(arr: int[10]): int {
    arr[2] = 123 + arr[2];
    arr[0] = 12; 
    return arr[2];
}


fn array_item(arr_item: int): int {
    arr_item = arr_item * 2;
    return arr_item;
}

let x: int = array_mod(mem);
let y: int = array_item(mem[4]);
let z: int = mem[4];