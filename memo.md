# Mémo — OS Rust / Raspberry Pi 4

Ce mémo résume les termes et commandes rencontrés pendant la création du tout premier kernel bare-metal Rust ciblant uniquement un Raspberry Pi 4.

---

## 1. Cible du projet

### Raspberry Pi 4
- SoC : **BCM2711**
- CPU : **4 × ARM Cortex-A72**
- Architecture utilisée : **AArch64 / ARM64**
- Notre kernel est prévu spécifiquement pour cette machine.

### ARM
Historiquement :
- **ARM = Acorn RISC Machine**
- puis **Advanced RISC Machines**

### RISC
- **Reduced Instruction Set Computer**
- Famille d’architectures privilégiant notamment des instructions relativement simples et régulières.

### AArch64
- Architecture ARM en **64 bits**.
- Notre cible Rust : `aarch64-unknown-none`

### `usize`
Sur notre cible AArch64 :
- `usize` = **64 bits**
- Les pointeurs font également 64 bits.

---

## 2. Bare metal et Rust

### Bare metal
Programme exécuté directement sur le matériel, sans Windows/Linux/autre OS sous lui.

### `#![no_std]`
Notre programme n’utilise pas la bibliothèque standard `std`, car beaucoup de ses fonctionnalités supposent qu’un OS existe.

On conserve notamment :
- `core`
- et plus tard éventuellement `alloc`

### `#![no_main]`
On n’utilise pas le démarrage classique d’un programme Rust avec `main()`.

Notre propre point d’entrée est `_start`.

### `unsafe`
`unsafe` signifie que Rust ne peut pas prouver lui-même que certaines garanties sont respectées.

Exemple :
```rust
write_volatile(0xFE20_1000 as *mut u32, 65);
```

Rust ne peut pas vérifier physiquement que `0xFE201000` correspond réellement au registre UART du Raspberry Pi 4.

L’objectif du kernel :

- garder le plus de code possible en Rust safe ;
- enfermer les opérations matérielles dangereuses dans de petites couches `unsafe`.
