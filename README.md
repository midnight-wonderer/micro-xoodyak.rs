# MicroXoodyak

[![Crates.io](https://img.shields.io/crates/v/micro-xoodyak.svg)](https://crates.io/crates/micro-xoodyak)
[![Documentation](https://docs.rs/micro-xoodyak/badge.svg)](https://docs.rs/micro-xoodyak)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)

`MicroXoodyak` is a lightweight, zero-allocation cryptographic library specifically optimized for **32-bit microcontrollers** (such as ARM Cortex-M and 32-bit RISC-V). It is a fork of the `xoodyak` crate, designed to provide a highly performant and secure `no_std` implementation of the **Xoodyak** permutation-based cryptographic scheme.

Xoodyak is a versatile primitive designed by the Keccak team (creators of SHA-3). Operating within the **Cyclist** framework, it can handle:
* **Hashing** (arbitrary-length output)
* **Message Authentication Codes (MAC)**
* **Authenticated Encryption with Associated Data (AEAD)**
* **Session-based encryption & ratcheting** (incremental streaming and state progression)

---

## Key Features

* **Strict `no_std` by Default**: Zero dynamic memory allocations (`no_alloc`). All operations run on the stack, making it ideal for bare-metal targets.
* **Optimized for 32-bit Hardware**: Native assembly and architecture-specific implementations for:
  * ARM Thumb-1 (e.g., Cortex-M0, Cortex-M0+, Cortex-M1)
  * ARM Thumb-2 (e.g., Cortex-M3, Cortex-M4, Cortex-M7, Cortex-M33)
  * RISC-V 32-bit (e.g., RV32I, RV32E, ESP32-C3, etc.)
* **Security First**: Automatic zeroization of sensitive internal state on drop using the `zeroize` crate.
* **Flexible AEAD**: Supports both in-place and buffer-to-buffer authenticated encryption with attached or detached tags.
* **Streaming Support**: Easily squeeze and absorb data in chunks.

---

## Installation

Add `micro-xoodyak` to your `Cargo.toml`:

```toml
[dependencies]
micro-xoodyak = "0.0.1"
```

### Features

* `alloc`: Enables helper methods that return heap-allocated `Vec<u8>` objects. This feature is optional and disabled by default.

---

## Examples

### 1. Cryptographic Hashing
For simple, unkeyed hashing of arbitrary-length data.

```rust,ignore
use micro_xoodyak::{XoodyakHash, XoodyakCommon};

let mut hash = XoodyakHash::new();

// Absorb data incrementally
hash.absorb(b"Hello ");
hash.absorb(b"MicroXoodyak!");

// Squeeze out the hash bytes
let mut output = [0u8; 32];
hash.squeeze(&mut output);
```

### 2. Authenticated Encryption (AEAD)
For encrypting messages with a shared key and verifying integrity with associated data.

```rust,ignore
use micro_xoodyak::{XoodyakKeyed, XoodyakCommon, XOODYAK_AUTH_TAG_BYTES};

let key = [0u8; 16];
let nonce = [1u8; 16];
let mut cipher = XoodyakKeyed::new(&key, Some(&nonce), None, None).unwrap();

// Absorb optional associated data
cipher.absorb(b"associated metadata");

let message = b"highly sensitive data";
let mut ciphertext = [0u8; 21 + XOODYAK_AUTH_TAG_BYTES]; // message len + 16-byte tag

// Encrypt and append the authentication tag
cipher.aead_encrypt(&mut ciphertext, Some(message)).unwrap();
```

### 3. In-Place Decryption & Verification
In-place operations are ideal for memory-constrained microcontrollers as they avoid buffer copying.

```rust,ignore
use micro_xoodyak::{XoodyakKeyed, XoodyakCommon};

let key = [0u8; 16];
let nonce = [1u8; 16];
let mut cipher = XoodyakKeyed::new(&key, Some(&nonce), None, None).unwrap();

// Must absorb the identical associated data used during encryption
cipher.absorb(b"associated metadata");

// Decrypt the ciphertext buffer in-place and verify its tag
let mut buffer = ciphertext; // Buffer contains [ciphertext || tag]
let decrypted = cipher.aead_decrypt_in_place(&mut buffer).unwrap();

assert_eq!(decrypted, b"highly sensitive data");
```

### 4. Stateful Hashing & Ratcheting (Session Mode)
Xoodyak's design allows chaining multiple operations (absorb, squeeze, encrypt) in a single session. Ratcheting provides forward secrecy by key rolling.

```rust,ignore
use micro_xoodyak::{XoodyakKeyed, XoodyakCommon};

let key = [0u8; 16];
let mut session = XoodyakKeyed::new(&key, None, None, None).unwrap();

// Absorb a command
session.absorb(b"first-command");

// Ratchet the state to secure the session up to this point
session.ratchet();

// Squeeze a session token
let mut session_token = [0u8; 16];
session.squeeze(&mut session_token);
```

---

## Hardware Optimizations

MicroXoodyak automatically selects the most optimized implementation path at compile time based on your target architecture:

| Target Architecture | Optimization Path | Target Devices |
| --- | --- | --- |
| ARM Thumb-1 | Optimized Thumb-1 Assembly | Cortex-M0, Cortex-M0+, Cortex-M1 |
| ARM Thumb-2 | Optimized Thumb-2 Assembly | Cortex-M3, Cortex-M4, Cortex-M7, Cortex-M33, ARMv7-R, ARMv8-M |
| RISC-V 32-bit (RV32I) | Optimized RV32I Assembly (32 registers) | RV32IMAC, ESP32-C3, FE310, etc. |
| RISC-V 32-bit (RV32E) | Optimized RV32E Assembly (16 registers) | RV32EC, low-power/budget embedded MCUs |
| x86_64 | Optimized SIMD/Vector | Intel/AMD 64-bit platforms |
| Others | Portable Rust Fallback | WASM, other architectures |

---

## License

This project is licensed under the MIT License. See [LICENSE.md](LICENSE.md) for details.
