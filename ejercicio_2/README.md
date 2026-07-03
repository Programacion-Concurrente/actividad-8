# 🫀 Ejercicio 2: Heartbeats y Detección de Fallas

Este proyecto consiste en una simulación en **Rust** de un sistema de monitoreo distribuido mediante **heartbeats**. Un conjunto de nodos envía periódicamente señales de vida a un monitor, el cual determina el estado de cada nodo según el tiempo transcurrido desde el último heartbeat recibido.

La simulación contempla tres comportamientos distintos: un nodo que funciona normalmente, un nodo que presenta demoras variables simulando congestión de red y un nodo que deja de enviar heartbeats luego de algunos segundos, simulando una caída.

## 📐 Justificación del Diseño Técnico

Para este escenario se optó por un diseño basado en las primitivas de concurrencia de la biblioteca estándar de Rust.

* **Comunicación mediante `mpsc`:** Cada nodo posee un clon del `Sender` y envía únicamente su identificador al monitor cada vez que genera un heartbeat. El monitor recibe todos los mensajes desde un único `Receiver`, desacoplando completamente la ejecución de los nodos.

* **Monitoreo con `recv_timeout`:** El monitor utiliza `recv_timeout` para alternar entre la recepción de heartbeats y la verificación periódica del estado de los nodos. De esta manera nunca queda bloqueado esperando mensajes y puede detectar oportunamente la ausencia de heartbeats.

* **Registro del último heartbeat:** Para cada nodo se almacena el instante (`Instant`) en el que se recibió su último heartbeat junto con su estado actual. En cada iteración del monitor se calcula el tiempo transcurrido desde ese instante para determinar si el nodo continúa activo, pasa a sospechoso o debe declararse caído.

* **Planificación mediante instantes absolutos:** Los nodos no generan sus heartbeats realizando simplemente un `sleep(intervalo)` en cada iteración. En cambio, mantienen el instante exacto en que debe enviarse el próximo heartbeat y esperan únicamente el tiempo restante hasta dicho instante. Esta decisión evita que pequeños retrasos de planificación del sistema operativo se acumulen entre iteraciones y desplacen progresivamente los envíos.

* **Finalización ordenada mediante `join`:** Una vez transcurridos los ocho segundos de simulación, el monitor finaliza y luego se realiza `join` sobre todos los hilos de los nodos para asegurar una terminación limpia del programa y evitar que queden threads ejecutándose.

## 📝 Decisiones de diseño

La consigna presentaba cierta ambigüedad (para nosotros) respecto de cómo interpretar los estados **ACTIVO**, **SOSPECHOSO** y **CAÍDO**, debido a ciertas inconsistencias en el output de ejemplo. En esta implementación se adoptó el siguiente criterio:

* Se espera que cada nodo envíe un heartbeat aproximadamente cada **500 ms**.
* Si transcurren **más de 500 ms** sin recibir un nuevo heartbeat, el nodo pasa al estado **SOSPECHOSO**.
* Si el tiempo sin heartbeats alcanza **1.5 segundos**, el nodo pasa al estado **CAÍDO**.
* Si un nodo marcado como **SOSPECHOSO** vuelve a enviar un heartbeat antes de alcanzar el límite de 1.5 segundos, recupera inmediatamente el estado **ACTIVO**.
* **Modelado del comportamiento de los nodos:** Para representar distintos escenarios de funcionamiento se definieron tres comportamientos fijos. El nodo 1 envía heartbeats cada 500 ms durante toda la simulación, representando un nodo estable. El nodo 2 selecciona aleatoriamente el intervalo del próximo heartbeat entre 500 ms, 700 ms y 1200 ms con igual probabilidad, simulando un nodo afectado por demoras de red. De esta manera, aproximadamente en dos de cada tres envíos supera el intervalo esperado de 500 ms y es marcado temporalmente como **SOSPECHOSO**, recuperando luego el estado **ACTIVO** al llegar el siguiente heartbeat. Finalmente, el nodo 3 envía heartbeats normalmente hasta un instante aleatorio entre los 2 y 4 segundos, momento en el que deja de responder definitivamente para simular una caída.

