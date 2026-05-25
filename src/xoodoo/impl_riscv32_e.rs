use core::arch::asm;

use super::{ROUND_KEYS, Xoodoo};

impl Xoodoo {
    /// Permutes the 12-word state using the RV32E optimized assembly.
    #[allow(clippy::many_single_char_names)]
    pub fn permute(&mut self) {
        let st_words = unsafe { &mut *(self.st.as_mut_ptr() as *mut [u32; 12]) };
        let rkeys = ROUND_KEYS.as_ptr();
        let rkeys_end = unsafe { rkeys.add(12) };

        let state_ptr = st_words.as_mut_ptr();
        let rkeys_ptr = rkeys;

        unsafe {
            asm!(
                // Allocate 40 bytes on the stack for storing columns parity, metadata, and s0/s1
                "addi    sp, sp, -40",
                // Store s0, s1, state_ptr, rkeys_ptr, rkeys_end
                "sw      s0, 28(sp)",
                "sw      s1, 32(sp)",
                "sw      a0, 16(sp)",
                "sw      a1, 20(sp)",
                "sw      a2, 24(sp)",

                // Load Row 1 (A10, A11, A12, A13) from state into s0, s1, a2, a3
                "lw      s0, 16(a0)",
                "lw      s1, 20(a0)",
                "lw      a2, 24(a0)",
                "lw      a3, 28(a0)",

                // Load Row 2 (A20, A21, A22, A23) from state into a4, a5, t0, t1
                "lw      a4, 32(a0)",
                "lw      a5, 36(a0)",
                "lw      t0, 40(a0)",
                "lw      t1, 44(a0)",

                "2:", // Loop start label

                // Theta step

                // Column 0: P0 = A00 ^ s0 (A10) ^ a4 (A20)
                "lw      ra, 0(a0)",
                "xor     ra, ra, s0",
                "xor     ra, ra, a4",
                "sw      ra, 0(sp)",

                // Column 1: P1 = A01 ^ s1 (A11) ^ a5 (A21)
                "lw      ra, 4(a0)",
                "xor     ra, ra, s1",
                "xor     ra, ra, a5",
                "sw      ra, 4(sp)",

                // Column 2: P2 = A02 ^ a2 (A12) ^ t0 (A22)
                "lw      ra, 8(a0)",
                "xor     ra, ra, a2",
                "xor     ra, ra, t0",
                "sw      ra, 8(sp)",

                // Column 3: P3 = A03 ^ a3 (A13) ^ t1 (A23)
                "lw      ra, 12(a0)",
                "xor     ra, ra, a3",
                "xor     ra, ra, t1",
                "sw      ra, 12(sp)",

                // --- Column 0 (uses P3 from 12(sp)) ---
                "lw      t2, 12(sp)",
                "slli    ra, t2, 5",
                "srli    t2, t2, 27",
                "or      ra, ra, t2",          // ra = ROTL32(P3, 5)
                "lw      t2, 12(sp)",
                "slli    t2, t2, 14",
                "srli    t2, t2, 18",
                "or      t2, t2, ra",          // t2 = E0

                "xor     s0, s0, t2",          // s0 (A10) ^= E0
                "xor     a4, a4, t2",          // a4 (A20) ^= E0
                "lw      ra, 0(a0)",
                "xor     ra, ra, t2",
                "sw      ra, 0(a0)",           // A00 ^= E0

                // --- Column 1 (uses P0 from 0(sp)) ---
                "lw      t2, 0(sp)",
                "slli    ra, t2, 5",
                "srli    t2, t2, 27",
                "or      ra, ra, t2",          // ra = ROTL32(P0, 5)
                "lw      t2, 0(sp)",
                "slli    t2, t2, 14",
                "srli    t2, t2, 18",
                "or      t2, t2, ra",          // t2 = E1

                "xor     s1, s1, t2",          // s1 (A11) ^= E1
                "xor     a5, a5, t2",          // a5 (A21) ^= E1
                "lw      ra, 4(a0)",
                "xor     ra, ra, t2",
                "sw      ra, 4(a0)",           // A01 ^= E1

                // --- Column 2 (uses P1 from 4(sp)) ---
                "lw      t2, 4(sp)",
                "slli    ra, t2, 5",
                "srli    t2, t2, 27",
                "or      ra, ra, t2",          // ra = ROTL32(P1, 5)
                "lw      t2, 4(sp)",
                "slli    t2, t2, 14",
                "srli    t2, t2, 18",
                "or      t2, t2, ra",          // t2 = E2

                "xor     a2, a2, t2",          // a2 (A12) ^= E2
                "xor     t0, t0, t2",          // t0 (A22) ^= E2
                "lw      ra, 8(a0)",
                "xor     ra, ra, t2",
                "sw      ra, 8(a0)",           // A02 ^= E2

                // --- Column 3 (uses P2 from 8(sp)) ---
                "lw      t2, 8(sp)",
                "slli    ra, t2, 5",
                "srli    t2, t2, 27",
                "or      ra, ra, t2",          // ra = ROTL32(P2, 5)
                "lw      t2, 8(sp)",
                "slli    t2, t2, 14",
                "srli    t2, t2, 18",
                "or      t2, t2, ra",          // t2 = E3

                "xor     a3, a3, t2",          // a3 (A13) ^= E3
                "xor     t1, t1, t2",          // t1 (A23) ^= E3
                "lw      ra, 12(a0)",
                "xor     ra, ra, t2",
                "sw      ra, 12(a0)",          // A03 ^= E3

                // Rho-west: Plane Shift Row 1 & Rotate Row 2
                // Shift Row 1: (s0, s1, a2, a3) <- (a3, s0, s1, a2)
                "mv      t2, a3",
                "mv      a3, a2",
                "mv      a2, s1",
                "mv      s1, s0",
                "mv      s0, t2",

                // Rotate Row 2 left by 11 bits: A2i = ROTL32(A2i, 11)
                // A20 (a4)
                "slli    t2, a4, 11",
                "srli    ra, a4, 21",
                "or      a4, ra, t2",

                // A21 (a5)
                "slli    t2, a5, 11",
                "srli    ra, a5, 21",
                "or      a5, ra, t2",

                // A22 (t0)
                "slli    t2, t0, 11",
                "srli    ra, t0, 21",
                "or      t0, ra, t2",

                // A23 (t1)
                "slli    t2, t1, 11",
                "srli    ra, t1, 21",
                "or      t1, ra, t2",

                // Iota: Round Constant
                "lw      t2, 20(sp)",          // Load RC_ptr
                "lw      ra, 0(t2)",           // Load constant
                "addi    t2, t2, 4",
                "sw      t2, 20(sp)",          // Store updated RC_ptr

                "lw      a1, 0(a0)",           // A00
                "xor     a1, a1, ra",          // A00 ^= rc
                "sw      a1, 0(a0)",

                // Chi: Non-linear Step (on Columns)

                // --- Column 0 ---
                "lw      t2, 0(a0)",           // t2 = A00
                "not     ra, s0",
                "and     ra, ra, a4",
                "xor     ra, ra, t2",          // ra = B0
                "not     a1, a4",
                "and     a1, a1, t2",
                "xor     a1, a1, s0",          // a1 = B1
                "not     t2, t2",
                "and     t2, t2, s0",
                "xor     a4, a4, t2",          // a4 = B2 (A20 updated)
                "mv      s0, a1",              // s0 = B1 (A10 updated)
                "sw      ra, 0(a0)",           // A00 = B0

                // --- Column 1 ---
                "lw      t2, 4(a0)",           // t2 = A01
                "not     ra, s1",
                "and     ra, ra, a5",
                "xor     ra, ra, t2",          // ra = B0
                "not     a1, a5",
                "and     a1, a1, t2",
                "xor     a1, a1, s1",          // a1 = B1
                "not     t2, t2",
                "and     t2, t2, s1",
                "xor     a5, a5, t2",          // a5 = B2 (A21 updated)
                "mv      s1, a1",              // s1 = B1 (A11 updated)
                "sw      ra, 4(a0)",           // A01 = B0

                // --- Column 2 ---
                "lw      t2, 8(a0)",           // t2 = A02
                "not     ra, a2",
                "and     ra, ra, t0",
                "xor     ra, ra, t2",          // ra = B0
                "not     a1, t0",
                "and     a1, a1, t2",
                "xor     a1, a1, a2",          // a1 = B1
                "not     t2, t2",
                "and     t2, t2, a2",
                "xor     t0, t0, t2",          // t0 = B2 (A22 updated)
                "mv      a2, a1",              // a2 = B1 (A12 updated)
                "sw      ra, 8(a0)",           // A02 = B0

                // --- Column 3 ---
                "lw      t2, 12(a0)",          // t2 = A03
                "not     ra, a3",
                "and     ra, ra, t1",
                "xor     ra, ra, t2",          // ra = B0
                "not     a1, t1",
                "and     a1, a1, t2",
                "xor     a1, a1, a3",          // a1 = B1
                "not     t2, t2",
                "and     t2, t2, a3",
                "xor     t1, t1, t2",          // t1 = B2 (A23 updated)
                "mv      a3, a1",              // a3 = B1 (A13 updated)
                "sw      ra, 12(a0)",          // A03 = B0

                // Rho-east: Plane Shift & Rotate Rows
                // Rotate Row 1 left by 1 bit: (s0, s1, a2, a3)
                "slli    t2, s0, 1",
                "srli    ra, s0, 31",
                "or      s0, ra, t2",

                "slli    t2, s1, 1",
                "srli    ra, s1, 31",
                "or      s1, ra, t2",

                "slli    t2, a2, 1",
                "srli    ra, a2, 31",
                "or      a2, ra, t2",

                "slli    t2, a3, 1",
                "srli    ra, a3, 31",
                "or      a3, ra, t2",

                // Rotate Row 2 left by 8 bits and cyclic shift by 2 lanes:
                "mv      ra, a4",              // Store old_a4
                "mv      a1, a5",              // Store old_a5

                "slli    t2, t0, 8",
                "srli    a4, t0, 24",
                "or      a4, a4, t2",          // a4 = new_a4

                "slli    t2, t1, 8",
                "srli    a5, t1, 24",
                "or      a5, a5, t2",          // a5 = new_a5

                "slli    t2, ra, 8",
                "srli    t0, ra, 24",
                "or      t0, t0, t2",          // t0 = new_t0

                "slli    t2, a1, 8",
                "srli    t1, a1, 24",
                "or      t1, t1, t2",          // t1 = new_t1

                // Loop Check
                "lw      t2, 20(sp)",          // current RC_ptr
                "lw      ra, 24(sp)",          // RC_end
                "bne     t2, ra, 2b",

                // Write Row 1 (s0, s1, a2, a3)
                "sw      s0, 16(a0)",
                "sw      s1, 20(a0)",
                "sw      a2, 24(a0)",
                "sw      a3, 28(a0)",

                // Write Row 2 (a4, a5, t0, t1)
                "sw      a4, 32(a0)",
                "sw      a5, 36(a0)",
                "sw      t0, 40(a0)",
                "sw      t1, 44(a0)",

                // Restore s0 and s1
                "lw      s0, 28(sp)",
                "lw      s1, 32(sp)",
                // Deallocate stack space
                "addi    sp, sp, 40",

                inout("a0") state_ptr => _,
                inout("a1") rkeys_ptr => _,
                inout("a2") rkeys_end => _,
                out("ra") _,
                out("t0") _, out("t1") _, out("t2") _,
                out("a3") _, out("a4") _, out("a5") _,
            );
        }
    }
}
