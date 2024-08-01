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
            '}', ')', ']' => if (!expectedClosedPair(c, &expected_closed_pairs)) {
                return false;
            },
            else => {},
        }
    }
    return expected_closed_pairs.items.len == 0;
}

fn expectedClosedPair(closed_pair: u8, closed_pair_queue: *std.ArrayList(u8)) bool {
    return closed_pair_queue.popOrNull() == closed_pair;
}
