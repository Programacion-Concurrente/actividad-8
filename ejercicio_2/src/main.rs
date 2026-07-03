use rand::Rng;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};
use std::thread;
use std::time::{Duration, Instant};

const CANTIDAD_NODOS: u32 = 3;

const DURACION_SIMULACION_SEGUNDOS: u64 = 8;

const HEARTBEAT_NORMAL_MS: u64 = 500;
const HEARTBEAT_LENTO_1_MS: u64 = 700;
const HEARTBEAT_LENTO_2_MS: u64 = 1200;

const INTERVALO_REVISION_MONITOR_MS: u64 = 100;

const LIMITE_ACTIVO_MS: u64 = 500;
const LIMITE_CAIDO_MS: u64 = 1500;

const MARGEN_TIEMPO_MS: u64 = 20;

#[derive(Clone, Copy, PartialEq)]
enum EstadoNodo {
    Activo,
    Sospechoso,
    Caido,
}

#[derive(Clone, Copy)]
enum ComportamientoNodo {
    Normal,
    Variable,
    Crash,
}

struct InfoNodo {
    estado: EstadoNodo,
    ultimo_heartbeat: Instant,
}

fn ejecutar_nodo(id: u32, comportamiento: ComportamientoNodo, tx: Sender<u32>) {
    let inicio = Instant::now();

    let tiempo_crash = if let ComportamientoNodo::Crash = comportamiento {
        let mut rng = rand::thread_rng();
        Some(rng.gen_range(2..=4))
    } else {
        None
    };

    let mut proximo_heartbeat = Instant::now();

    if tx.send(id).is_err() {
        return;
    }
    println!(
        "[{:.1}s] Nodo {}: heartbeat inicial.",
        inicio.elapsed().as_secs_f32(),
        id
    );

    loop {
        let transcurrido = inicio.elapsed();

        if transcurrido >= Duration::from_secs(DURACION_SIMULACION_SEGUNDOS) {
            break;
        }

        if let Some(segundos) = tiempo_crash {
            if transcurrido >= Duration::from_secs(segundos) {
                println!("Nodo {}: CRASH.", id);
                break;
            }
        }

        let intervalo = match comportamiento {
            ComportamientoNodo::Normal => HEARTBEAT_NORMAL_MS,

            ComportamientoNodo::Variable => {
                let mut rng = rand::thread_rng();

                match rng.gen_range(0..3) {
                    0 => HEARTBEAT_NORMAL_MS,
                    1 => HEARTBEAT_LENTO_1_MS,
                    _ => HEARTBEAT_LENTO_2_MS,
                }
            }

            ComportamientoNodo::Crash => HEARTBEAT_NORMAL_MS,
        };

        // Reemplazo del sleep (para evitar que el nodo se duerma más de lo debido)
        proximo_heartbeat += Duration::from_millis(intervalo);
        let ahora = Instant::now();
        if proximo_heartbeat > ahora {
            thread::sleep(proximo_heartbeat - ahora);
        }

        if inicio.elapsed() >= Duration::from_secs(DURACION_SIMULACION_SEGUNDOS) {
            break;
        }
        if tx.send(id).is_err() {
            break;
        }
        
        println!(
            "[{:.1}s] Nodo {}: heartbeat enviado ({} ms).",
            inicio.elapsed().as_secs_f32(),
            id,
            intervalo
        );
    }

    println!("[{:.1}s] Nodo {}: finalizado.", inicio.elapsed().as_secs_f32(), id);
}

fn imprimir_estado(id: u32, estado: EstadoNodo, tiempo: Duration, inicio: Instant) {
    match estado {
        EstadoNodo::Activo => {}

        EstadoNodo::Sospechoso => {
            println!(
                "[{:.1}s] Monitor: Nodo {} sin heartbeat hace {:.2}s. Estado: SOSPECHOSO.",
                inicio.elapsed().as_secs_f32(),
                id,
                tiempo.as_secs_f32()
            );
        }

        EstadoNodo::Caido => {
            println!(
                "[{:.1}s] Monitor: Nodo {} sin heartbeat hace {:.2}s. Estado: CAIDO.",
                inicio.elapsed().as_secs_f32(),
                id,
                tiempo.as_secs_f32()
            );
        }
    }
}

