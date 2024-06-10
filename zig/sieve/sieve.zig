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

    const largest_prime: u32 = sqrt(limit);
    for (FIRST_PRIME..largest_prime + 1) |n| {
        if (!sieve_mask.isSet(n)) {
            continue;
        }
        // mark prime composites
        const p = n;
        var i = p * p;
        while (i <= limit) : (i += p) {
            sieve_mask.unset(i);
        }
    }

    var unused_buffer = buffer[0..];
    for (FIRST_PRIME..limit + 1) |n| {
        if (sieve_mask.isSet(n)) {
            unused_buffer[0] = @intCast(n);
            unused_buffer = unused_buffer[1..];
        }
    }
    const used_buffer_len = buffer.len - unused_buffer.len;
    return buffer[0..used_buffer_len];
}

fn sqrt(number: u32) u32 {
    return @intFromFloat(@ceil(@sqrt(@as(f64, @floatFromInt(number)))));
}
