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
    return score & @intFromEnum(allergen) != 0;
}

pub fn initAllergenSet(score: usize) EnumSet(Allergen) {
    var allergen_set = EnumSet(Allergen).initEmpty();

    for (std.enums.values(Allergen)) |allergen| {
        if (isAllergicTo(@truncate(score), allergen)) {
            allergen_set.insert(allergen);
        }
    }

    return allergen_set;
}
