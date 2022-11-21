let q: str = "hola";
let k: str = "mundo";
let concat: str = q + " " + k ;
let x: int = 42;

println("Valor de concat: ", concat, x);

let str_arr: str[10];

let i: int = 1;
str_arr[0] = "fizz";
let last: str = "fizz";

while (i < 10) {
    if (last == "fizz") {
        last = "buzz";
    } else {
        last = "fizz";
    }

    str_arr[i] = str_arr[i - 1] + last;
    println(str_arr[i]);

    i = i + 1;
}

