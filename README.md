# Moris Language

## Build from source

To build this compiler, you must use rust and cargo to build a binary:

1. Produce a release build.

  ```bash
  cargo build -r
  ```

2. Executable will live under target/release.
3. The executable will receive a file path for a Moris file to execute. e.g.

```bash
target/release/moris ./local_program.mo
```

## User Manual

[Video Demo](https://youtu.be/cAxQNM8lj6c)

The programming language supports basic procedural language characteristics:

### Data Types

The language supports `int`, `float`, `str`, `bool`, `DataFrame`, and `Series` types.
These can be used to assign variables and parameters to functions, as well to their return types.

Arrays and matrices can be declares for any of these types, but functions can only return **scalars**.

### Variable declarations

In Moris, a variable declaration requires explicitly setting it's type, using the following notation:

```moris
let var_name: type_id;
let var_name: type_id = value;
```

Declaring variables for each data type will look like:

```moris
let int_var_und: int;
let int_var: int = 5;

let float_var: float = 2.5;

let bool_var: bool = false;

let str_var: str = "string";
```

For other types, they can be declared, but their value should be assigned through special functions. For arrays and matrices, by assigning values to each individual element or through special functions.

### Array indexing

Items of arrays or matrices can be accessed with the following syntax:

```moris
let int_arr: int[10];

int_arr[0] = 10;
```

### Expressions

The language supports basic operations and expression evaluation, including the following operators:

| Operator type | operators       |
| ------------- | --------------- |
| Arithmetic    | + - * /         |
| Logic         | ! && \|\|       |
| Comparison    | > < >= <= != == |

#### **Pipes**

The language has a pipe operator `|>`, that allows the chaining of functions, propagating the return value from a function as the argument to the next one. For example, the following code:

```moris
result = input |> func1 |> func2 |> func3;
```

Would be equivalent to:

```moris
result_0 = func1(input);

result_1 = func2(result_0);

result = func3(result_1);
```

### Functions

Functions are declared with the following syntax:

```moris
fn function_id(param_0: type_id_0, param_1: type_id_1, ...) : return_type_id {
  ... function body ...
}
```

The parameters can be defined just as local and global variables, and they can return any scalar type, or have a `void` return type.

For example, a fibonacci function can be defined like:

```moris
fn fibonacci(n: int): int {
    if (n <= 0) {
        return 0;
    }
    if (n <= 2) {
        return n;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}
```

### Special Functions

The language include native functions that serve as utility to deal and explore numeric data and perform I/O operations. 

#### **I/O**

| Function  | Params                     | Return Type | Description                                                                                                                    |
| --------- | -------------------------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------ |
| `read`    | ind. amount of variables   | `void`        | Reads an input line split by spaces and assigns the value of each splitted element into their corresponding parameter variable |
| `print`   | ind. amount of expressions | `void`        | Prints each expression passed as parameter in order                                                                            |
| `println` | ind. amount of expressions | `void`        | Prints each expression passed as parameter in order, prints a new line at the end                                              |


#### **DataFrames**

| Function      | Params                      | Return Type | Description                                                  |
| ------------- | --------------------------- | ----------- | ------------------------------------------------------------ |
| `read_csv`    | path: `str`                 | DataFrame   | Returns the dataframe read from the local path `path`        |
| `select`      | df: `DataFrame`, col: `str` | Series      | Returns the column `col` from the dataframe `df` as a Series |
| `print_names` | df: `DataFrame`             | void        | Prints the column names of the input dataframe               |
| `describe`    | df: `DataFrame`             | void        | Prints a description summary of the dataframe's contents     |


#### **Charts**

| Function       | Params                     | Return Type | Description                                                        |
| -------------- | -------------------------- | ----------- | ------------------------------------------------------------------ |
| `set_caption`  | title: `str`               | `void`      | Sets the title of the next plotted chart.                          |
| `set_x_title`  | title: `str`               | `void`      | Sets the title of the x axis for the next plotted chart.           |
| `set_y_title`  | title: `str`               | `void`      | Sets the title of the y axis for the next plotted chart.           |
| `set_x_bounds` | min: `float`, max: `float` | `void`      | Prints the column names of the input dataframe                     |
| `set_y_bounds` | min: `float`, max: `float` | `void`      | Prints the column names of the input dataframe                     |
| `set_plot_out` | path: `str`                | `void`      | Sets the output path of the chart image for the next plotted chart |
| `scatter`      | x: `Series`, y: `Series`   | `void`      | Plots a scatter plot with the x and y values                       |


#### **Utils / Random**

| Function      | Params            | Return Type | Description                              |
| ------------- | ----------------- | ----------- | ---------------------------------------- |
| `zeros`       | array of any type | `void`      | Fills the input array with zeroes        |
| `random`      |                   | `float`     | Returns a random number between 0 and 1  |
| `random_fill` | array of any type | `void`      | Fills the input array with random values |

#### **Statistics**

There are also definitions of `mean`, `median`, `std`, `sum`, `var`, which all receive an array or matrix of numeric values and returns their corresponding statistic value.