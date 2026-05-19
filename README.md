# LightVM: 8-bit Custom Virtual Machine

A lightweight, 8-bit custom Virtual Machine implemented in Rust. It processes 8-bit instructions to manipulate four internal registers using a custom ISA (Instruction Set Architecture).

## Table of Contents

* [Features](#features)
* [Instruction Set Architecture (ISA)](#instruction-set-architecture-isa)
* [Instruction Encoding](#instruction-encoding)
* [Getting Started](#getting-started)
* [Example Usage](#example-usage)

---

## Features

* **4 Internal Registers:** 8-bit general-purpose registers (`a`, `b`, `c`, `d`).
* **Flexible Addressing:** Supports both register-to-register and immediate-to-register (literal value) operations.
* **Basic Arithmetic & Logic:** Includes `MOV`, `ADD`, `SUB`, `DIV`, `MUL`, and `NAND`.
* **Safe Execution:** Implements wrapping arithmetic to prevent overflow panics, along with a zero-division guard.

---

## Instruction Set Architecture (ISA)

### Registers

| Binary Code | Register Name |
| --- | --- |
| `00` | `a` |
| `01` | `b` |
| `10` | `c` |
| `11` | `d` |

### Opcodes (Operations)

| Opcode (3-bit) | Instruction | Description |
| --- | --- | --- |
| `000` | `NOP` | No operation. |
| `001` | `MOV` | Move source value into the destination register. |
| `010` | `ADD` | Add source value to destination (wrapping). |
| `011` | `SUB` | Subtract source value from destination (wrapping). |
| `100` | `DIV` | Divide destination by source value (guarded against division by zero). |
| `101` | `MUL` | Multiply destination by source value (wrapping). |
| `110` | `NAND` | Perform bitwise NAND operation on destination and source. |
| `111` | `EXIT` | Placeholder for exit instruction. |

---

## Instruction Encoding

Each instruction is exactly 1 byte (8-bit unsigned integer) structured as follows:

```
┌───────────────┬──────────────┬───────────────┬───────────────┐
│  Opcode (3b)  │   Dst (2b)   │  Prefix (1b)  │   Src (2b)    │
└───────────────┴──────────────┴───────────────┴───────────────┘
  7           5   4          3         2         1           0  (Bit position)

```

* **Opcode (Bits 7–5):** Determines the operation to execute.
* **Dst (Bits 4–3):** The destination register (`a`, `b`, `c`, or `d`).
* **Prefix (Bit 2):** Determines how the `Src` bits are interpreted:
* `0`: **Register Mode** (Src represents a register code `00` to `11`).
* `1`: **Immediate Mode** (Src represents a literal value `0` to `3`).


* **Src (Bits 1–0):** The source register index or a 2-bit immediate value.

### Encoding Examples

* **`MOV a, 1`** (Move immediate value `1` into register `a`):
* Opcode (`MOV`): `001`
* Dst (`a`): `00`
* Prefix (`imm`): `1`
* Src (`1`): `01`
* **Binary:** `00100101` $\rightarrow$ **Decimal:** `37`


* **`ADD b, c`** (Add the value of register `c` to register `b`):
* Opcode (`ADD`): `010`
* Dst (`b`): `01`
* Prefix (`reg`): `0`
* Src (`c`): `10`
* **Binary:** `01001010` $\rightarrow$ **Decimal:** `74`



---

## Getting Started

### Prerequisites

Make sure you have [Rust and Cargo](https://www.google.com/search?q=https://www.rust-lang.org/tools/install) installed on your system.

### Running the Project

1. Clone or navigate to the project directory.
2. Run the application using Cargo:
```bash
cargo run

```


3. Type a decimal number (`0`–`255`) corresponding to your encoded instruction and press **Enter**.
4. To safely exit the loop and stop the VM, enter `255`.

---

## Example Usage

An interactive session putting values into registers and performing operations:

```text
==========================vm==========================
37
register:
a: 1, b: 0, c: 0, d: 0
38
register:
a: 2, b: 0, c: 0, d: 0
74
register:
a: 2, b: 0, c: 0, d: 0
255
register:
a: 2, b: 0, c: 0, d: 0

```