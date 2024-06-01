const std = @import("std");
const mem = std.mem;

pub fn abbreviate(allocator: mem.Allocator, words: []const u8) mem.Allocator.Error![]u8 {
    var list = std.ArrayList(u8).init(allocator);
    defer list.deinit();

    var remainder = words[0..];
    while (remainder.len > 0) {
        const char = remainder[0];
        if (std.ascii.isAlphabetic(char)) {
            try list.append(std.ascii.toUpper(char));
            // Skip until the word ends
            while (remainder.len > 0 and isPartOfWord(remainder[0])) {
                remainder = remainder[1..];
            }
        } else {
            // Skip until next word begins
            while (remainder.len > 0 and !isPartOfWord(remainder[0])) {
                remainder = remainder[1..];
            }
        }
    }
    return list.toOwnedSlice();
}

fn isDiacriticalMark(c: u8) bool {
    return switch (c) {
        '\'' => true,
        else => false,
    };
}

fn isPartOfWord(c: u8) bool {
    return std.ascii.isAlphabetic(c) or isDiacriticalMark(c);
}