fn ejecutar_monitor(rx: Receiver<u32>) {
    let inicio = Instant::now();

    let mut nodos = HashMap::new();

    for id in 1..=CANTIDAD_NODOS {
        nodos.insert(
            id,
            InfoNodo {
                estado: EstadoNodo::Activo,
                ultimo_heartbeat: Instant::now(),
            },
        );
    }

    while inicio.elapsed() < Duration::from_secs(DURACION_SIMULACION_SEGUNDOS) {
        match rx.recv_timeout(Duration::from_millis(INTERVALO_REVISION_MONITOR_MS)) {
            Ok(id) => {
                let info = nodos.get_mut(&id).unwrap();

                if info.estado != EstadoNodo::Activo {
                    println!(
                        "[{:.1}s] Monitor: Heartbeat recibido de Nodo {}. Estado: ACTIVO (recuperado).",
                        inicio.elapsed().as_secs_f32(),
                        id
                    );
                } else {
                    println!("[{:.1}s] Monitor: Heartbeat recibido de Nodo {}.", inicio.elapsed().as_secs_f32(), id);
                }

                info.estado = EstadoNodo::Activo;
                info.ultimo_heartbeat = Instant::now();
            }

            Err(RecvTimeoutError::Timeout) => {}

            Err(RecvTimeoutError::Disconnected) => {
                break;
            }
        }

        let ahora = Instant::now();

        for (id, info) in nodos.iter_mut() {
            let tiempo = ahora
                .duration_since(info.ultimo_heartbeat)
                .saturating_sub(Duration::from_millis(MARGEN_TIEMPO_MS));

            let nuevo_estado = if tiempo >= Duration::from_millis(LIMITE_CAIDO_MS) {
                EstadoNodo::Caido
            } else if tiempo > Duration::from_millis(LIMITE_ACTIVO_MS) {
                EstadoNodo::Sospechoso
            } else {
                EstadoNodo::Activo
            };

            if nuevo_estado != info.estado {
                info.estado = nuevo_estado;
                imprimir_estado(*id, nuevo_estado, tiempo, inicio);
            }
        }
    }

    println!("\n========== REPORTE FINAL ==========");

    let ahora = Instant::now();

    for id in 1..=CANTIDAD_NODOS {
        let info = nodos.get(&id).unwrap();

        let tiempo = ahora
            .duration_since(info.ultimo_heartbeat)
            .saturating_sub(Duration::from_millis(MARGEN_TIEMPO_MS));

        let estado = match info.estado {
            EstadoNodo::Activo => "ACTIVO",
            EstadoNodo::Sospechoso => "SOSPECHOSO",
            EstadoNodo::Caido => "CAIDO",
        };

        println!(
            "Nodo {} -> {:11} | Último heartbeat hace {:.2} ms",
            id,
            estado,
            tiempo.as_millis()
        );
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();

    let mut handles = Vec::new();

    let configuracion = [
        (1, ComportamientoNodo::Normal),
        (2, ComportamientoNodo::Variable),
        (3, ComportamientoNodo::Crash),
    ];

    for (id, comportamiento) in configuracion {
        let tx_nodo = tx.clone();

        let handle = thread::spawn(move || {
            ejecutar_nodo(id, comportamiento, tx_nodo);
        });

        handles.push(handle);
    }

    drop(tx);

    ejecutar_monitor(rx);

    for handle in handles {
        if handle.join().is_err() {
            eprintln!("Error: un nodo terminó inesperadamente.");
        }
    }

    println!("\nSimulación finalizada.");
}