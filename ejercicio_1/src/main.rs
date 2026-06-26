use rand::Rng;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};
use std::thread;
use std::time::{Duration, Instant};

const CANTIDAD_RONDAS: u32 = 3;
const CANTIDAD_REPLICAS: u32 = 5;
const QUORUM: u32 = 3;
const TIMEOUT_SEGUNDOS: u64 = 1;
const VALOR: i32 = 42;
const LATENCIA_MIN_MS: u64 = 100;
const LATENCIA_MAX_MS: u64 = 800;
const PROBABILIDAD_FALLA: u32 = 30;

fn simular_falla() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=100) < PROBABILIDAD_FALLA
}

fn ejecutar_replica(id: u32, valor: i32, tx: Sender<u32>) {
    let mut rng = rand::thread_rng();
    let latencia = rng.gen_range(LATENCIA_MIN_MS..=LATENCIA_MAX_MS);
    if simular_falla() {
        println!("Réplica {}: falla simulada, no respondo.", id);
        return;
    }
    thread::sleep(Duration::from_millis(latencia));
    println!(
        "Réplica {}: recibí el valor {}, confirmando (latencia: {} ms).",
        id, valor, latencia
    );
    let _ = tx.send(id);
}

fn imprimir_resultado(confirmaciones: u32) {
    if confirmaciones >= QUORUM {
        println!(
            "Coordinador: Quorum alcanzado ({}/{}). Escritura exitosa.",
            confirmaciones, CANTIDAD_REPLICAS
        );
    } else {
        println!(
            "Coordinador: Quorum no alcanzado ({}/{}). Escritura fallida.",
            confirmaciones, CANTIDAD_REPLICAS
        );
    }
}

fn ejecutar_coordinador(rx: Receiver<u32>) {
    let inicio = Instant::now();
    let mut confirmaciones = 0;
    while confirmaciones < QUORUM {
        let transcurrido = inicio.elapsed();
        if transcurrido >= Duration::from_secs(TIMEOUT_SEGUNDOS) {
            break;
        }
        let restante = Duration::from_secs(TIMEOUT_SEGUNDOS) - transcurrido;
        match rx.recv_timeout(restante) {
            Ok(id) => {
                confirmaciones += 1;
                println!("Coordinador: confirmación recibida de Réplica {}.", id);
            }
            Err(RecvTimeoutError::Timeout) => {
                break;
            }
            Err(RecvTimeoutError::Disconnected) => {
                break;
            }
        }
    }
    imprimir_resultado(confirmaciones);
}

fn ejecutar_ronda() {
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
    for id in 1..=CANTIDAD_REPLICAS {
        let tx_replica = tx.clone();
        let handle = thread::spawn(move || {
            ejecutar_replica(id, VALOR, tx_replica);
        });
        handles.push(handle);
    }
    drop(tx);
    ejecutar_coordinador(rx);
    for handle in handles {
        if handle.join().is_err() {
            eprintln!("Error: una réplica finalizó inesperadamente.");
        }
    }
}

fn main() {
    for ronda in 1..=CANTIDAD_RONDAS {
        println!("\n=== Ronda {} ===", ronda);
        ejecutar_ronda();
    }
}
