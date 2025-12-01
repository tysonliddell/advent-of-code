const std = @import("std");

const NUM_DIAL_CLICKS = 100;

pub fn main() !void {
    const input = @embedFile("./puzzle_input/d1");
    const rotations = try parse_input(input);

    const p1 = solve_part_1(rotations);
    std.debug.print("{}\n", .{p1});

    const p2 = solve_part_2(rotations);
    std.debug.print("{}\n", .{p2});
}

fn solve_part_1(rotations: []i32) u32 {
    var zero_frequency: u32 = 0;
    var curr_position: u8 = 50;
    for (rotations) |rotation| {
        curr_position = std.math.comptimeMod(@as(i32, curr_position) + rotation, NUM_DIAL_CLICKS);
        if (curr_position == 0) {
            zero_frequency += 1;
        }
    }
    return zero_frequency;
}

fn solve_part_2(rotations: []i32) u64 {
    var clockwise_clicks_from_zero: u8 = 50;
    var zero_frequency: u32 = 0;
    for (rotations) |r| {
        var clicks_remaining: u32 = @abs(r);
        if (clicks_remaining == 0) continue;

        const clicks_from_zero: u8 = if (r > 0) clockwise_clicks_from_zero else NUM_DIAL_CLICKS - clockwise_clicks_from_zero;
        if (clicks_remaining >= clicks_from_zero) {
            // move to zero, if not already there
            if (clicks_from_zero > 0) {
                zero_frequency += 1;
                clicks_remaining -= clicks_from_zero;
            }

            // add the number of full revolutions remaining
            zero_frequency += clicks_remaining / NUM_DIAL_CLICKS;
        }

        clockwise_clicks_from_zero = std.math.comptimeMod(clockwise_clicks_from_zero - r, NUM_DIAL_CLICKS);
    }
    return zero_frequency;
}

fn parse_input(input: []const u8) ![]i32 {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    var rotations = try std.ArrayList(i32).initCapacity(allocator, 100);

    const trimmed_input = std.mem.trim(u8, input, &[_]u8{'\n'});
    var it = std.mem.splitSequence(u8, trimmed_input, "\n");
    while (it.next()) |line| {
        const dir: i32 = if (line[0] == 'R') 1 else -1;
        const val = try std.fmt.parseInt(i32, line[1..], 10);
        try rotations.append(allocator, val * dir);
    }

    return rotations.toOwnedSlice(allocator);
}

const test_input =
    \\L68
    \\L30
    \\R48
    \\L5
    \\R60
    \\L55
    \\L1
    \\L99
    \\R14
    \\L82
;

test "testing part 1 with test input" {
    const rotations = try parse_input(test_input);
    const result = solve_part_1(rotations);
    try std.testing.expectEqual(3, result);
}

test "testing part 2 with test input" {
    const rotations = try parse_input(test_input);
    const result = solve_part_2(rotations);
    try std.testing.expectEqual(6, result);
}
