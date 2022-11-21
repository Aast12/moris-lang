let arr: int[10];

zeros(arr);

for (i in 0:10:2) {
    arr[i] = i * 2;
}

for(i in 1 : 10 : 2) {
    arr[i] = i * 3;
}

for (i in 0:10) {
    println(i, arr[i]);
}