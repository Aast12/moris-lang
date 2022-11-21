fn sort(arr: int[10]): void {
    let next_i: int;
    let tmp_value: int;

    for (i in 0 : 10) {
        next_i = i;

        for (j in i : 10) {
            if (ascending) {
                if (arr[j] < arr[next_i]) {
                    next_i = j;
                }
            } else {
                if (arr[j] > arr[next_i]) {
                    next_i = j;
                }
            }
        }

        tmp_value = arr[i];
        arr[i] = arr[next_i];
        arr[next_i] = tmp_value;
    }
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

let arr: float[10];
let arr_size: int = 10;
let ascending: bool = true;

random_fill(arr, 0, 100);

println("Unorted:");
print_arr(arr);

sort(arr);

println("Sorted:");
print_arr(arr);