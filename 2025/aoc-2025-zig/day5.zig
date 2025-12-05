const std = @import("std");

pub fn main() !void {
    const input = @embedFile("./puzzle_input/d5");

    const ranges, const ids = try parse_input(input);
    std.mem.sort(Range, ranges, {}, rangeLessThanFn);

    const p1 = solve_part_1(ranges, ids);
    std.debug.print("{}\n", .{p1});

    const p2 = solve_part_2(ranges);
    std.debug.print("{}\n", .{p2});
}

const Id = u64;
const Range = [2]Id;

fn solve_part_1(sorted_ranges: []Range, ids: []Id) usize {
    var total_fresh: usize = 0;
    for (ids) |id| {
        if (bin_search_freshness(sorted_ranges, id) != null) {
            total_fresh += 1;
        }
    }
    return total_fresh;
}

fn solve_part_2(sorted_ranges: []Range) usize {
    var done: usize = 0;
    var fresh_count: usize = 0;
    for (sorted_ranges) |range| {
        var min, const max = range;
        if (max <= done) {
            // range has already been fully counted
            continue;
        } else if (min <= done) {
            // skip the part of range already counted
            min = done + 1;
        }

        fresh_count += max - min + 1;
        done = max;
    }
    return fresh_count;
}

fn bin_search_freshness(ranges: []const Range, target: Id) ?Range {
    var slice: []const Range = ranges;

    while (slice.len > 0) {
        var mid = slice.len / 2;
        // move left while neighbouring range covers a larger extent
        while (mid > 0 and slice[mid - 1][1] > slice[mid][1]) {
            mid -= 1;
        }

        const r_min, const r_max = slice[mid];
        if (target < r_min) {
            slice = slice[0..mid];
        } else if (target <= r_max) {
            return .{ r_min, r_max };
        } else {
            slice = slice[mid + 1 ..];
        }
    }
    return null;
}

fn rangeLessThanFn(_: void, a: Range, b: Range) bool {
    return a[0] < b[0] or (a[0] == b[0] and a[1] < b[1]);
}

fn parse_range(s: []const u8) !Range {
    var it = std.mem.splitScalar(u8, s, '-');
    const first = try std.fmt.parseInt(Id, it.first(), 10);
    const second = try std.fmt.parseInt(Id, it.rest(), 10);
    return .{ first, second };
}

fn parse_input(comptime input: []const u8) !struct { []Range, []Id } {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    var ranges = try std.ArrayList(Range).initCapacity(allocator, 100);
    var ids = try std.ArrayList(Id).initCapacity(allocator, 100);

    const trimmed_input = std.mem.trim(u8, input, &[_]u8{'\n'});
    var it = std.mem.splitSequence(u8, trimmed_input, "\n\n");
    var ranges_raw = std.mem.splitScalar(u8, it.first(), '\n');
    var ids_raw = std.mem.splitScalar(u8, it.rest(), '\n');

    while (ranges_raw.next()) |line| {
        const range = try parse_range(line);
        try ranges.append(allocator, range);
    }
    while (ids_raw.next()) |line| {
        const id = try std.fmt.parseInt(Id, line, 10);
        try ids.append(allocator, id);
    }

    return .{
        try ranges.toOwnedSlice(allocator),
        try ids.toOwnedSlice(allocator),
    };
}

const test_input =
    \\3-5
    \\10-14
    \\16-20
    \\12-18
    \\
    \\1
    \\5
    \\8
    \\11
    \\17
    \\32
;

const test_input_2 =
    \\4-5
    \\4-5
    \\3-6
    \\3-5
    \\
    \\1
    \\2
    \\3
    \\4
    \\5
    \\6
    \\7
;

test "testing part 1 with test input" {
    const ranges, const ids = try parse_input(test_input);
    const result = solve_part_1(ranges, ids);
    try std.testing.expectEqual(3, result);
}

test "testing part 1 with test input 2" {
    const ranges, const ids = try parse_input(test_input_2);
    std.mem.sort(Range, ranges, {}, rangeLessThanFn);
    const result = solve_part_1(ranges, ids);
    try std.testing.expectEqual(4, result);
}

test "testing part 2 with test input 1" {
    const ranges, _ = try parse_input(test_input);
    std.mem.sort(Range, ranges, {}, rangeLessThanFn);
    const result = solve_part_2(ranges);
    try std.testing.expectEqual(14, result);
}
