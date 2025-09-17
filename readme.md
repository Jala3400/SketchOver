# Sketch Over

**Sketch Over** es una aplicación usada para dibujar encima de la pantalla. Está inspirada en la aplicación para windows ["ZoomIt"](https://docs.microsoft.com/en-us/sysinternals/downloads/zoomit) de Microsoft. Solo la parte de dibujar.

Ofrece como ventajas:

- **Pantalla actualizada en tiempo real**: El fondo se actualiza mientras dibujas.
- **Borrar dibujo**: Puedes borrar lo dibujado en cualquier momento.
- **Conserva el dibujo**: El último dibujo se mantiene al ocultar la aplicación (pero no después de cerrar la aplicación).
- **Cambio de fondo sin perder dibujo**: Puedes cambiar el color de fondo sin borrar lo dibujado.
- **Uso transparente del ratón**: Puedes usar el ratón detrás del canvas mientras dibujas (la ventana es transparente al ratón).

Aún no ofrece:

- **Añadir texto**.
- **Puntero láser**.
- **Guardar el dibujo**.
- **Configuración personalizada**.

## Índice

- [Sketch Over](#sketch-over)
  - [Índice](#índice)
  - [Instalación](#instalación)
    - [Prerrequisitos](#prerrequisitos)
    - [Pasos](#pasos)
  - [Uso](#uso)
    - [Inicio](#inicio)
    - [Dibujar](#dibujar)
    - [Cambiar colores](#cambiar-colores)
    - [Cambiar monitores](#cambiar-monitores)
  - [Otros](#otros)

## Instalación

### Prerrequisitos

Antes de empezar, asegúrate de tener instalados los siguientes programas:

- [**Rust**](https://www.rust-lang.org/)
- [**Git**](https://git-scm.com/)

### Pasos

1. **Clona este repositorio**:
   Abre la terminal y ejecuta:

    ```bash
    git clone https://github.com/Jala3400/SketchOver
    ```

2. **Compila el repositorio**:
   Entra en la carpeta del repositorio y ejecuta:

    ```bash
    cargo build --release
    ```

## Uso

### Inicio

Para iniciar la aplicación, ejecuta:

```bash
cargo run --release
```

Esto iniciará la aplicación en segundo plano, y se verá un ícono de lápiz en la barra de tareas.

Al hacer clic en el icono se mostrará un menú con las siguientes opciones junto con sus atajos de teclado:

- **Transparent to mouse**: Permite usar el ratón detrás de la ventana de dibujo.  
    _Atajo:_ **Ctrl+Alt+T**

- **New canvas**: Abre un canvas en blanco para empezar a dibujar.  
    _Atajo:_ **Ctrl+Alt+S**

- **Show**: Muestra el canvas anterior.  
    _Atajo:_ **Ctrl+Alt+R**

- **Hide**: Oculta el dibujo.  
    _Atajo:_ **Esc**

- **Exit**: Cierra la aplicación.  
    _Atajo:_ **Ctrl+Alt+Q**

**Extra**: Si haces doble clic en el ícono del lápiz, se abrirá el dibujo anterior.

> **Nota**: Los atajos de teclado solo funcionan cuando la aplicación está en primer plano, excepto el atajo de "Show", que se puede usar en cualquier momento.

### Dibujar

- **Para dibujar**: Se mantiene presionado el botón izquierdo del ratón y arrastra.
- **Para cambiar el tamaño del pincel**: Se usa la rueda del ratón.
- **Para limpiar el canvas**: Se pulsa **Ctrl+Alt+C**.
- **Para dibujar líneas rectas**: Se mantiene presionado **Shift** mientras se dibuja.
- **Para dibujar cuadrados**: Se mantiene presionado **Ctrl** mientras se dibuja.
- **Para deshacer**: Se pulsa **Ctrl+Z**.
- **Para rehacer**: Se pulsa **Ctrl+Y**.

### Cambiar colores

Presiona las siguientes teclas para cambiar el color del pincel:

- **R**: Rojo
- **G**: Verde
- **B**: Azul
- **Y**: Amarillo
- **C**: Cian
- **M**: Magenta
- **W**: Blanco
- **K**: Negro (La letra A también, que está más cerca del ctrl)
- **O**: Naranja
- **P**: Morado
- **I**: Rosa
- **N**: Marrón
- **H**: Gris
- **V**: Azul marino
- **T**: Verde azulado

**Para cambiar el color de fondo**:
Se mantiene presionado **Shift** y pulsa la tecla del color que desees.

**Para borrar**:
Se pulsa **Espacio** para usar la goma. Si se presiona **Ctrl+Espacio**, el fondo se volverá transparente.

### Cambiar monitores

Se puede cambiar el monitor en el que se muestra el dibujo de varias formas:

- **Tab**: Cambia el monitor en el que está el dibujo.
- **Ctrl+Alt+S**: Muestra la aplicación en el monitor actual, pero borrará el dibujo.
- **Ctrl+Alt+R**: Mueve la ventana al monitor correspondiente, manteniendo el dibujo.

## Otros

Algunas aplicaciones como YouTube o Discord (compartir pantalla) dejarán de actualizarse si están completamente cubiertas por el canvas.
El canvas tiene un margen de 1 píxel por cada lado, por lo que si la aplicación toca un borde, no debería haber problemas.
