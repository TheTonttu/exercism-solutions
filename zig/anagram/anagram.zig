const std = @import("std");
const mem = std.mem;

const alphabet_count = 'z' - 'a' + 1;

/// Returns the set of strings in `candidates` that are anagrams of `word`.
/// Caller owns the returned memory.
pub fn detectAnagrams(
    allocator: mem.Allocator,
    word: []const u8,
    candidates: []const []const u8,
) !std.BufSet {
    var word_char_count = std.mem.zeroes([alphabet_count]u16);
    for (word) |character| {
        const alphabet_index = 'z' - std.ascii.toLower(character);
        // Stop early when word has non-alphabetic characters.
        if (alphabet_index >= word_char_count.len) {
            const empty = std.BufSet.init(allocator);
            return empty;
        }
        word_char_count[alphabet_index] += 1;
    }

    var anagrams = std.BufSet.init(allocator);

    var tally = std.mem.zeroes([alphabet_count]u16);
    for (candidates) |candidate| {
        @memcpy(&tally, &word_char_count);
        if (isAnagram(word, candidate, &tally)) {
            try anagrams.insert(candidate);
        }
    }

    return anagrams;
}

fn isAnagram(word: []const u8, candidate: []const u8, tally: []u16) bool {
    if (word.len != candidate.len) {
        return false;
    }

    var exact_match = true;
    for (word, candidate) |word_character, candidate_character| {
        const lower_word_char = std.ascii.toLower(word_character);
        const lower_candidate_char = std.ascii.toLower(candidate_character);
        if (lower_word_char != lower_candidate_char) {
            exact_match = false;
        }

        const alphabet_index = 'z' - lower_candidate_char;
        if (alphabet_index >= tally.len or tally[alphabet_index] <= 0) {
            return false;
        }
        tally[alphabet_index] -= 1;
    }

    if (exact_match) {
        return false;
    }

    for (tally) |count| {
        if (count != 0) {
            return false;
        }
    }

    return true;
}
