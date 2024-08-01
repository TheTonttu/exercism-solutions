const std = @import("std");
const mem = std.mem;

pub fn isBalanced(allocator: mem.Allocator, s: []const u8) !bool {
    var expected_close_pairs = std.ArrayList(u8).init(allocator);
    defer expected_close_pairs.deinit();

    for (s) |c| {
        switch (c) {
            '{', '(', '[' => try expected_close_pairs.append(closePair(c)),
            '}', ')', ']' => if (expected_close_pairs.popOrNull() != c) {
                return false;
            },
            else => {},
        }
    }
    return expected_close_pairs.items.len == 0;
}

fn closePair(open_pair: u8) u8 {
    return switch (open_pair) {
        '{' => '}',
        '(' => ')',
        '[' => ']',
        else => unreachable,
    };
}
