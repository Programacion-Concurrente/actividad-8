# 🧵 Actividad 8: Concurrencia en Sistemas Distribuidos

Este repositorio contiene la resolución de la **Actividad 8**, enfocada en la simulación y análisis de mecanismos clave en **Sistemas Distribuidos** utilizando Rust: la replicación con protocolo de quórum y el monitoreo de nodos mediante heartbeats (señales de vida).

---

## 📑 Índice
1. [Integrantes](#-integrantes)
2. [Descripción del Proyecto](#-descripción-del-proyecto)
3. [Conceptos Teóricos Analizados](#-conceptos-teóricos-analizados)
    - [Garantías de MPSC vs Redes Reales](#garantías-de-mpsc-vs-redes-reales)
    - [Quórum y Teorema CAP](#quórum-y-teorema-cap)
    - [Falsos Positivos en Detección de Fallas](#falsos-positivos-en-detección-de-fallas)
    - [El Problema de los Dos Generales](#el-problema-de-los-dos-generales)
4. [Estructura del Repositorio](#-estructura-del-repositorio)
5. [Instalación y Ejecución](#-instalación-y-ejecución)

---

## 👥 Integrantes
| Alumno | Padrón |
| :--- | :--- |
| Sebastian Brizuela | 105288 |
| Raquel Ana Dávila | 112002 |
| Lucas Facundo Couttulenc | 109726 |
| Joel Isaac Fernandez Fox | 104424 |
| Luciano Costa | 102104 |

---

## 📋 Descripción del Proyecto
El objetivo de esta actividad es diseñar e implementar simulaciones concurrentes en Rust que modelen las problemáticas y desafíos de coordinación propios de los sistemas distribuidos reales. El trabajo práctico se divide en dos secciones principales:

1. **Ejercicio 1 - Escritura con Quórum:** Simulación de un coordinador central que replica un valor hacia 5 réplicas bajo condiciones de red imperfectas (latencia variable y un 30% de probabilidad de fallo aleatorio). Requiere un quórum de mayoría ($3/5$) dentro de un timeout global de 1 segundo para declarar exitosa la transacción.
2. **Ejercicio 2 - Heartbeats y Detección de Fallas:** Implementación de un monitor centralizado que evalúa la salud de 3 nodos independientes a través de señales de vida periódicas cada 500 ms. Se manejan de manera dinámica los estados de disponibilidad: **ACTIVO**, **SOSPECHOSO** (demoras parciales) y **CAÍDO** (ausencia prolongada).

---

## 💡 Conceptos Teóricos Analizados

### Garantías de MPSC vs Redes Reales
Los canales en memoria compartida (`std::sync::mpsc`) ofrecen certezas ideales que no existen en una red física real. En el informe se destacan tres diferencias críticas:
* **Fiabilidad absoluta:** Los mensajes en `mpsc` siempre llegan a destino. En una red real, los paquetes pueden perderse, duplicarse o corromperse debido a fallas físicas o de enrutamiento.
* **Latencias ínfimas:** La concurrencia local opera en el orden de los nanosegundos. En internet, las distancias físicas y el medio de transporte introducen latencias de milisegundos o incluso segundos.
* **Determinismo en el orden:** Los canales locales garantizan la entrega en orden (FIFO). En una red real, el orden de llegada depende del enrutamiento dinámico de cada paquete, provocando que mensajes enviados después lleguen antes.

### Quórum y Teorema CAP
Al reducir el quórum a un mínimo de 1 réplica para confirmar una escritura:
* **Ganancia (Disponibilidad y Latencia):** El sistema responde con extrema rapidez, ya que no depende de la respuesta de múltiples nodos ni se ve penalizado por latencias acumuladas o caídas parciales.
* **Pérdida (Consistencia):** Se debilita severamente la consistencia. Si el coordinador confirma la escritura con una sola réplica y esta se cae inmediatamente después, el dato se pierde, dando lugar a lecturas inconsistentes (lecturas de datos viejos en otras réplicas).

### Falsos Positivos en Detección de Fallas
Declarar "caído" a un nodo que en realidad está vivo pero experimenta lentitud (como el Nodo 2 del ejercicio) introduce consecuencias severas en entornos reales:
* **Ejemplo concreto:** En un clúster de base de datos distribuidos, si el líder asume incorrectamente que un nodo secundario murió, iniciará una costosa reconfiguración del sistema (trigger de una elección de nuevo líder o migración/redistribución de shards de datos). Esto sobrecarga innecesariamente la red y el procesamiento de los nodos restantes, agravando el problema de lentitud original.

### El Problema de los Dos Generales
Incluso si el coordinador recibe una confirmación, no existe la certeza absoluta en una red real debido a la imprevisibilidad de los tiempos de viaje. Si extendiéramos las latencias más allá del timeout, confirmaciones rezagadas de rondas anteriores podrían ser interpretadas erróneamente como respuestas válidas para la transacción actual, demostrando la imposibilidad de lograr un acuerdo perfecto sobre un canal no confiable.

---

## 📂 Estructura del Repositorio

El repositorio se organiza modularmente dividiendo cada simulación en su propio subdirectorio con entornos de compilación independientes:

* **`ejercicio_1/`**: Simulación del orquestador de escrituras concurrentes con timeouts globales dinámicos calculados mediante `Instant::now()`.
* **`ejercicio_2/`**: Simulación de heartbeats basada en planificación de tiempos absolutos para evitar el desplazamiento progresivo y mitigar retrasos del scheduler del sistema operativo.

Cada subdirectorio cuenta con su propio archivo explicativo `README.md` detallando las decisiones técnicas específicas y ejemplos particulares de salida de consola.

---

## 🚀 Instalación y Ejecución

**Clonar el repositorio:**
```bash
git clone git@github.com:Programacion-Concurrente/actividad-8.git
cd actividad-8
```

**Ejecutar el Ejercicio 1 (Escritura con Quórum):**
```bash
cargo run --manifest-path ejercicio_1/Cargo.toml
```

**Ejecutar el Ejercicio 2 (Heartbeats):**
```bash
cargo run --manifest-path ejercicio_2/Cargo.toml
```
