let x: int = 7;
let y: float = 6;
let z: float = x * y;

while (x * 2 + y * y < z) {
    if (x == 10) {
        break;
    }

    x = x + 1;
    y = y * 2;
}