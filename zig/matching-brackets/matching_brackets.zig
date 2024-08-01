const std = @import("std");
const mem = std.mem;

pub fn isBalanced(allocator: mem.Allocator, s: []const u8) !bool {
    var expected_closed_pairs = std.ArrayList(u8).init(allocator);
    defer expected_closed_pairs.deinit();

    for (s) |c| {
        switch (c) {
            '{' => try expected_closed_pairs.append('}'),
            '(' => try expected_closed_pairs.append(')'),
            '[' => try expected_closed_pairs.append(']'),
            '}', ')', ']' => if (expected_closed_pairs.popOrNull() != c) {
                return false;
            },
            else => {},
        }
    }
    return expected_closed_pairs.items.len == 0;
}
