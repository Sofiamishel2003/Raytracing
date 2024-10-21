# Proyecto de Raytracing en Rust

Este proyecto es una implementación de un diorama en 3D con efectos avanzados de **raytracing** desarrollado en **Rust**. Utiliza varias características para crear una escena compleja con materiales, luces, una cámara ajustable, normal mapping, y más. El objetivo principal es simular efectos realistas de iluminación, reflejos, sombras, y texturización.

## puntos acumulados

- [20 puntos] Criterio subjetivo. Por qué tan compleja sea su escena
- [10 puntos] Criterio subjetivo. Por qué tan visualmente atractiva sea su escena
- [25 puntos] Criterio subjetivo. Según el performance de su raytracer.
- [25 puntos] 5 texturas con su su propia textura, y sus propios parametros para albedo, specular, transparencia y reflectivida
- [15 puntos] Por implementar un skybox.
- [5 puntos] Modificar la cámara para que se pueda acercar y alejar en la dirección de su center.
- [5 puntos] Agregar fresnel al calculo de transparencia y reflectividad.
- [10 puntos] Agregar soporte para múltiples fuentes de luz con diferentes colores e intensidades
- [20 puntos] Agregar soporte para mapeo de normales (normal map) para aumentar el detalle aparente de las superficies sin aumentar la complejidad geométrica.
- [10 puntos] Implementar un ciclo de día y noche con condiciones de iluminación cambiantes. Puede ser que la luz cambie de posición y color según el tiempo o al presionar algunos botones
- [15 puntos] Agregar soporte para materiales emisivos que actúen como fuentes de luz por sí mismos.
### Total 105 + 55 de criterio subjetivo
## Características principales

- **Ciclo de día y noche**: La escena cuenta con un sistema dinámico de iluminación que cambia según el tiempo del día.
- **Normal mapping**: Se utiliza para agregar detalles a la superficie de los objetos.
- **Materiales emisivos**: Objetos que generan su propia luz.
- **Texturas animadas**: Soporte para texturas con animaciones.
- **Fresnel**: Implementación del efecto Fresnel para reflejos más realistas.
- **Skybox**: Fondo dinámico que cambia con el ciclo día/noche.
- **Cámara orbital**: La cámara puede moverse alrededor de la escena utilizando controles para `yaw` y `pitch`.
- **Fuentes de luz múltiples**: Soporte para diferentes tipos de luces en la escena.
- **Optimización de rendimiento**: Enfocado en correr de manera eficiente en tiempo real.

## Archivos del Proyecto

- `main.rs`: Archivo principal donde se inicializa la escena y se gestionan los componentes principales del sistema.
- `camera.rs`: Implementa la cámara orbital, permitiendo controlar la vista y el movimiento dentro de la escena.
- `framebuffer.rs`: Gestiona el buffer donde se almacenan los píxeles que luego serán renderizados en pantalla.
- `light.rs`: Contiene la lógica para las diferentes fuentes de luz en la escena.
- `material.rs`: Define los materiales, texturas y propiedades de reflexión/refracción.
- `ray_intersect.rs`: Se encarga de las intersecciones de rayos con los objetos en la escena.
- `texture.rs`: Gestiona las texturas aplicadas a los objetos, incluyendo el normal mapping.
- `color.rs`: Define los colores utilizados para la iluminación y los objetos.
- `cube.rs`: Implementación de los objetos cúbicos utilizados en el diorama.

## Requisitos

Para ejecutar este proyecto, necesitarás tener instaladas las siguientes dependencias:

- **Rust** (Versión más reciente)
- **Librerías**:
  - `nalgebra_glm` para operaciones matemáticas en 3D.
  - `minifb` para crear ventanas y renderizar el framebuffer.
  - `once_cell` para la inicialización de variables estáticas.

## Cómo ejecutar el proyecto

1. Clona el repositorio:

   ```bash
   git clone https://github.com/usuario/proyecto-raytracing.git
   ```

2. Navega al directorio del proyecto:

   ```bash
   cd proyecto-raytracing
   ```

3. Compila y ejecuta el proyecto:

   ```bash
   cargo run
   ```

## Funcionalidades adicionales

- **Efectos de iluminación**: Utiliza la ley del coseno de Lambert para calcular la iluminación difusa.
- **Sombras y reflexiones**: Calculadas utilizando la ley de Snell para manejar refracciones y reflexiones.
- **Mapeo UV**: Para la correcta aplicación de texturas sobre las superficies de los objetos.

## Video del proyecto

https://github.com/user-attachments/assets/8406bd97-0c49-4058-89e5-7fa3f4a04b8c



