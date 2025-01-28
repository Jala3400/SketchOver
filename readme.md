# Sketch Over

Es una aplicación usada para dibujar encima de la pantalla. Está inspirada en la aplicación para windows ["ZoomIt"](https://docs.microsoft.com/en-us/sysinternals/downloads/zoomit) de Microsoft. Solo la parte de dibujar.

Ofrece como ventajas:

-   Se ve actualizarse la pantalla de detrás.
-   Se puede borrar.
-   Se puede conservar el dibujo después de ocultarlo (no después de cerrar la aplicación).
-   Se puede cambiar el color del fondo sin borrar lo dibujado.
-   Opción de usar el mouse tras el canvas mientras se está dibujando (la ventana es transparente al mouse).

Aún no ofrece:

-   Lineas rectas, cuadrados, etc.
-   Texto.

# Índice

- [Sketch Over](#sketch-over)
- [Índice](#índice)
- [Instalación](#instalación)
- [Uso](#uso)
  - [Inicio:](#inicio)
  - [Dibujar:](#dibujar)
  - [Cambiar colores:](#cambiar-colores)
  - [Cambiar monitores:](#cambiar-monitores)
- [Otros:](#otros)

# Instalación

-   Prerrequisitos:

    -   [Rust](https://www.rust-lang.org/tools/install)
    -   [Git](https://git-scm.com/downloads)

-   Pasos:

1. Descarga este repositorio con:
    ```
    git clone https://github.com/Jala3400/SketchOver
    ```
2. Compila el repositorio: En la consola de comandos ejecuta
    ```
    cargo build --release
    ```

# Uso

## Inicio:

-   Para iniciar la aplicación ejecuta:

    ```
    cargo run --release
    ```

Esto iniciará la aplicación en segundo plano, por lo que se verá un icono de un lápiz en la barra de tareas.

Al pulsar este icono se mostrarán varias opciones junto con sus atajos de teclado:

-   Transparent to mouse: Permite usar el ratón detrás de la ventana de dibujo (Ctrl+Alt+T).
-   New canvas: Entra en modo dibujo con un canvas en blanco (Ctrl+Alt+S).
-   Show: Entra el modo dibujo con el canvas anterior (Ctrl+Alt+Shift+S).
-   Hide: Oculta el dibujo (Esc).
-   Exit: Cierra la aplicación (Ctrl+Alt+Q).

Además, si se hace doble click en el icono o se pulsa (Ctrl+Alt+Shift+S) se abrirá el dibujo anterior.

Cada acción se puede llamar tanto pulsando el botón como con el atajo de teclado.
Los atajos de teclado se deben pulsar cuando la aplicación está en primer plano, excepto el de "Show" que se puede pulsar en cualquier momento.

## Dibujar:

Para dibujar se pulsa el botón izquierdo del ratón y se arrastra.

Para cambiar el radio del pincel se usa la rueda del ratón.

## Cambiar colores:

Se cambian pulsando teclas:

-   R: Rojo
-   G: Verde
-   B: Azul
-   Y: Amarillo
-   C: Cian
-   M: Magenta
-   W: Blanco
-   K: Negro

Si se pulsa una tecla manteniendo el Ctrl pulsado, se cambiará el color de fondo.

Para borrar se pulsa el espacio. Si se pulsa Ctrl+espacio el color de fondo volverá a ser transparente.

## Cambiar monitores:

Se puede cambiar el monitor en el que está el dibujo de varias maneras:

-   Tab: Al pulsar el tabulador se cambiará el monitor en el que está manteniendo el dibujo
-   Ctrl+Alt+S: (Borra el dibujo) Es el comando con el que se muestra la aplicación. La iniciará en el monitor en el que se esté.
-   Ctrl+Alt+Shift+S: (No borra el dibujo). Moverá la ventana al monitor correspondiente.

# Otros:

Hay aplicaciones (como youtube o compartir pantalla en discord) que dejarán de actualizarse si están completamente cubiertas por el canvas.

El canvas tiene un márgen de 1 pixel por cada lado, por lo que si la aplicación toca un borde no debería haber problemas.
