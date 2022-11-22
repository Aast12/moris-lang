# Moris Lang

**Autor:** Andrés Alam Sánchez Torres (A00824854)

## Estátus

## Noviembre 15, 2022

- Ejecución
  - La máquina virtual ejecuta código para comparaciones lógicas (>, <, >=, <=, !=, ==) con Floats, Ints y Bools.
  - Se ejecuta código para estatutos if-else,

## Noviembre 14, 2022

- Generación de cuadruplos
  - Arreglos
    - Genera cuadruplos y asigna memoria para arreglos y matrices.
- Ejecución
  - La máquina virtual ejecuta código para operaciones básicas (=, +, -, *, /) con Floats, Ints y Bools.


## Noviembre 7, 2022

- Generación de cuadruplos
  - Funciones
    - Se resetean y asignan locales para cada función distinta.
    - Se asigna una dirección global para el valor de retorno de cada función.
    - La firma de cada función se pre-define en las tablas de símbolos antes de generar los cuádruplos (permite referencias a funciones declaradas después de la línea donde se llama).
    - Antes de generar cuádruplos, el código de las funciones se mueve al final del archivo para permitir la declaración de las variables globales.

### Ejemplo de cuádruplos Generados

#### **Input**

```moris
fn fibonacci3(n: int, k: int): int {
    if (n <= 1 || z == 3) {
        return n;
    }

    return fibonacci2(n - 2) + fibonacci3(n - 2, n);
}

let x: int = 7;
let y: float = 6;
let z: float = x * y;

fn fibonacci(n: int): int {
    if (n <= 1) {
        return n;
    }

    return fibonacci(n - 2) + fibonacci(n - 2);
}

let q: bool = false;

fn fibonacci2(n: int): int {
    if (n <= 1) {
        return n;
    }

    return fibonacci2(n - 2) + fibonacci(n - 2);
}

let res: int = fibonacci3(10, 7.2);
```

#### **Output**

|     |            |       |       |            |
| --- | ---------- | ----- | ----- | ---------- |
| 0   | =          | 7     |       | 14003      |
| 1   | Float      | 6     |       | 12001      |
| 2   | =          | 12001 |       | 12000      |
| 3   | *          | 14003 | 12000 | 12003      |
| 4   | =          | 12003 |       | 12002      |
| 5   | =          | false |       | 10000      |
| 6   | era        |       |       | fibonacci3 |
| 7   | param      | 10    |       | 0          |
| 8   | Int        | 7.2   |       | 14005      |
| 9   | param      | 14005 |       | 1          |
| 10  | gosub      |       |       | fibonacci3 |
| 11  | =          | 14002 |       | 14004      |
| 12  | endprogram |       |       |            |
| 13  | <=         | 24000 | 1     | 20000      |
| 14  | ==         | 12002 | 3     | 20001      |
| 15  | \| \|      | 20000 | 20001 | 20002      |
| 16  | gotoFalse  | 20002 |       | 18         |
| 17  | return     |       |       | 24000      |
| 18  | era        |       |       | fibonacci2 |
| 19  | -          | 24000 | 2     | 24000      |
| 20  | param      | 24000 |       | 0          |
| 21  | gosub      |       |       | fibonacci2 |
| 22  | era        |       |       | fibonacci3 |
| 23  | -          | 24000 | 2     | 24001      |
| 24  | param      | 24001 |       | 0          |
| 25  | param      | 24000 |       | 1          |
| 26  | gosub      |       |       | fibonacci3 |
| 27  | +          | 14000 | 14002 | 24002      |
| 28  | return     |       |       | 24002      |
| 29  | endfunc    |       |       |            |
| 30  | <=         | 24000 | 1     | 20000      |
| 31  | gotoFalse  | 20000 |       | 33         |
| 32  | return     |       |       | 24000      |
| 33  | era        |       |       | fibonacci  |
| 34  | -          | 24000 | 2     | 24000      |
| 35  | param      | 24000 |       | 0          |
| 36  | gosub      |       |       | fibonacci  |
| 37  | era        |       |       | fibonacci  |
| 38  | -          | 24000 | 2     | 24001      |
| 39  | param      | 24001 |       | 0          |
| 40  | gosub      |       |       | fibonacci  |
| 41  | +          | 14001 | 14001 | 24002      |
| 42  | return     |       |       | 24002      |
| 43  | endfunc    |       |       |            |
| 44  | <=         | 24000 | 1     | 20000      |
| 45  | gotoFalse  | 20000 |       | 47         |
| 46  | return     |       |       | 24000      |
| 47  | era        |       |       | fibonacci2 |
| 48  | -          | 24000 | 2     | 24000      |
| 49  | param      | 24000 |       | 0          |
| 50  | gosub      |       |       | fibonacci2 |
| 51  | era        |       |       | fibonacci  |
| 52  | -          | 24000 | 2     | 24001      |
| 53  | param      | 24001 |       | 0          |
| 54  | gosub      |       |       | fibonacci  |
| 55  | +          | 14000 | 14001 | 24002      |
| 56  | return     |       |       | 24002      |
| 57  | endfunc    |       |       |            |