Este criterio permite diferenciar un nodo lento de uno realmente caído, manteniendo el comportamiento esperado del nodo con latencias variables.

Además, debido a que en una simulación siempre existen pequeños retrasos producidos por la planificación de threads del sistema operativo, se considera un pequeño margen temporal al realizar las comparaciones de tiempo. Esto evita falsos positivos cuando un heartbeat llega apenas algunos milisegundos después del instante esperado.

## 📂 Estructura de archivos

- `src/main.rs`: Contiene la implementación de los nodos, el monitor, la lógica de detección de fallas y el reporte final.
- `Cargo.toml`: Archivo de configuración y dependencias del proyecto (ej. `rand`).

## 🚀 Cómo ejecutar

Para compilar y ejecutar la simulación: <br> 
Desde `/ejercicio_2/`
```bash
cargo run
```

## 📋 Salida esperada

Debido a que el instante de caída del nodo 3 y los intervalos del nodo 2 son aleatorios, cada ejecución puede producir una secuencia diferente de eventos.

Algunos ejemplos de salida son los siguientes:

```text
[0.0s] Nodo 1: heartbeat inicial.
[0.0s] Monitor: Heartbeat recibido de Nodo 1.
[0.0s] Nodo 3: heartbeat inicial.
[0.0s] Nodo 2: heartbeat inicial.
[0.0s] Monitor: Heartbeat recibido de Nodo 3.
[0.0s] Monitor: Heartbeat recibido de Nodo 2.
[0.5s] Nodo 1: heartbeat enviado (500 ms).
[0.5s] Monitor: Heartbeat recibido de Nodo 1.
[0.5s] Monitor: Heartbeat recibido de Nodo 2.
[0.5s] Monitor: Heartbeat recibido de Nodo 3.
[0.5s] Nodo 2: heartbeat enviado (500 ms).
[0.5s] Nodo 3: heartbeat enviado (500 ms).
[1.0s] Nodo 3: heartbeat enviado (500 ms).
[1.0s] Nodo 1: heartbeat enviado (500 ms).
[1.0s] Monitor: Heartbeat recibido de Nodo 3.
[1.0s] Monitor: Heartbeat recibido de Nodo 1.
[1.1s] Monitor: Nodo 2 sin heartbeat hace 0.60s. Estado: SOSPECHOSO.
[1.2s] Nodo 2: heartbeat enviado (700 ms).
[1.2s] Monitor: Heartbeat recibido de Nodo 2. Estado: ACTIVO (recuperado).
[1.5s] Nodo 3: heartbeat enviado (500 ms).
[1.5s] Nodo 1: heartbeat enviado (500 ms).
[1.5s] Monitor: Heartbeat recibido de Nodo 3.
[1.5s] Monitor: Heartbeat recibido de Nodo 1.
[1.7s] Nodo 2: heartbeat enviado (500 ms).
[1.7s] Monitor: Heartbeat recibido de Nodo 2.
[2.0s] Nodo 3: heartbeat enviado (500 ms).
[2.0s] Nodo 1: heartbeat enviado (500 ms).
[2.0s] Monitor: Heartbeat recibido de Nodo 3.
[2.0s] Monitor: Heartbeat recibido de Nodo 1.
[2.3s] Monitor: Nodo 2 sin heartbeat hace 0.59s. Estado: SOSPECHOSO.
[2.5s] Nodo 3: heartbeat enviado (500 ms).
[2.5s] Monitor: Heartbeat recibido de Nodo 3.
[2.5s] Monitor: Heartbeat recibido de Nodo 1.
[2.5s] Nodo 1: heartbeat enviado (500 ms).
[2.9s] Nodo 2: heartbeat enviado (1200 ms).
[2.9s] Monitor: Heartbeat recibido de Nodo 2. Estado: ACTIVO (recuperado).
[3.0s] Nodo 3: heartbeat enviado (500 ms).
Nodo 3: CRASH.
[3.0s] Nodo 3: finalizado.
[3.0s] Nodo 1: heartbeat enviado (500 ms).
[3.0s] Monitor: Heartbeat recibido de Nodo 3.
[3.0s] Monitor: Heartbeat recibido de Nodo 1.
[3.4s] Nodo 2: heartbeat enviado (500 ms).
[3.4s] Monitor: Heartbeat recibido de Nodo 2.
[3.5s] Nodo 1: heartbeat enviado (500 ms).
[3.5s] Monitor: Heartbeat recibido de Nodo 1.
[3.6s] Monitor: Nodo 3 sin heartbeat hace 0.59s. Estado: SOSPECHOSO.
[3.9s] Nodo 2: heartbeat enviado (500 ms).
[3.9s] Monitor: Heartbeat recibido de Nodo 2.
[4.0s] Nodo 1: heartbeat enviado (500 ms).
[4.0s] Monitor: Heartbeat recibido de Nodo 1.
[4.4s] Monitor: Nodo 2 sin heartbeat hace 0.52s. Estado: SOSPECHOSO.
[4.5s] Nodo 1: heartbeat enviado (500 ms).
[4.5s] Monitor: Heartbeat recibido de Nodo 1.
[4.6s] Monitor: Nodo 3 sin heartbeat hace 1.58s. Estado: CAIDO.
[5.0s] Nodo 1: heartbeat enviado (500 ms).
[5.0s] Monitor: Heartbeat recibido de Nodo 1.
[5.1s] Nodo 2: heartbeat enviado (1200 ms).
[5.1s] Monitor: Heartbeat recibido de Nodo 2. Estado: ACTIVO (recuperado).
[5.5s] Nodo 1: heartbeat enviado (500 ms).
[5.5s] Monitor: Heartbeat recibido de Nodo 1.
[5.7s] Monitor: Nodo 2 sin heartbeat hace 0.60s. Estado: SOSPECHOSO.
[6.0s] Nodo 1: heartbeat enviado (500 ms).
[6.0s] Monitor: Heartbeat recibido de Nodo 1.
[6.3s] Nodo 2: heartbeat enviado (1200 ms).
[6.3s] Monitor: Heartbeat recibido de Nodo 2. Estado: ACTIVO (recuperado).
[6.5s] Nodo 1: heartbeat enviado (500 ms).
[6.5s] Monitor: Heartbeat recibido de Nodo 1.
[6.8s] Monitor: Nodo 2 sin heartbeat hace 0.50s. Estado: SOSPECHOSO.
[7.0s] Nodo 1: heartbeat enviado (500 ms).
[7.0s] Monitor: Heartbeat recibido de Nodo 1.
[7.5s] Nodo 2: heartbeat enviado (1200 ms).
[7.5s] Nodo 1: heartbeat enviado (500 ms).
[7.5s] Monitor: Heartbeat recibido de Nodo 2. Estado: ACTIVO (recuperado).
[7.5s] Monitor: Heartbeat recibido de Nodo 1.
[8.0s] Nodo 1: finalizado.
[8.0s] Monitor: Nodo 1 sin heartbeat hace 0.52s. Estado: SOSPECHOSO.
[8.0s] Monitor: Nodo 2 sin heartbeat hace 0.52s. Estado: SOSPECHOSO.

========== REPORTE FINAL ==========
Nodo 1 -> SOSPECHOSO  | Último heartbeat hace 516 ms
Nodo 2 -> SOSPECHOSO  | Último heartbeat hace 517 ms
Nodo 3 -> CAIDO       | Último heartbeat hace 5016 ms
[8.7s] Nodo 2: finalizado.

Simulación finalizada.
```

