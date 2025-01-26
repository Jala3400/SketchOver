# Sketch Over

Es una aplicación usada para dibujar encima de la pantalla. Está inspirada en la aplicación para windows "ZoomIt" de Microsoft (https://docs.microsoft.com/en-us/sysinternals/downloads/zoomit). Solo la parte de dibujar.

Ofrece como ventajas:

-   Se ve actualizarse la pantalla de detrás.
-   Se puede borrar lo dibujado.
-   Se puede conservar lo dibujado después de ocultarlo
-   Se puede cambiar el color del fondo.

# Índice

- [Sketch Over](#sketch-over)
- [Índice](#índice)
- [Instalación](#instalación)
- [Uso](#uso)
  - [Inicio:](#inicio)
  - [Dibujar:](#dibujar)
  - [Cambiar colores:](#cambiar-colores)
  - [Cambiar monitores:](#cambiar-monitores)
- [Información útil:](#información-útil)

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

# Información útil:

Aunque no lo parezca, esta aplicación no pone la ventana en pantalla completa. Solo ha sido comprobado en Windows 10, pero si se pone en pantalla completa no se ve lo que hay detrás porque se pone en negro, seguramente porque el manejador de ventanas ahorra recursos porque hay una ventana que ocupa todo el monitor.

Esto se soluciona poniendo el tamaño de la ventana como el del monitor menos 1 pixel de altura. Sim embargo, algunas aplicaciones (como discord o youtube) dejan de actualizarse cuando hay otra ventana que las está tapando.

Es por esto por lo que se la ventana se pone en la posición (1, 1) en vez de la (0, 0) y tiene una altura y una anchura 2 píxeles menor a la del monitor (1 píxel por cada lado). Esto simulará un margen de 1 pixel de la ventana con respecto al monitor.

El que las aplicaciones completamente ocultas tras la ventana se actualicen o no dependerá del manejador de ventanas.
