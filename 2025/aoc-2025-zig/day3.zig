const std = @import("std");

const Bank = []const u8;

pub fn main() !void {
    const input = @embedFile("./puzzle_input/d3");
    const banks = try parse_input(input);

    const p1 = solve_part_1(banks);
    std.debug.print("{}\n", .{p1});

    const p2 = solve_part_2(banks);
    std.debug.print("{}\n", .{p2});
}

fn get_joltage(bank: Bank, num_digts: u8) u64 {
    var result: u64 = 0;
    var bank_remaining: Bank = bank;

    for (1..num_digts + 1) |digit_num| {
        const slice = bank_remaining[0 .. bank_remaining.len - (num_digts - digit_num)];
        var max: u8 = 0;
        var max_index: usize = 0;
        for (slice, 0..) |charge, i| {
            if (charge > max) {
                max = charge;
                max_index = i;
            }
        }
        const digit = max;
        result *= 10;
        result += digit - '0';
        bank_remaining = bank_remaining[max_index + 1 ..];
    }

    return result;
}

fn solve_part_1(banks: []Bank) u64 {
    var sum: u64 = 0;
    for (banks) |bank| {
        sum += get_joltage(bank, 2);
    }
    return sum;
}

fn solve_part_2(banks: []Bank) u64 {
    var sum: u64 = 0;
    for (banks) |bank| {
        sum += get_joltage(bank, 12);
    }
    return sum;
}

fn parse_input(input: []const u8) ![]Bank {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    var banks = try std.ArrayList(Bank).initCapacity(allocator, 100);

    const trimmed_input = std.mem.trim(u8, input, &[_]u8{'\n'});
    var line_it = std.mem.splitScalar(u8, trimmed_input, '\n');
    while (line_it.next()) |bank| {
        try banks.append(allocator, bank);
    }

    return banks.toOwnedSlice(allocator);
}

const test_input =
    \\987654321111111
    \\811111111111119
    \\234234234234278
    \\818181911112111
;

test "testing joltage calculation" {
    try std.testing.expectEqual(98, get_joltage("987654321111111", 2));
    try std.testing.expectEqual(89, get_joltage("811111111111119", 2));
    try std.testing.expectEqual(78, get_joltage("234234234234278", 2));
    try std.testing.expectEqual(92, get_joltage("818181911112111", 2));

    try std.testing.expectEqual(987654321111, get_joltage("987654321111111", 12));
    try std.testing.expectEqual(811111111119, get_joltage("811111111111119", 12));
    try std.testing.expectEqual(434234234278, get_joltage("234234234234278", 12));
    try std.testing.expectEqual(888911112111, get_joltage("818181911112111", 12));
}

test "testing part 1 with test input" {
    const banks = try parse_input(test_input);

    const result = solve_part_1(banks);
    try std.testing.expectEqual(357, result);
}

test "testing part 2 with test input" {
    const banks = try parse_input(test_input);

    const result = solve_part_2(banks);
    try std.testing.expectEqual(3121910778619, result);
}
