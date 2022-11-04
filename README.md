# Moris Lang

**Autor:** Andrés Alam Sánchez Torres (A00824854)

## Status

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
