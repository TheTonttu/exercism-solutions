const std = @import("std");

const FIRST_PRIME = 2;

pub fn primes(buffer: []u32, comptime limit: u32) []u32 {
    if (limit < FIRST_PRIME) {
        return buffer[0..0];
    }

    var sieve_mask = std.StaticBitSet(limit + 1).initFull();
    // 0 and 1 are not primes
    // sieve_mask.unset(0);
    // sieve_mask.unset(1);

    var unused_buffer = buffer[0..];
    for (FIRST_PRIME..limit + 1) |n| {
        if (!sieve_mask.isSet(n)) {
            continue;
        }

        const prime: u32 = @intCast(n);
        unused_buffer[0] = prime;
        unused_buffer = unused_buffer[1..];

        var composite = prime * prime;
        while (composite <= limit) : (composite += prime) {
            sieve_mask.unset(composite);
        }
    }

    const used_buffer_len = buffer.len - unused_buffer.len;
    return buffer[0..used_buffer_len];
}
