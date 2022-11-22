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

random_fill(arr, 0, 50);

println("Array:");
print_arr(arr);

let mean_v: float = mean(arr);
let std_v: float = std(arr);
let var_v: float = var(arr);
let median_v: float = median(arr);

