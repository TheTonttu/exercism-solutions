const std = @import("std");
const mem = std.mem;

pub fn abbreviate(allocator: mem.Allocator, words: []const u8) mem.Allocator.Error![]u8 {
    var list = std.ArrayList(u8).init(allocator);
    defer list.deinit();
    var prevChar: u8 = std.ascii.control_code.nul;
    for (words) |char| {
        if (std.ascii.isAlphabetic(char) and
            !(std.ascii.isAlphabetic(prevChar) or isDiacriticalMark(prevChar)))
        {
            try list.append(std.ascii.toUpper(char));
        }
        prevChar = char;
    }
    return list.toOwnedSlice();
}

fn isDiacriticalMark(c: u8) bool {
    return switch (c) {
        '\'' => true,
        else => false,
    };
}
