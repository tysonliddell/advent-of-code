const std = @import("std");

const BUFFER_SIZE = 20;

pub fn main() !void {
    const input = @embedFile("./puzzle_input/d2");
    const ranges = try parse_input(input);

    const p1 = solve_part_1(ranges);
    std.debug.print("{}\n", .{p1});

    const p2 = solve_part_2(ranges);
    std.debug.print("{}\n", .{p2});
}

fn is_repeated_once_num(val: u64) bool {
    var buf: [BUFFER_SIZE]u8 = undefined;
    const val_string = std.fmt.bufPrint(&buf, "{}", .{val}) catch unreachable;
    if (val_string.len % 2 != 0) return false;

    const mid = val_string.len / 2;
    return std.mem.eql(u8, val_string[0..mid], val_string[mid..]);
}

fn is_repeated_num(val: u64) bool {
    var buf: [BUFFER_SIZE]u8 = undefined;
    const val_string = std.fmt.bufPrint(&buf, "{}", .{val}) catch unreachable;
    const mid = val_string.len / 2;
    var length: usize = 1;
    while (length <= mid) : (length += 1) {
        const pattern = val_string[0..length];
        var rest = val_string[length..];
        while (std.mem.startsWith(u8, rest, pattern)) {
            rest = rest[length..];
        }
        if (rest.len == 0) return true;
    }

    return false;
}

fn solve_part_1(ranges: [][2]u64) u64 {
    var sum: u64 = 0;
    for (ranges) |range| {
        var curr = range[0];
        const end = range[1];
        while (curr <= end) : (curr += 1) {
            if (is_repeated_once_num(curr)) sum += curr;
        }
    }
    return sum;
}

fn solve_part_2(ranges: [][2]u64) u64 {
    var sum: u64 = 0;
    for (ranges) |range| {
        var curr = range[0];
        const end = range[1];
        while (curr <= end) : (curr += 1) {
            if (is_repeated_num(curr)) sum += curr;
        }
    }
    return sum;
}

fn parse_input(input: []const u8) ![][2]u64 {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    var ranges = try std.ArrayList([2]u64).initCapacity(allocator, 100);

    const trimmed_input = std.mem.trim(u8, input, &[_]u8{'\n'});
    var pair_it = std.mem.splitScalar(u8, trimmed_input, ',');
    while (pair_it.next()) |pair| {
        var num_it = std.mem.splitScalar(u8, pair, '-');
        const first = try std.fmt.parseInt(u64, num_it.first(), 10);
        const second = try std.fmt.parseInt(u64, num_it.rest(), 10);

        try ranges.append(allocator, [_]u64{ first, second });
    }

    return ranges.toOwnedSlice(allocator);
}

const test_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224," ++
    "1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827," ++
    "2121212118-2121212124";

test "testing is_repeated_once_num" {
    try std.testing.expectEqual(false, is_repeated_once_num(1));
    try std.testing.expectEqual(true, is_repeated_once_num(11));
    try std.testing.expectEqual(true, is_repeated_once_num(22));
    try std.testing.expectEqual(true, is_repeated_once_num(33113311));
    try std.testing.expectEqual(false, is_repeated_once_num(33113312));
}

test "testing is_repeated_num" {
    try std.testing.expectEqual(false, is_repeated_num(1));
    try std.testing.expectEqual(true, is_repeated_num(11));
    try std.testing.expectEqual(true, is_repeated_num(22));
    try std.testing.expectEqual(true, is_repeated_num(33113311));
    try std.testing.expectEqual(false, is_repeated_num(33113312));

    try std.testing.expectEqual(false, is_repeated_num(12341235));
    try std.testing.expectEqual(false, is_repeated_num(123451234));
    try std.testing.expectEqual(true, is_repeated_num(12341234));
    try std.testing.expectEqual(true, is_repeated_num(123123123));
    try std.testing.expectEqual(true, is_repeated_num(1212121212));
    try std.testing.expectEqual(true, is_repeated_num(1111111));
}

test "testing part 1 with test input" {
    const ranges = try parse_input(test_input);
    const result = solve_part_1(ranges);
    try std.testing.expectEqual(1227775554, result);
}

test "testing part 2 with test input" {
    const ranges = try parse_input(test_input);
    const result = solve_part_2(ranges);
    try std.testing.expectEqual(4174379265, result);
}
