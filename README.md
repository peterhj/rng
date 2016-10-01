# librng

This Rust library contains implementations of random number generators, as well
as auxiliary RNG traits.

Currently, it contains an implementation of the original xorshift128+ RNG
(https://arxiv.org/pdf/1404.0390v1.pdf), where the addition is done after the
shifts.
