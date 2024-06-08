const std = @import("std");
const EnumSet = std.EnumSet;

pub const Allergen = enum(u8) {
    eggs = 1,
    peanuts = 2,
    shellfish = 4,
    strawberries = 8,
    tomatoes = 16,
    chocolate = 32,
    pollen = 64,
    cats = 128,
};

pub fn isAllergicTo(score: u8, allergen: Allergen) bool {
    const allergen_set = initAllergenSet(score);
    return allergen_set.contains(allergen);
}

pub fn initAllergenSet(score: usize) EnumSet(Allergen) {
    var allergen_set = EnumSet(Allergen).initEmpty();

    inline for (std.meta.fields(Allergen)) |allergen| {
        if (is_set(score, allergen.value)) {
            allergen_set.insert(@enumFromInt(allergen.value));
        }
    }

    return allergen_set;
}

inline fn is_set(value: usize, bit_mask: usize) bool {
    return value & bit_mask != 0;
}
