This `rng` crate contains PRNGs implementing a `Read` interface,
i.e. they are simply byte streams, and can be swapped in and out
with other backing readers (e.g. a randomness file).

This also contains some helper traits and functions for drawing
samples from distributions (src/dist.rs).
