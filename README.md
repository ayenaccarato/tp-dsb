<h1 align="center">Trabajo Práctico – Desarrollo de Software para Blockchain 2026</h1>
<h2 align="center">EVM y Solana</h2>

Las blockchain compatibles con EVM (Ethereum Virtual Machine) y Solana representan 2 modelos de ejecución y diseños distintos para el desarrollo de contratos inteligentes. 
Por ejemplo, mientras que EVM prioriza simplicidad y estado interno, Solana está diseñada para tener un alto rendimiento, paralelismo y separación entre lógica y estado.

## Modelo de Ejecución
 - EVM
    * Cada contrato es una máquina de estado con almacenamiento propio
    * Las transacciones se ejecutan de forma secuencial
    * El contrato mantiene su propio estado global
    * El paralelismo es muy limitado

- Solana
    * Los programas son stateless
    * El estado se almacena en cuentas externas
    * Las transacciones pueden ejecutarse en paralelo si no comparten cuentas
    * El programa solo define reglas y validaciones

## Almacenamiento de Estado
 - EVM
    * El estado vive dentro del contrato
        mapping (uint256 => Libro) public libros;

    Características:
        - El contrato actúa como base de datos
        - El estado es global y persistente
        - Acceso implícito al storage
 - Solana
    * No existe storage global
    * Cada entidad se almacena en una cuenta independiente
    * El tamaño de las cuentas es fijo
    * El estado debe ser pasado explícitamente al programa

    #[account]
    pub struct Libro {
        pub id: u64,
        pub titulo: String,
    }

## Separación entre lógica y datos
 - EVM
    * Lógica y datos está mezclado
    * El contrato controla y posee su estado
    * Difícil paralelizar
 - Solana
    * Lógica separada del estado
    * Las cuentas tienen dueño
    * El programa valida permisos y modifica datos

## Direccionamiento y Determinismo
 - EVM
    * Las direcciones de contratos son generadas automaticamente
    * El direccionamiento es opaco
    * No hay derivación determinística estándar
 - Solana
    * Uso de Program Derived Address (PDA)
    * Direcciones determinísticas basadas en seeds
    * El programa puede calcular direcciones sin almacenarlas

## Control de acceso
 - EVM
    * Uso de msg.sender
    * Validaciones simples dentro del cnotrato
 - Solana
    * Uso explícito de Signer
    * Validación manual de autoridad
    * Mayor control y seguridad

## Costos y Performance
 - EVM
    * Costos de gas variables
    * Ejecución más lenta
 - Solana
    * Costos muy bajos
    * Alto rendimiento
    * Miles de transacciones por segundo

## Desarrollo y tooling
 - EVM
    * Solidity
 - Solana
    * Rust

En conclusión, patrones comunes en EVM (como storage global) no son válidos en Solana, lo que nos obliga a desarrollar un diseño explícito basado en cuentas y control de acceso estricto.
Ambos modelos, responden a necesidades y objetivos diferentes.


