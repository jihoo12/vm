# LightVM: 8-bit Custom Virtual Machine

A lightweight, 8-bit custom Virtual Machine implemented in Rust. It processes 8-bit instructions to manipulate four internal registers or manage an internal 16-bit output buffer using a custom ISA (Instruction Set Architecture).

## Table of Contents

* [Features](#features)
* [Instruction Set Architecture (ISA)](#instruction-set-architecture-isa)
* [Instruction Encoding](#instruction-encoding)
* [Extended Instruction Set (Stdout Buffer)](#extended-instruction-set-stdout-buffer)
* [Getting Started](#getting-started)
* [Example Usage](#example-usage)

---

## Features

* **4 Internal Registers:** 8-bit general-purpose registers (`a`, `b`, `c`, `d`).
* **Flexible Addressing:** Supports both register-to-register and immediate-to-register (literal value) operations.
* **Basic Arithmetic & Logic:** Includes `MOV`, `ADD`, `SUB`, `DIV`, `MUL`, and `NAND`.
* **Safe Execution:** Implements wrapping arithmetic to prevent overflow panics, along with a zero-division guard.
* **Extended Features:** Includes a 16-bit internal buffer (`buf`) with custom stack-like operations (`PUSH`, `POP`, `FLUSH`, `PRINT`) for output control.

---

## Instruction Set Architecture (ISA)

### Registers

| Binary Code | Register Name |
| --- | --- |
| `00` | `a` |
| `01` | `b` |
| `10` | `c` |
| `11` | `d` |

### Standard Opcodes (Operations)

| Opcode (3-bit) | Instruction | Description |
| --- | --- | --- |
| `000` | `NOP` / `EXT` | No operation, or triggers the **Extended Instruction Set** if the extension prefix is set. |
| `001` | `MOV` | Move source value into the destination register. |
| `010` | `ADD` | Add source value to destination (wrapping). |
| `011` | `SUB` | Subtract source value from destination (wrapping). |
| `100` | `DIV` | Divide destination by source value (guarded against division by zero). |
| `101` | `MUL` | Multiply destination by source value (wrapping). |
| `110` | `NAND` | Perform bitwise NAND operation on destination and source. |
| `111` | `EXIT` | Reached through a special manual exit shortcut (`255`). |

---

## Instruction Encoding

### Standard Instructions

Each standard instruction is exactly 1 byte (8-bit unsigned integer) structured as follows:


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

#### Encoding Examples
* **`MOV a, 1`** (Move immediate value `1` into register `a`):
  * Opcode (`MOV`): `001` | Dst (`a`): `00` | Prefix (`imm`): `1` | Src (`1`): `01`
  * **Binary:** `00100101` $\rightarrow$ **Decimal:** `37`
* **`ADD b, c`** (Add the value of register `c` to register `b`):
  * Opcode (`ADD`): `010` | Dst (`b`): `01` | Prefix (`reg`): `0` | Src (`c`): `10`
  * **Binary:** `01001010` $\rightarrow$ **Decimal:** `74`

---

## Extended Instruction Set (Stdout Buffer)

When the Opcode is `000` (Bits 7–5 are `000`), LightVM can interpret the lower bits as **Extended Instructions** for manipulating an internal 16-bit storage buffer (`buf`).

### Extended Instruction Encoding


```

┌─────────────────┬───────────────┬───────────────┬──────────────────┐
│ Opcode=000 (3b) │  Prefix (1b)  │   Imm (2b)    │ Extended Op (2b) │
└─────────────────┴───────────────┴───────────────┴──────────────────┘
7             5         4         3           2   1              0  (Bit position)

```

* **Opcode (Bits 7–5):** Must be `000`.
* **Extended Prefix (Bit 4):**
  * `0`: Standard `NOP` (No operation).
  * `1`: Activate **Extended Instruction**.
* **Extended Imm (Bits 3–2):** A 2-bit literal value used by the `PUSH` operation.
* **Extended Opcode (Bits 1–0):** The specific buffer operation to execute:

| Binary Code | Extended Op | Description |
| --- | --- | --- |
| `00` | `PUSH` | Appends the 2-bit `Imm` value to the buffer (Max 16 bits / 8 times). |
| `01` | `POP` | Clears the lowest 2 bits of the buffer (`buf &= !0b11`). |
| `10` | `FLUSH` | Resets the entire buffer to 0. |
| `11` | `PRINT` | Prints the buffer contents in binary format (`print:0101...`). |

#### Extended Encoding Examples
* **`PUSH 1`** (Push 2-bit immediate value `01` into the buffer):
  * Opcode: `000` | Ext Prefix: `1` | Ext Imm: `01` | Ext Op: `00`
  * **Binary:** `00010100` $\rightarrow$ **Decimal:** `20`
* **`PRINT`** (Print the buffer binary representation):
  * Opcode: `000` | Ext Prefix: `1` | Ext Imm: `00` | Ext Op: `11`
  * **Binary:** `00010011` $\rightarrow$ **Decimal:** `19`

---

## Getting Started

### Prerequisites

Make sure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed on your system.

### Running the Project

1. Clone or navigate to the project directory.
2. Run the application using Cargo:
   ```bash
   cargo run
   ```

3. Type a decimal number (`0`–`254`) corresponding to your encoded instruction and press **Enter**.
4. **Exit the VM:** Type `255` and press **Enter**. This safely terminates the runtime loop.

---

## Example Usage

An interactive session putting values into registers, utilizing the extended stdout buffer, and shutting down:

```text
==========================vm==========================
37
register:
a: 1, b: 0, c: 0, d: 0
20
register:
a: 1, b: 0, c: 0, d: 0
19
print:1
register:
a: 1, b: 0, c: 0, d: 0
255
register:
a: 1, b: 0, c: 0, d: 0

```