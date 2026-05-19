# LightVM: 8-bit Custom Virtual Machine

A lightweight, 8-bit custom Virtual Machine implemented in Rust. It processes 8-bit instructions to manipulate four internal registers or manage an internal 16-bit output buffer using a custom ISA (Instruction Set Architecture).

## Table of Contents

* [Features](#features)
* [Instruction Set Architecture (ISA)](#instruction-set-architecture-isa)
* [Instruction Encoding](#instruction-encoding)
* [Extended Instruction Set (Stdout Buffer)](#extended-instruction-set-stdout-buffer)
* [CMP Jump & Exit](#cmp-jump--exit)
* [Getting Started](#getting-started)
* [Example Usage](#example-usage)

---

## Features

* **4 Internal Registers:** 8-bit general-purpose registers (`a`, `b`, `c`, `d`).
* **Flexible Addressing:** Supports both register-to-register and immediate-to-register (literal value) operations.
* **Basic Arithmetic & Logic:** Includes `MOV`, `ADD`, `SUB`, `DIV`, `MUL`, and `NAND`.
* **Safe Execution:** Implements wrapping arithmetic to prevent overflow panics, along with a zero-division guard.
* **Extended Features:** Includes a 16-bit internal buffer (`buf`) with stack-like operations (`PUSH`, `POP`, `FLUSH`, `PRINT`) for output control.
* **Control Flow:** Supports conditional jump (`CMP JUMP`) and program termination (`EXIT`) via the `111` opcode.

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
| `000` | `NOP` / `EXT` | No operation, or triggers the **Extended Instruction Set** if the extension prefix bit is `1`. |
| `001` | `MOV` | Move source value into the destination register. |
| `010` | `ADD` | Add source value to destination (wrapping). |
| `011` | `SUB` | Subtract source value from destination (wrapping). |
| `100` | `DIV` | Divide destination by source value (guarded against division by zero). |
| `101` | `MUL` | Multiply destination by source value (wrapping). |
| `110` | `NAND` | Perform bitwise NAND operation on destination and source. |
| `111` | `CMP JUMP` / `EXIT` | Conditional jump or program exit (see [CMP Jump & Exit](#cmp-jump--exit)). |

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
  * `0`: **Register Mode** — Src represents a register code `00` to `11`.
  * `1`: **Immediate Mode** — Src represents a literal value `0` to `3`.
* **Src (Bits 1–0):** The source register index or a 2-bit immediate value.

#### Encoding Examples

* **`MOV a, 1`** (Move immediate value `1` into register `a`):
  * Opcode (`MOV`): `001` | Dst (`a`): `00` | Prefix (`imm`): `1` | Src (`1`): `01`
  * **Binary:** `00100101` → **Decimal:** `37`

* **`ADD b, c`** (Add the value of register `c` to register `b`):
  * Opcode (`ADD`): `010` | Dst (`b`): `01` | Prefix (`reg`): `0` | Src (`c`): `10`
  * **Binary:** `01001010` → **Decimal:** `74`

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
| `00` | `PUSH` | Appends the 2-bit `Imm` value to the buffer (max 16 bits / 8 pushes). If the buffer is full, an error message is printed and the push is ignored. |
| `01` | `POP` | Removes the most recently pushed 2-bit value from the buffer (LIFO order). Has no effect if the buffer is empty. |
| `10` | `FLUSH` | Resets the entire buffer to `0` and clears the push counter. |
| `11` | `PRINT` | Interprets the buffer as a Unicode code point and prints the corresponding character. |

#### Extended Encoding Examples

* **`PUSH 1`** (Push 2-bit immediate value `01` into the buffer):
  * Opcode: `000` | Ext Prefix: `1` | Ext Imm: `01` | Ext Op: `00`
  * **Binary:** `00010100` → **Decimal:** `20`

* **`PRINT`**:
  * Opcode: `000` | Ext Prefix: `1` | Ext Imm: `00` | Ext Op: `11`
  * **Binary:** `00010011` → **Decimal:** `19`

---

## CMP Jump & Exit

When the Opcode is `111` (Bits 7–5 are `111`), LightVM interprets the instruction as either a **conditional jump** or a **program exit**, depending on the prefix bit.

### CMP Jump / Exit Encoding

```
┌─────────────────┬───────────────┬───────────────┬───────────────┐
│ Opcode=111 (3b) │  Prefix (1b)  │   Dst (2b)    │   Src (2b)    │
└─────────────────┴───────────────┴───────────────┴───────────────┘
7             5         4         3           2   1           0  (Bit position)
```

* **Prefix (Bit 4):**
  * `0`: **EXIT** — Immediately terminates the VM execution loop.
  * `1`: **CMP JUMP** — Compares `Dst` and `Src` registers. If they are equal, sets the program counter to the current value of `buf`.

### Encoding Examples

* **`EXIT`**:
  * Opcode: `111` | Prefix: `0` | Dst: `00` | Src: `00`
  * **Binary:** `11100000` → **Decimal:** `224`

* **`CMP JUMP a, b`** (Jump to `buf` if register `a` equals register `b`):
  * Opcode: `111` | Prefix: `1` | Dst (`a`): `00` | Src (`b`): `01`
  * **Binary:** `11110001` → **Decimal:** `241`

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
3. Provide encoded instructions as decimal numbers (`0`–`255`) corresponding to your encoded instruction bytes.
4. **Exit the VM:** Use an `EXIT` instruction (Opcode `111`, Prefix `0`), e.g. decimal `224`.

---

## Example Usage

An interactive session that builds the Unicode character `'a'` (code point `97` = `0b01100001`) in the buffer and prints it:

```text
==========================vm==========================
// PUSH 01 → buf = 0b01
20
register:
a: 0, b: 0, c: 0, d: 0
// PUSH 00 → buf = 0b0001
16
register:
a: 0, b: 0, c: 0, d: 0
// PUSH 00 → buf = 0b000001
16
register:
a: 0, b: 0, c: 0, d: 0
// PUSH 01 → buf = 0b01000001  (65 = 'A')
20
register:
a: 0, b: 0, c: 0, d: 0
// PRINT → outputs 'A'
19
A
register:
a: 0, b: 0, c: 0, d: 0
// FLUSH → reset buf and counter
18
register:
a: 0, b: 0, c: 0, d: 0
// EXIT
224
```