## Noviembre 4, 2022

- Mapeo de memoria:

  Se asignan las siguientes direcciones para cada tipo de alcance:
  
  | Alcance  | Rango de direcciones |
  | -------- | -------------------- |
  | Global   | 10,000 - 19,999      |
  | Local    | 20,000 - 29,999      |
  | Constant | 30,000 - 39,999      |

  Para cada alcance, se especifican los siguientes rangos para cada tipo de dato:

  | Tipo      | Rango de direcciones |
  | --------- | -------------------- |
  | Bool      | 0 - 1,999            |
  | Float     | 2,000 - 3,999        |
  | Int       | 4,000 - 5,999        |
  | String    | 6,000 - 7,999        |
  | DataFrame | 8,000 - 9,999        |

- Generación de cuadruplos
  - loops
    - Implementación completa con continue y break statement.
    - *Implementación de for faltante, requiere de la implementación de arreglos.

### Ejemplo de cuádruplos Generados

#### **Input**

```moris
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
```

#### **Output**

|     |           |       |       |       |
| --- | --------- | ----- | ----- | ----- |
| 0   | =         | 7     |       | 14000 |
| 1   | Float     | 6     |       | 12001 |
| 2   | =         | 12001 |       | 12000 |
| 3   | *         | 14000 | 12000 | 12003 |
| 4   | =         | 12003 |       | 12002 |
| 5   | *         | 14000 | 2     | 14001 |
| 6   | *         | 12000 | 12000 | 12004 |
| 7   | +         | 14001 | 12004 | 12005 |
| 8   | <         | 12005 | 12002 | 10000 |
| 9   | gotoFalse | 10000 |       | 18    |
| 10  | ==        | 14000 | 10    | 10001 |
| 11  | gotoFalse | 10001 |       | 13    |
| 12  | goto      |       |       | 18    |
| 13  | +         | 14000 | 1     | 14002 |
| 14  | =         | 14002 |       | 14000 |
| 15  | *         | 12000 | 2     | 12006 |
| 16  | =         | 12006 |       | 12000 |
| 17  | goto      |       |       | 5     |

## Octubre 12, 2022

- Validación de tipos en expresiones y asignaciones de variable
- Generación de cuadruplos
  - expresiones
  - conversiones de tipos en asignación (e.g. asignación de un int a float para asignar a una variable)
  - condicionales

### Ejemplo de cuádruplos Generados

#### **Input**

```moris
let x: int = 5;
let z: int = 7 + 2 / x;


if (z > 4) {
    x = 6;
} else {
    x = 9;
    z = 4;
}

let y: bool;
let w: float = 7 * x + 3 / 2;

if (w == 7) {
    y = false;
}
```

#### **Output**

|     |           |       |      |      |
| --- | --------- | ----- | ---- | ---- |
| 0   | =         | 5     |      | x    |
| 1   | /         | 2     | x    | tmp0 |
| 2   | +         | 7     | tmp0 | tmp1 |
| 3   | Int       | tmp1  |      | tmp2 |
| 4   | =         | tmp2  |      | z    |
| 5   | >         | z     | 4    | tmp3 |
| 6   | gotoFalse |       |      | 9    |
| 7   | =         | 6     |      | x    |
| 8   | goto      |       |      | 11   |
| 9   | =         | 9     |      | x    |
| 10  | =         | 4     |      | z    |
| 11  | *         | 7     | x    | tmp4 |
| 12  | /         | 3     | 2    | tmp5 |
| 13  | +         | tmp4  | tmp5 | tmp6 |
| 14  | =         | tmp6  |      | w    |
| 15  | ==        | w     | 7    | tmp7 |
| 16  | gotoFalse |       |      | 18   |
| 17  | =         | false |      | y    |

### Octubre 10, 2022

- Definición de reglas semánticas. Clase que permite obtener el tipo resultado de todas las operaciones y alertar cuando hay un error de tipo.

### Octubre 3, 2022

- Definición de la gramática del lenguaje
- Definición de estructuras de datos para almacenar elementos de sintaxis (Ver src/syntax.rs). Los elementos estructurados se extraen durante el parsing del programa:
  - Enum para tipos de datos
  - Indexado (notación [])
  - Declaración de variable
  - Referencia de variable (id o acceso a arreglo)
  - Constantes
  - Operadores
  - Definición de funciones
  - Llamadas de funcion
  - If/Else
  - For loop
  - While
  - Programa

Por hacer / Siguientes pasos:

- Documentación de sintaxis (diagramas).
- Mejorar pruebas para la sintaxis, actualmente solo hay pruebas para validar si un programa es válido.
- Agregar `continue` y `break`.
- Operador `NOT`.
- Implementación de acciones en reconocimiento de sintaxis.
