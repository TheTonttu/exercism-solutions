const std = @import("std");

const FIRST_PRIME = 2;
// 💀
var sieve_mask_buffer: [10_000]bool = undefined;

pub fn primes(buffer: []u32, limit: u32) []u32 {
    if (limit < FIRST_PRIME) {
        return buffer[0..0];
    }

    if (limit > sieve_mask_buffer.len - 1) {
        @panic(std.fmt.comptimePrint("maximum supported limit is {}", .{sieve_mask_buffer.len - 1}));
    }

    var sieve_mask = sieve_mask_buffer[0..(limit + 1)];
    // 0 and 1 are not primes
    //@memset(sieve_mask[0..FIRST_PRIME], false);
    @memset(sieve_mask[FIRST_PRIME..], true);

    const largest_prime: u32 = sqrt(limit);
    for (FIRST_PRIME..(largest_prime + 1)) |n| {
        if (!sieve_mask[n]) {
            continue;
        }
        // mark prime composites
        const p = n;
        var i = p * p;
        while (i < sieve_mask.len) : (i += p) {
            sieve_mask[i] = false;
        }
    }

    var unused_buffer = buffer[0..];
    for (sieve_mask[FIRST_PRIME..], FIRST_PRIME..) |is_prime, n| {
        if (is_prime) {
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
