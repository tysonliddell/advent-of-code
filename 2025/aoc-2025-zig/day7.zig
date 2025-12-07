const std = @import("std");
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
    var buffer1: [MAX_LINE_WIDTH]usize = undefined;
    var buffer2: [MAX_LINE_WIDTH]usize = undefined;

    var timeline_counts = &buffer1;
    var prev_counts = &buffer2;
    @memset(timeline_counts, 0);

    const top_line = line_it.next().?;
    const start_pos = std.mem.indexOfScalar(u8, top_line, 'S').?;
    timeline_counts[start_pos] = 1;

    var num_splits: usize = 0;
    while (line_it.next()) |line| {
        // swap buffers
        // prev_counts, timeline_counts = .{ timeline_counts, prev_counts };
        // ^ this does not work - see https://ziglang.org/documentation/0.15.2/#Result-Locations.
        const tmp = timeline_counts;
        timeline_counts = prev_counts;
        prev_counts = tmp;
        @memset(timeline_counts, 0);

        for (line, 0..) |c, pos| {
            if (c == '^') {
                if (prev_counts[pos] > 0) {
                    num_splits += 1;
                }
                timeline_counts[pos - 1] += prev_counts[pos]; // TODO: Add bounds checking!
                timeline_counts[pos] = 0;
                timeline_counts[pos + 1] += prev_counts[pos]; // TODO: Add bounds checking!
            } else {
                timeline_counts[pos] += prev_counts[pos];
            }
        }
    }

    var num_timelines: usize = 0;
    for (timeline_counts) |count| {
        num_timelines += count;
    }

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
