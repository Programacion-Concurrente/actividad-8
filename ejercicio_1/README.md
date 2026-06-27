# 🏁 Ejercicio 1: Escritura con Quórum

Este proyecto consiste en una simulación en **Rust** de un sistema de escritura distribuida con replicación y tolerancia a fallas utilizando un protocolo basado en **Quórum** ($3/5$). 

El programa simula un flujo completo donde un nodo coordinador distribuye un valor hacia múltiples réplicas concurrentes. Cada réplica posee condiciones de red y disponibilidad variables (latencia y fallas aleatorias), obligando al coordinador a gestionar tiempos de espera (*timeouts*) dinámicos y a tomar decisiones asíncronas para validar el éxito de la operación.

## 📐 Justificación del Diseño Técnico

Para este escenario particular, se optó por un diseño robusto y eficiente basado en primitivas nativas por las siguientes razones:

* **Paso de mensajes no bloqueante con `mpsc`:** Se utiliza `std::sync::mpsc` (múltiples productores, un único consumidor) para la comunicación desde las réplicas hacia el coordinador. Cada réplica recibe un clon del `Sender` (`tx`), enviando su identificador tan pronto como confirma la escritura. Esto desacopla completamente el procesamiento de los nodos del hilo del coordinador.
* **Uso de un Timeout Global Dinámico con `recv_timeout`:** En lugar de aplicar un timeout fijo por cada confirmación aislada (lo que multiplicaría el tiempo de espera si los mensajes llegan escalonados), se implementa un cronómetro global mediante `Instant::now()`. Antes de cada llamada a `recv_timeout`, el coordinador recalcula el tiempo remanente exacto de la ventana de 1 segundo. Esto garantiza que la operación total jamás supere el límite establecido en el enunciado.
* **Finalización anticipada al alcanzar el Quórum:** El coordinador corta el ciclo de recepción de forma proactiva apenas contabiliza 3 confirmaciones exitosas ($3/5$ constituye la mayoría absoluta). Continuar esperando las respuestas de los nodos lentos incrementaría innecesariamente la latencia de la API distribuida sin alterar el resultado de la transacción.
* **Sincronización mediante `join` post-quórum:** Aunque el coordinador declare el éxito de la escritura de manera temprana, se realiza explitamente un `.join()` sobre los *handles* de todos los hilos. Esto previene condiciones de carrera (*race conditions*), evita que queden hilos zombies ejecutándose entre rondas de prueba y garantiza que las impresiones por consola no se mezclen.

## 📂 Estructura de archivos

- `src/main.rs`: Contiene la lógica del bucle de simulación, las funciones de comportamiento aleatorio de las réplicas, y el cálculo del quórum dinámico en el coordinador.
- `Cargo.toml`: Archivo de configuración y metadatos de las dependencias (ej. `rand`).

## 🚀 Cómo ejecutar

Para compilar y correr la simulación, ejecutá el siguiente comando desde la raíz del repositorio:

```bash
cargo run --manifest-path ejercicio_1/Cargo.toml
```

## 📋 Salida esperada

Dado que el comportamiento de la red es no determinista (las latencias oscilan entre 100ms y 800ms y existe un 30% de probabilidad de fallo por nodo), cada ronda mostrará un flujo diferente. 

Un ejemplo de ejecución real en consola (donde se observa una falla de escritura en la Ronda 1 por falta de nodos, y escrituras exitosas con impresiones asíncronas post-quórum en las Rondas 2 y 3) luce de la siguiente manera:

```text
=== Ronda 1 ===
Réplica 4: falla simulada, no respondo.
Réplica 2: falla simulada, no respondo.
Réplica 3: falla simulada, no respondo.
Réplica 1: falla simulada, no respondo.
Réplica 5: recibí el valor 42, confirmando (latencia: 783 ms).
Coordinador: confirmación recibida de Réplica 5.
Coordinador: Quorum no alcanzado (1/5). Escritura fallida.

=== Ronda 2 ===
Réplica 1: falla simulada, no respondo.
Réplica 4: falla simulada, no respondo.
Réplica 3: recibí el valor 42, confirmando (latencia: 198 ms).
Coordinador: confirmación recibida de Réplica 3.
Réplica 2: recibí el valor 42, confirmando (latencia: 501 ms).
Coordinador: confirmación recibida de Réplica 2.
Réplica 5: recibí el valor 42, confirmando (latencia: 573 ms).
Coordinador: confirmación recibida de Réplica 5.
Coordinador: Quorum alcanzado (3/5). Escritura exitosa.

=== Ronda 3 ===
Réplica 4: falla simulada, no respondo.
Réplica 1: recibí el valor 42, confirmando (latencia: 440 ms).
Coordinador: confirmación recibida de Réplica 1.
Réplica 2: recibí el valor 42, confirmando (latencia: 460 ms).
Coordinador: confirmación recibida de Réplica 2.
Réplica 5: recibí el valor 42, confirmando (latencia: 505 ms).
Coordinador: confirmación recibida de Réplica 5.
Coordinador: Quorum alcanzado (3/5). Escritura exitosa.
Réplica 3: recibí el valor 42, confirmando (latencia: 680 ms).
```

*Nota: Como se evidencia al final de la Ronda 3, debido al mecanismo de **join** ordenado, 
es perfectamente normal que las réplicas que tardaron más tiempo en procesar terminen de imprimir 
su confirmación local después de que el coordinador ya haya validado el quórum y cerrado la transacción.*
