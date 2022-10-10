# Moris Lang

**Autor:** Andrés Alam Sánchez Torres (A00824854)

## Status

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
