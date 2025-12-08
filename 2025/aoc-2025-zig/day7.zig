const std = @import("std");
const sum = @import("./util.zig").sum;

const MAX_LINE_WIDTH = 200;

const LineIterator = std.mem.SplitIterator(u8, .scalar);

pub fn main() !void {
    const input = @embedFile("./puzzle_input/d7");

    var line_it = parse_input(input);
    const p1, const p2 = solve_part_1_and_2(&line_it);
    std.debug.print("{}\n", .{p1});
    std.debug.print("{}\n", .{p2});
}

fn solve_part_1_and_2(line_it: *LineIterator) struct { usize, usize } {
    const Buffer = [MAX_LINE_WIDTH]usize;
    var buffer: Buffer = undefined;
    var timeline_counts = &buffer;
    @memset(timeline_counts, 0);

    const top_line = line_it.next().?;
    const start_pos = std.mem.indexOfScalar(u8, top_line, 'S').?;
    timeline_counts[start_pos] = 1;

    var num_splits: usize = 0;
    while (line_it.next()) |line| {
        for (line, 0..) |c, pos| {
            if (c == '^') {
                if (timeline_counts[pos] > 0) {
                    num_splits += 1;
                }
                const start_weight = timeline_counts[pos];
                timeline_counts[pos] = 0;
                timeline_counts[pos - 1] += start_weight; // TODO: Add bounds checking!
                timeline_counts[pos + 1] += start_weight; // TODO: Add bounds checking!
            }
        }
    }

    const num_timelines: usize = sum(u64, timeline_counts);

    return .{ num_splits, num_timelines };
}

fn parse_input(comptime input: []const u8) LineIterator {
    const trimmed_input = std.mem.trim(u8, input, &[_]u8{'\n'});
    return std.mem.splitScalar(u8, trimmed_input, '\n');
}

const test_input =
    \\.......S.......
    \\...............
    \\.......^.......
    \\...............
    \\......^.^......
    \\...............
    \\.....^.^.^.....
    \\...............
    \\....^.^...^....
    \\...............
    \\...^.^...^.^...
    \\...............
    \\..^...^.....^..
    \\...............
    \\.^.^.^.^.^...^.
    \\...............
;

test "testing with test input" {
    var line_it = parse_input(test_input);
    const result = solve_part_1_and_2(&line_it);
    try std.testing.expectEqual(.{ 21, 40 }, result);
}
