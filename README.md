# Sistema Solar Procedural en Rust

Este proyecto es una simulación visual de un sistema solar generada completamente de forma procedural usando Rust y la librería minifb para la ventana y gráficos. Incluye:

## Características
- **Cámara 3D**: Movimiento libre con WASD y rotación con el mouse.
- **Estrellas animadas**: Fondo de estrellas que parpadean y se mueven suavemente.
- **Planetas y luna**: Órbitas animadas, planetas rocosos y gaseosos, y una luna.
- **Colisiones**: La cámara no atraviesa los planetas.
- **Shaders de textura**: Degradados, bandas, manchas y mezcla de colores para mejorar el diseño de los planetas.
- **FPS óptimo**: Limitación a 60 FPS para rendimiento estable.

## Estructura
- `src/main.rs`: Lógica principal, renderizado y controles.
- `src/framebuffer.rs`: Framebuffer para dibujar píxeles.
- `src/textura.rs`: Funciones de textura y shaders para los planetas.
- `src/estrellas.rs`: Generación y animación de las estrellas del fondo.


## Controles y uso

Al iniciar el programa, verás el sistema solar en 3D con planetas orbitando y estrellas animadas en el fondo. Puedes interactuar de la siguiente manera:

- **W**: Avanza la cámara hacia adelante.
- **S**: Retrocede la cámara.
- **A**: Mueve la cámara a la izquierda.
- **D**: Mueve la cámara a la derecha.
- **Barra espaciadora**: Sube la cámara (eje Y positivo).
- **Shift izquierdo**: Baja la cámara (eje Y negativo).
- **Mouse**: Mueve el mouse para rotar la vista en cualquier dirección (horizontal y vertical).
- **Escape**: Cierra la ventana y termina la simulación.

La cámara tiene colisión con los planetas, por lo que no puedes atravesarlos. El movimiento es suave y la velocidad puede ajustarse en el código. El fondo de estrellas es animado y los planetas tienen texturas procedurales con degradados y detalles.

## Cómo ejecutar
1. Instala Rust: https://rustup.rs/
2. Clona el repositorio y entra a la carpeta `solar_renderer`.
3. Ejecuta:
   ```
   cargo run
   ```

## Personalización
- Puedes modificar los shaders en `textura.rs` para cambiar el aspecto de los planetas.
- Ajusta el número de estrellas, colores y parámetros para experimentar con el diseño.

