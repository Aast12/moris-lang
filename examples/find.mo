fn find(arr: int[10], item: int): int {
    for (i in 0 : 10) {
        println("Check item ", item, " against ", arr[i]);
        if (item == arr[i]) {
            return i;
        }
    }

    println("not found");
    return -1;
}

fn print_arr(arr: int[10]): void {
    for (i in 0 : 10) {        
        if (i == 10 - 1) {
            println(arr[i]);
        } else {
            print(arr[i], ", ");
        }
    }
}

let arr: int[10];
random_fill(arr, 0, 10);

println("Target array");
print_arr(arr);

let target: int = 5;
let found_idx: int = find(arr, target);

if (found_idx != -1) {
    println("Found target ", target, " in index ", found_idx);
} else {
    println("Target ", target, " was not found!");
}