```
[0.0s] Nodo 2: heartbeat inicial.
[0.0s] Monitor: Heartbeat recibido de Nodo 2.
[0.0s] Nodo 1: heartbeat inicial.
[0.0s] Nodo 3: heartbeat inicial.
[0.0s] Monitor: Heartbeat recibido de Nodo 1.
[0.0s] Monitor: Heartbeat recibido de Nodo 3.
[0.5s] Nodo 1: heartbeat enviado (500 ms).
[0.5s] Monitor: Heartbeat recibido de Nodo 1.
[0.5s] Nodo 3: heartbeat enviado (500 ms).
[0.5s] Monitor: Heartbeat recibido de Nodo 3.
[0.6s] Monitor: Nodo 2 sin heartbeat hace 0.58s. Estado: SOSPECHOSO.
[1.0s] Nodo 3: heartbeat enviado (500 ms).
[1.0s] Nodo 1: heartbeat enviado (500 ms).
[1.0s] Monitor: Heartbeat recibido de Nodo 3.
[1.0s] Monitor: Heartbeat recibido de Nodo 1.
[1.2s] Nodo 2: heartbeat enviado (1200 ms).
[1.2s] Monitor: Heartbeat recibido de Nodo 2. Estado: ACTIVO (recuperado).
[1.5s] Nodo 1: heartbeat enviado (500 ms).
[1.5s] Nodo 3: heartbeat enviado (500 ms).
[1.5s] Monitor: Heartbeat recibido de Nodo 1.
[1.5s] Monitor: Heartbeat recibido de Nodo 3.
[1.7s] Nodo 2: heartbeat enviado (500 ms).
[1.7s] Monitor: Heartbeat recibido de Nodo 2.
[2.0s] Nodo 3: heartbeat enviado (500 ms).
[2.0s] Nodo 1: heartbeat enviado (500 ms).
[2.0s] Monitor: Heartbeat recibido de Nodo 3.
[2.0s] Monitor: Heartbeat recibido de Nodo 1.
[2.3s] Monitor: Nodo 2 sin heartbeat hace 0.58s. Estado: SOSPECHOSO.
[2.4s] Nodo 2: heartbeat enviado (700 ms).
[2.4s] Monitor: Heartbeat recibido de Nodo 2. Estado: ACTIVO (recuperado).
[2.5s] Nodo 3: heartbeat enviado (500 ms).
[2.5s] Nodo 1: heartbeat enviado (500 ms).
[2.5s] Monitor: Heartbeat recibido de Nodo 3.
[2.5s] Monitor: Heartbeat recibido de Nodo 1.
[2.9s] Monitor: Nodo 2 sin heartbeat hace 0.51s. Estado: SOSPECHOSO.
[3.0s] Nodo 3: heartbeat enviado (500 ms).
[3.0s] Nodo 1: heartbeat enviado (500 ms).
[3.0s] Monitor: Heartbeat recibido de Nodo 3.
[3.0s] Monitor: Heartbeat recibido de Nodo 1.
[3.1s] Nodo 2: heartbeat enviado (700 ms).
[3.1s] Monitor: Heartbeat recibido de Nodo 2. Estado: ACTIVO (recuperado).
[3.5s] Nodo 3: heartbeat enviado (500 ms).
[3.5s] Nodo 1: heartbeat enviado (500 ms).
[3.5s] Monitor: Heartbeat recibido de Nodo 3.
[3.5s] Monitor: Heartbeat recibido de Nodo 1.
[3.7s] Monitor: Nodo 2 sin heartbeat hace 0.61s. Estado: SOSPECHOSO.
[3.8s] Nodo 2: heartbeat enviado (700 ms).
[3.8s] Monitor: Heartbeat recibido de Nodo 2. Estado: ACTIVO (recuperado).
[4.0s] Nodo 3: heartbeat enviado (500 ms).
Nodo 3: CRASH.
[4.0s] Nodo 3: finalizado.
[4.0s] Nodo 1: heartbeat enviado (500 ms).
[4.0s] Monitor: Heartbeat recibido de Nodo 3.
[4.0s] Monitor: Heartbeat recibido de Nodo 1.
[4.4s] Monitor: Nodo 2 sin heartbeat hace 0.61s. Estado: SOSPECHOSO.
[4.5s] Nodo 2: heartbeat enviado (700 ms).
[4.5s] Nodo 1: heartbeat enviado (500 ms).
[4.5s] Monitor: Heartbeat recibido de Nodo 2. Estado: ACTIVO (recuperado).
[4.5s] Monitor: Heartbeat recibido de Nodo 1.
[4.6s] Monitor: Nodo 3 sin heartbeat hace 0.58s. Estado: SOSPECHOSO.
[5.0s] Nodo 1: heartbeat enviado (500 ms).
[5.0s] Monitor: Heartbeat recibido de Nodo 1.
[5.1s] Monitor: Nodo 2 sin heartbeat hace 0.59s. Estado: SOSPECHOSO.
[5.5s] Nodo 1: heartbeat enviado (500 ms).
[5.5s] Monitor: Heartbeat recibido de Nodo 1.
[5.6s] Monitor: Nodo 3 sin heartbeat hace 1.58s. Estado: CAIDO.
[5.7s] Nodo 2: heartbeat enviado (1200 ms).
[5.7s] Monitor: Heartbeat recibido de Nodo 2. Estado: ACTIVO (recuperado).
[6.0s] Nodo 1: heartbeat enviado (500 ms).
[6.0s] Monitor: Heartbeat recibido de Nodo 1.
[6.2s] Nodo 2: heartbeat enviado (500 ms).
[6.2s] Monitor: Heartbeat recibido de Nodo 2.
[6.5s] Nodo 1: heartbeat enviado (500 ms).
[6.5s] Monitor: Heartbeat recibido de Nodo 1.
[6.7s] Monitor: Nodo 2 sin heartbeat hace 0.50s. Estado: SOSPECHOSO.
[6.9s] Nodo 2: heartbeat enviado (700 ms).
[6.9s] Monitor: Heartbeat recibido de Nodo 2. Estado: ACTIVO (recuperado).
[7.0s] Nodo 1: heartbeat enviado (500 ms).
[7.0s] Monitor: Heartbeat recibido de Nodo 1.
[7.4s] Monitor: Nodo 2 sin heartbeat hace 0.52s. Estado: SOSPECHOSO.
[7.5s] Nodo 1: heartbeat enviado (500 ms).
[7.5s] Monitor: Heartbeat recibido de Nodo 1.
[8.0s] Nodo 1: finalizado.
[8.0s] Monitor: Nodo 1 sin heartbeat hace 0.52s. Estado: SOSPECHOSO.

========== REPORTE FINAL ==========
Nodo 1 -> SOSPECHOSO  | Último heartbeat hace 523 ms
Nodo 2 -> SOSPECHOSO  | Último heartbeat hace 1124 ms
Nodo 3 -> CAIDO       | Último heartbeat hace 4022 ms
[8.1s] Nodo 2: finalizado.

Simulación finalizada.
```

