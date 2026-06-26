# actividad-8

## Ejercicio_1: Escritura con Quórum

### Descripción

Este programa simula un sistema de escritura distribuida con replicación mediante quorum.

Un thread coordinador envía un valor a cinco réplicas, cada una de las cuales puede responder exitosamente luego de una latencia aleatoria o fallar y no responder. El coordinador espera las confirmaciones durante un máximo de un segundo y considera que la escritura fue exitosa cuando recibe al menos tres confirmaciones (quorum). En caso contrario, reporta una falla de escritura.

La simulación se ejecuta tres veces para observar distintos escenarios producidos por las fallas y latencias aleatorias.

### Ejecución

Desde la raíz del repositorio, ejecutar:

```bash
cargo run --manifest-path ejercicio_1/Cargo.toml
