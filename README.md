# Moris

## Status

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
- Pruebas unitarias para la sintaxis.
- Agregar `continue` y `break`.
- Implementación de acciones en reconocimiento de sintaxis.
