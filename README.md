# Conway's Game of Life

Este es mi segundo laboratorio que implementa el "Game of Life" de Conway.

![Game of Life](game_of_life.gif)

## Cómo ejecutar
Para ejecutar el juego, sigue estos pasos:

1. Clona este repositorio.
2. Asegúrate de tener Rust instalado en tu sistema.
3. Ejecuta el comando cargo run desde el directorio del proyecto.


```
git clone https://github.com/vgcarlol/Graficas
cd Graficas/Lab2
cargo run
```


## Descripción

El "Game of Life" de Conway es un autómata celular desarrollado por el matemático John Horton Conway en 1970. Es un juego sin jugadores en el que la evolución se determina por su estado inicial, sin necesidad de más entradas. Este proyecto visualiza el juego usando la librería `minifb` en Rust.

## Patrones Iniciales

En este proyecto, hemos implementado varios patrones iniciales clásicos para el "Game of Life" de Conway. Aquí hay una descripción de cada uno de ellos:

### Bloque (Block)
Un bloque es un patrón estático compuesto por un cuadrado de 2x2 células vivas. Este patrón no cambia con el tiempo.

```plaintext
OO
OO
```

### Intermitente (Blinker)
Un blinker es un patrón oscilante que alterna entre dos estados. Consiste en una línea vertical de tres células que se convierte en una línea horizontal de tres células.

```
Estado 1:
O
O
O

Estado 2:
OOO
```

### Sapo (Toad)
El toad es otro patrón oscilante compuesto por seis células en dos filas de tres, desplazadas una respecto a la otra.

```
Estado 1:
 OOO
OOO

Estado 2:
O  O
O  O
 OOO
```

### Planeador (Glider)
El planeador es un patrón que se mueve diagonalmente a través del tablero. Es uno de los patrones más conocidos del "Game of Life".

```
O
 O
OOO
```

### Nave Espacial Ligera (Lightweight Spaceship - LWSS)
La nave espacial ligera es un patrón móvil que se desplaza horizontalmente. Es más grande y rápido que el planeador.

```
O    O
O    O
 O  O
  OO
```

### Pulsar
El pulsar es un patrón oscilante más grande que alterna entre dos estados cada tres generaciones. Consiste en un conjunto de bloques dispuestos en un patrón simétrico.

```
  OOO   OOO

O     O     O
O     O     O
O     O     O
  OOO   OOO

  OOO   OOO

O     O     O
O     O     O
O     O     O
  OOO   OOO
```


# Notas
Los patrones iniciales se generan aleatoriamente en diferentes posiciones al iniciar el juego.
Puedes ajustar la resolución del tablero y la velocidad de la simulación modificando los parámetros en el código fuente.
