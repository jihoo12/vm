# LightVM: 8-bit Custom Virtual Machine

A lightweight, 8-bit custom Virtual Machine implemented in Rust. It processes 8-bit instructions to manipulate four internal registers, a 32-bit output buffer, and a 4000-byte memory space using a custom ISA (Instruction Set Architecture).

## Table of Contents

* [Features](#features)
* [Instruction Set Architecture (ISA)](#instruction-set-architecture-isa)
* [Instruction Encoding](#instruction-encoding)
* [Extended Instruction Set (Buffer Operations)](#extended-instruction-set-buffer-operations)
* [Extended Extended Instruction Set (Memory LOAD/STORE)](#extended-extended-instruction-set-memory-loadstore)
* [CMP Jump & Exit](#cmp-jump--exit)
* [Getting Started](#getting-started)
* [Example Usage](#example-usage)

---

## Features

* **4 Internal Registers:** 8-bit general-purpose registers (`a`, `b`, `c`, `d`).
* **Flexible Addressing:** Supports both register-to-register and immediate-to-register (literal value) operations.
* **Basic Arithmetic & Logic:** Includes `MOV`, `ADD`, `SUB`, `DIV`, `MUL`, and `NAND`.
* **Safe Execution:** Implements wrapping arithmetic to prevent overflow panics, along with a zero-division guard.
* **Extended Features:** Includes a 32-bit internal buffer (`buf`) with stack-like operations (`PUSH`, `POP`, `FLUSH`, `PRINT`) for output control.
* **Memory Access:** 4000-byte memory array accessible via `LOAD` and `STORE` instructions, using `buf` as the address pointer.
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
| `000` | `EXT` | Triggers the **Extended Instruction Set** or **Extended Extended Instruction Set** depending on lower bits. |
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

Each instruction is exactly 1 byte (8-bit unsigned integer). The bit layout follows the specification:

```
┌───────────────┬──────────────┬───────────────┬───────────────┐
│  Opcode (3b)  │  Src Reg(2b) │  Prefix (1b)  │   Dst (2b)    │
└───────────────┴──────────────┴───────────────┴───────────────┘
7           5   4          3         2         1           0  (Bit position)
```

* **Opcode (Bits 7–5):** Determines the operation to execute.
* **Src (Bits 4–3):** The source register (`a`, `b`, `c`, or `d`).
* **Prefix (Bit 2):** Determines how the `Dst` bits are interpreted:
  * `0`: **Register Mode** — Dst represents a register code `00` to `11`.
  * `1`: **Immediate Mode** — Dst represents a literal value `0` to `3`.
* **Dst (Bits 1–0):** The destination register index or a 2-bit immediate value.

> **Note:** In the actual Rust implementation (`vm.rs`), the bit fields are parsed as `dst = bits[4:3]`, `prefix = bit[2]`, `src = bits[1:0]`. The destination receives the result, and the source provides the operand value.

#### Encoding Examples

* **`MOV a, 1`** (Move immediate value `1` into register `a`):
  * Opcode (`MOV`): `001` | Dst (`a`): `00` | Prefix (`imm`): `1` | Src (`1`): `01`
  * **Binary:** `00100101` → **Decimal:** `37`

* **`ADD b, c`** (Add the value of register `c` to register `b`):
  * Opcode (`ADD`): `010` | Dst (`b`): `01` | Prefix (`reg`): `0` | Src (`c`): `10`
  * **Binary:** `01001010` → **Decimal:** `74`

---

## Extended Instruction Set (Buffer Operations)

When the Opcode is `000` (Bits 7–5 are `000`), LightVM interprets the lower bits as **Extended Instructions** for manipulating an internal 32-bit buffer (`buf`).

### Extended Instruction Encoding

```
┌─────────────────┬───────────────┬───────────────┬──────────────────┐
│ Opcode=000 (3b) │  Prefix (1b)  │  Reg Src (2b) │ Extended Op (2b) │
└─────────────────┴───────────────┴───────────────┴──────────────────┘
7             5         4         3           2   1              0  (Bit position)
```

* **Opcode (Bits 7–5):** Must be `000`.
* **Extended Prefix (Bit 4):**
  * `0`: Triggers the **Extended Extended Instruction Set** (LOAD/STORE). See next section.
  * `1`: Activate **Extended Instruction** (buffer operations).
* **Reg Src (Bits 3–2):** The register whose value is used by the `PUSH` operation.
* **Extended Opcode (Bits 1–0):** The specific buffer operation to execute:

| Binary Code | Extended Op | Description |
| --- | --- | --- |
| `00` | `PUSH` | Appends the **register value** specified by `Reg Src` to the buffer (2 bits at a time, max 16 pushes / 32 bits). If the buffer is full (`count >= 16`), an error message is printed and the push is ignored. |
| `01` | `POP` | Removes the most recently pushed 2-bit segment from the buffer (LIFO order). Has no effect if the buffer is empty. |
| `10` | `FLUSH` | Resets the entire buffer to `0` and clears the push counter. |
| `11` | `PRINT` | Interprets the current buffer value as a Unicode code point and prints the corresponding character. |

> **Important:** `PUSH` reads from a **register** (bits 3–2 select the register), not a literal immediate value. The 2 least-significant bits of that register's value are appended to `buf`.

#### Extended Encoding Examples

* **`PUSH a`** (Push 2 LSBs of register `a` into the buffer):
  * Opcode: `000` | Ext Prefix: `1` | Reg Src (`a`): `00` | Ext Op: `00`
  * **Binary:** `00010000` → **Decimal:** `16`

* **`PUSH b`** (Push 2 LSBs of register `b` into the buffer):
  * Opcode: `000` | Ext Prefix: `1` | Reg Src (`b`): `01` | Ext Op: `00`
  * **Binary:** `00010100` → **Decimal:** `20`

* **`POP`**:
  * Opcode: `000` | Ext Prefix: `1` | Reg Src: `00` | Ext Op: `01`
  * **Binary:** `00010001` → **Decimal:** `17`

* **`FLUSH`**:
  * Opcode: `000` | Ext Prefix: `1` | Reg Src: `00` | Ext Op: `10`
  * **Binary:** `00010010` → **Decimal:** `18`

* **`PRINT`**:
  * Opcode: `000` | Ext Prefix: `1` | Reg Src: `00` | Ext Op: `11`
  * **Binary:** `00010011` → **Decimal:** `19`

---

## Extended Extended Instruction Set (Memory LOAD/STORE)

When the Opcode is `000` **and** Bit 4 (Extended Prefix) is `0`, LightVM interprets the instruction as a **memory access** operation using `buf` as the address pointer.

### Extended Extended Instruction Encoding

```
┌──────────────────────┬──────────────────────┬───────────────┬────────────┐
│ Opcode+Prefix=0000(4b)│  LS Prefix (1b)      │  Reg Src (2b) │ Unused (1b)│
└──────────────────────┴──────────────────────┴───────────────┴────────────┘
7                   4         3               2           1         0  (Bit position)
```

* **Bits 7–4:** Must all be `0000`.
* **LS Prefix (Bit 3):**
  * `0`: **LOAD** — Load the byte from `mem[buf]` into the register specified by `Reg Src`.
  * `1`: **STORE** — Store the value of the register specified by `Reg Src` into `mem[buf]`.
* **Reg Src (Bits 2–1):** The register to read from (STORE) or write to (LOAD).
* **Bit 0:** Unused.

The VM has a 4000-byte memory array (`mem`). The current value of `buf` is used as the memory address for both reads and writes.

#### Extended Extended Encoding Examples

* **`LOAD a`** (Load `mem[buf]` into register `a`):
  * Fixed `0000` | LS Prefix (`LOAD`): `0` | Reg Src (`a`): `00` | Unused: `0`
  * **Binary:** `00000000` → **Decimal:** `0`

* **`STORE b`** (Store register `b` into `mem[buf]`):
  * Fixed `0000` | LS Prefix (`STORE`): `1` | Reg Src (`b`): `01` | Unused: `0`
  * **Binary:** `00001010` → **Decimal:** `10`

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
  * `1`: **CMP JUMP** — Compares the registers at `Dst` and `Src` indices. If their values are equal, sets the program counter to the current value of `buf`.

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
2. Create a `test.txt` file containing decimal instruction values (0–255), one per line or space-separated.
3. Run the application using Cargo:
   ```bash
   cargo run
   ```
4. **Exit the VM:** Use an `EXIT` instruction (Opcode `111`, Prefix `0`), e.g. decimal `224`.

---

## Example Usage

An interactive session that builds the Unicode character `'A'` (code point `65` = `0b01000001`) in the buffer and prints it.

The character `'A'` in binary is `01000001`. We push it 2 bits at a time (LSB first) using register values:

| Step | Register setup | Instruction | buf (binary) |
| --- | --- | --- | --- |
| MOV a, 1 | `a = 1` (binary `01`) | `37` | — |
| PUSH a | push `01` | `16` (adjusted for reg `a`) | `01` |
| MOV a, 0 | `a = 0` (binary `00`) | `33` | — |
| PUSH a | push `00` | `16` | `0001` |
| PUSH a | push `00` | `16` | `000001` |
| MOV a, 1 | `a = 1` (binary `01`) | `37` | — |
| PUSH a | push `01` | `16` | `01000001` = 65 = `'A'` |
| PRINT | print buf as Unicode | `19` | — |
| FLUSH | reset buf | `18` | `0` |
| EXIT | halt VM | `224` | — |

`test.txt` contents:
```
37 16 33 16 16 37 16 19 18 224
```

Expected output:
```
A
```

---

## Internal State

| Field | Type | Description |
| --- | --- | --- |
| `regs` | `[u8; 4]` | Four 8-bit general-purpose registers (a, b, c, d) |
| `buf` | `u32` | 32-bit buffer used for Unicode output and memory addressing |
| `count` | `u8` | Tracks how many 2-bit segments have been pushed into `buf` (max 16) |
| `mem` | `[u8; 4000]` | 4000-byte random-access memory, addressed via `buf` |