```
[0.0s] Nodo 1: heartbeat inicial.
[0.0s] Monitor: Heartbeat recibido de Nodo 1.
[0.0s] Nodo 2: heartbeat inicial.
[0.0s] Monitor: Heartbeat recibido de Nodo 2.
[0.0s] Nodo 3: heartbeat inicial.
[0.0s] Monitor: Heartbeat recibido de Nodo 3.
[0.5s] Nodo 1: heartbeat enviado (500 ms).
[0.5s] Monitor: Heartbeat recibido de Nodo 1.
[0.5s] Nodo 3: heartbeat enviado (500 ms).
[0.5s] Monitor: Heartbeat recibido de Nodo 3.
[0.6s] Monitor: Nodo 2 sin heartbeat hace 0.58s. Estado: SOSPECHOSO.
[0.7s] Nodo 2: heartbeat enviado (700 ms).
[0.7s] Monitor: Heartbeat recibido de Nodo 2. Estado: ACTIVO (recuperado).
[1.0s] Nodo 3: heartbeat enviado (500 ms).
[1.0s] Nodo 1: heartbeat enviado (500 ms).
[1.0s] Monitor: Heartbeat recibido de Nodo 3.
[1.0s] Monitor: Heartbeat recibido de Nodo 1.
[1.2s] Monitor: Nodo 2 sin heartbeat hace 0.52s. Estado: SOSPECHOSO.
[1.5s] Nodo 1: heartbeat enviado (500 ms).
[1.5s] Monitor: Heartbeat recibido de Nodo 1.
[1.5s] Nodo 3: heartbeat enviado (500 ms).
[1.5s] Monitor: Heartbeat recibido de Nodo 3.
[1.9s] Nodo 2: heartbeat enviado (1200 ms).
[1.9s] Monitor: Heartbeat recibido de Nodo 2. Estado: ACTIVO (recuperado).
[2.0s] Nodo 1: heartbeat enviado (500 ms).
[2.0s] Monitor: Heartbeat recibido de Nodo 1.
[2.0s] Monitor: Heartbeat recibido de Nodo 3.
[2.0s] Nodo 3: heartbeat enviado (500 ms).
Nodo 3: CRASH.
[2.0s] Nodo 3: finalizado.
[2.4s] Nodo 2: heartbeat enviado (500 ms).
[2.4s] Monitor: Heartbeat recibido de Nodo 2.
[2.5s] Nodo 1: heartbeat enviado (500 ms).
[2.5s] Monitor: Heartbeat recibido de Nodo 1.
[2.6s] Monitor: Nodo 3 sin heartbeat hace 0.58s. Estado: SOSPECHOSO.
[2.9s] Monitor: Nodo 2 sin heartbeat hace 0.50s. Estado: SOSPECHOSO.
[3.0s] Nodo 1: heartbeat enviado (500 ms).
[3.0s] Monitor: Heartbeat recibido de Nodo 1.
[3.5s] Nodo 1: heartbeat enviado (500 ms).
[3.5s] Monitor: Heartbeat recibido de Nodo 1.
[3.6s] Nodo 2: heartbeat enviado (1200 ms).
[3.6s] Monitor: Heartbeat recibido de Nodo 2. Estado: ACTIVO (recuperado).
[3.6s] Monitor: Nodo 3 sin heartbeat hace 1.58s. Estado: CAIDO.
[4.0s] Nodo 1: heartbeat enviado (500 ms).
[4.0s] Monitor: Heartbeat recibido de Nodo 1.
[4.1s] Nodo 2: heartbeat enviado (500 ms).
[4.1s] Monitor: Heartbeat recibido de Nodo 2.
[4.5s] Nodo 1: heartbeat enviado (500 ms).
[4.5s] Monitor: Heartbeat recibido de Nodo 1.
[4.7s] Monitor: Nodo 2 sin heartbeat hace 0.60s. Estado: SOSPECHOSO.
[5.0s] Nodo 1: heartbeat enviado (500 ms).
[5.0s] Monitor: Heartbeat recibido de Nodo 1.
[5.3s] Nodo 2: heartbeat enviado (1200 ms).
[5.3s] Monitor: Heartbeat recibido de Nodo 2. Estado: ACTIVO (recuperado).
[5.5s] Nodo 1: heartbeat enviado (500 ms).
[5.5s] Monitor: Heartbeat recibido de Nodo 1.
[5.9s] Monitor: Nodo 2 sin heartbeat hace 0.60s. Estado: SOSPECHOSO.
[6.0s] Nodo 1: heartbeat enviado (500 ms).
[6.0s] Monitor: Heartbeat recibido de Nodo 1.
[6.5s] Nodo 2: heartbeat enviado (1200 ms).
[6.5s] Nodo 1: heartbeat enviado (500 ms).
[6.5s] Monitor: Heartbeat recibido de Nodo 2. Estado: ACTIVO (recuperado).
[6.5s] Monitor: Heartbeat recibido de Nodo 1.
[7.0s] Nodo 1: heartbeat enviado (500 ms).
[7.0s] Monitor: Heartbeat recibido de Nodo 1.
[7.1s] Monitor: Nodo 2 sin heartbeat hace 0.59s. Estado: SOSPECHOSO.
[7.5s] Nodo 1: heartbeat enviado (500 ms).
[7.5s] Monitor: Heartbeat recibido de Nodo 1.
[7.7s] Nodo 2: heartbeat enviado (1200 ms).
[7.7s] Monitor: Heartbeat recibido de Nodo 2. Estado: ACTIVO (recuperado).
[8.0s] Nodo 1: finalizado.

========== REPORTE FINAL ==========
Nodo 1 -> ACTIVO      | Último heartbeat hace 496 ms
Nodo 2 -> ACTIVO      | Último heartbeat hace 296 ms
Nodo 3 -> CAIDO       | Último heartbeat hace 5996 ms
[8.4s] Nodo 2: finalizado.

Simulación finalizada.
```