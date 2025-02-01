

# Sketch Over 🎨

**Sketch Over** es una aplicación que te permite dibujar encima de tu pantalla, ideal para anotaciones rápidas, presentaciones o cualquier actividad que necesite una capa de dibujo en tiempo real. Inspirada en la herramienta **ZoomIt** de Microsoft, esta aplicación se centra solo en la parte de dibujar.

<!--
<p align="center">
  <img src="https://github.com/user-attachments/assets/f8bbba59-88d2-45d9-b245-58b5f3f3ee49" width="400" />
</p>
-->

<!--
<p align="center">
  <img src="https://github.com/user-attachments/assets/a380cccd-a4aa-434e-8b50-544137462b9c" width="300" />
</p>
-->

<p align="center">
  <img src="https://github.com/user-attachments/assets/576d8523-3fe0-4381-82da-891e7f4c51df" width="400" />
</p>








## 🚀 Características

### Ventajas:
- **Pantalla actualizada en tiempo real**: El fondo se actualiza mientras dibujas.
- **Borrar dibujo**: Puedes borrar lo dibujado en cualquier momento.
- **Conserva el dibujo**: Los dibujos se mantienen aunque ocultes la ventana (pero no después de cerrar la aplicación).
- **Cambio de fondo sin perder dibujo**: Puedes cambiar el color de fondo sin borrar lo dibujado.
- **Uso transparente del ratón**: Puedes usar el ratón detrás del canvas mientras dibujas (la ventana es transparente al ratón).

### Funcionalidades a futuro:
- **Ctrl+Z**: Deshacer.
- **Añadir texto**.
- **Puntero láser**.
- **Guardar el dibujo**.
- **Configuración personalizada**.



## 📝 Índice

1. [Instalación](#🔧-instalación)
2. [Uso](#🖥️-uso)
   - [Video](#video)
   - [Inicio](#inicio)
   - [Dibujar](#dibujar)
   - [Cambiar colores](#cambiar-colores)
   - [Cambiar monitores](#cambiar-monitores)
   - [Otros](#otros)




## 🔧 Instalación

### Prerrequisitos:
Antes de empezar, asegúrate de tener instalados los siguientes programas:

- [**Rust**](https://www.rust-lang.org/): Un lenguaje de programación moderno y eficiente.
- [**Git**](https://git-scm.com/): Sistema de control de versiones.

### Pasos para la instalación:

1. **Clona este repositorio**:
   Abre la terminal y ejecuta:
   ```bash
   git clone https://github.com/Jala3400/SketchOver
   ```

2. **Compila el repositorio**:
   Dirígete a la carpeta del repositorio y ejecuta:
   ```bash
   cargo build --release
   ```



## 🖥️ Uso

### 🎥 Video

Si quieres ver **Sketch Over** en acción, mira el siguiente video demostrativo:

......


### Inicio

Para iniciar la aplicación, ejecuta:

```bash
cargo run --release
```

Esto iniciará la aplicación en segundo plano, y verás un ícono de lápiz en la barra de tareas.

Al hacer clic en el ícono, se mostrará un menú con las siguientes opciones, junto con sus atajos de teclado:



### 🌟 Opciones disponibles:

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



### ✏️ Dibujar

- **Para dibujar**: Mantén presionado el botón izquierdo del ratón y arrastra.
- **Para cambiar el tamaño del pincel**: Usa la rueda del ratón.
- **Para limpiar el canvas**: Pulsa **Ctrl+Alt+C**.
- **Para dibujar líneas rectas**: Mantén presionado **Shift** mientras dibujas.
- **Para dibujar cuadrados**: Mantén presionado **Ctrl** mientras dibujas.



### 🎨 Cambiar colores

Presiona las siguientes teclas para cambiar el color del pincel:

- **R**: Rojo
- **G**: Verde
- **B**: Azul
- **Y**: Amarillo
- **C**: Cian
- **M**: Magenta
- **W**: Blanco
- **K**: Negro

**Para cambiar el color de fondo**:
Mantén presionado **Ctrl** y pulsa la tecla del color que desees.

**Para borrar**:
Pulsa **Espacio** para borrar lo dibujado. Si presionas **Ctrl+Espacio**, el fondo se volverá transparente.



### 💻 Cambiar monitores

Puedes cambiar el monitor en el que se muestra el dibujo de varias formas:

- **Tab**: Cambia el monitor en el que está el dibujo, manteniendo el dibujo en el nuevo monitor.
- **Ctrl+Alt+S**: Muestra la aplicación en el monitor actual, pero borrará el dibujo.
- **Ctrl+Alt+R**: Mueve la ventana al monitor correspondiente, manteniendo el dibujo.



### ⚙️ Otros

- Algunas aplicaciones como YouTube o Discord (compartir pantalla) dejarán de actualizarse si están completamente cubiertas por el canvas.
- El canvas tiene un margen de 1 píxel por cada lado, por lo que si la aplicación toca un borde, no debería haber problemas.



## 🤝 Contribuciones

Si deseas contribuir a **Sketch Over**, ¡serás muy bienvenido! Abre un **Pull Request** o reporta un **Issue** si encuentras algún problema. 


## 🔗 Enlaces

- [Jala3400](https://github.com/Jala3400)
- [Repositorio GitHub](https://github.com/Jala3400/SketchOver)


