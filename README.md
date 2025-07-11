Proyecto de Rasterización de Polígonos
Este repositorio contiene un algoritmo de rasterización para rellenar polígonos de más de 4 puntos, implementado en Rust utilizando la librería Raylib para la manipulación del framebuffer y la visualización.

Características
Relleno de Polígonos: Implementa un algoritmo de scan-line para rellenar polígonos de cualquier número de vértices.

Dibujo de Contornos: Dibuja el contorno de los polígonos utilizando el algoritmo de línea de Bresenham.

Manejo de Agujeros: Demuestra cómo crear agujeros dentro de polígonos rellenando el polígono interior con el color de fondo.

Polígonos Específicos: Incluye la implementación de los siguientes polígonos con sus colores de relleno y contorno especificados:

Polígono 1: Amarillo con orilla blanca.

Polígono 2: Azul con orilla blanca.

Polígono 3: Rojo con orilla blanca.

Polígono 4: Verde con orilla blanca, con el Polígono 5 como un agujero (rellenado con el color de fondo).

Fondo Personalizable: El fondo de la ventana y del framebuffer es de color rosa.

Estructura del Proyecto
El proyecto está organizado en los siguientes módulos:

main.rs: El punto de entrada principal de la aplicación, donde se definen y dibujan los polígonos.

framebuffer.rs: Contiene la estructura Framebuffer para la gestión de píxeles y la exportación de imágenes.

line.rs: Implementa el algoritmo de Bresenham para dibujar líneas.

polygon.rs: Contiene las funciones fill_polygon (relleno por scan-line) y draw_polygon_outline (dibujo del contorno).

Estrategia de Ramificación (Git)
El repositorio sigue una estrategia de ramificación específica para el laboratorio:

main branch: Contiene únicamente commits de tipo merge de las otras ramas. Es la rama principal que integra todos los polígonos.

Poligon-1 branch: Dibuja únicamente el Polígono 1 (amarillo con orilla blanca).

Poligon-2 branch: Dibuja únicamente el Polígono 2 (azul con orilla blanca).

Poligon-3 branch: Dibuja únicamente el Polígono 3 (rojo con orilla blanca).

Poligon-4 branch: Dibuja el Polígono 4 (verde con orilla blanca) con el Polígono 5 como un agujero.

Flujo de Trabajo Git Recomendado
Inicializar el repositorio:

git init
git add .
git commit -m "Initial commit: Base project structure"

Para cada polígono (ej. Polígono 1):

git branch Poligon-1
git checkout Poligon-1
# Modifica main.rs para dibujar solo el Polígono 1
git add main.rs
git commit -m "Feature: Added Poligon 1 (yellow fill, white outline)"
git checkout main
git merge Poligon-1

Repite este proceso para Poligon-2, Poligon-3 y Poligon-4.

Para el main branch final:
Asegúrate de que el main.rs en la rama main contenga el código para dibujar todos los polígonos. Si no lo has hecho ya, actualiza main.rs con la versión completa y haz un commit:

git add main.rs
git commit -m "Integrate all polygons (1, 2, 3, 4 with 5) into main.rs for final display"

Luego, si no lo hiciste en los pasos anteriores, fusiona cada rama de polígono en main:

git merge Poligon-1
git merge Poligon-2
git merge Poligon-3
git merge Poligon-4

Cómo Compilar y Ejecutar
Para compilar y ejecutar este proyecto, necesitas tener Rust y Raylib instalados en tu sistema.

Clonar el repositorio (si aún no lo has hecho):

git clone <URL_DE_TU_REPOSITORIO>
cd <nombre_de_tu_repositorio>

Compilar y ejecutar:
Asegúrate de estar en el branch main para ver todos los polígonos.

cargo run

Esto compilará el proyecto y abrirá una ventana mostrando los polígonos.

Salida
El programa generará un archivo out.bmp en la raíz del proyecto, que contendrá la imagen rasterizada de todos los polígonos. Este archivo puede ser abierto con cualquier visor de imágenes.

Notas Adicionales
No se incluye el folder build ni ningún código compilado en el repositorio.

La precisión del relleno del polígono es un criterio de calificación importante.
