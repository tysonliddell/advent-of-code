const std = @import("std");
const MAX_WIDTH = 100;

const LineIterator = std.mem.SplitIterator(u8, .scalar);

pub fn main() !void {
    const input = @embedFile("./puzzle_input/d7");

    var line_it = parse_input(input);
    const p1 = solve_part_1(&line_it);
    std.debug.print("{}\n", .{p1});

    var line_it2 = parse_input(input);
    const p2 = solve_part_2(&line_it2);
    std.debug.print("{}\n", .{p2});
}

fn solve_part_1(line_it: *LineIterator) usize {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    var positions_set = std.AutoArrayHashMap(usize, bool).init(allocator);
    defer positions_set.deinit();

    var buffer: [MAX_WIDTH]usize = undefined;
    var prev_positions = std.ArrayList(usize).initBuffer(&buffer);

    const top_line = line_it.next().?;
    const start_pos = std.mem.indexOfScalar(u8, top_line, 'S').?;
    positions_set.put(start_pos, true) catch unreachable;

    var split_count: usize = 0;
    while (line_it.next()) |line| {
        prev_positions.clearRetainingCapacity();
        prev_positions.appendSliceAssumeCapacity(positions_set.keys());
        positions_set.clearRetainingCapacity();

        for (prev_positions.items) |pos| {
            if (line[pos] == '^') {
                split_count += 1;
                positions_set.put(pos - 1, true) catch unreachable; // TODO: Add bounds checking!
                positions_set.put(pos + 1, true) catch unreachable;
            } else {
                positions_set.put(pos, true) catch unreachable;
            }
        }
    }

    return split_count;
}

fn solve_part_2(line_it: *LineIterator) usize {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    var timeline_count_map = std.AutoArrayHashMap(usize, usize).init(allocator);
    defer timeline_count_map.deinit();

    var buffer: [MAX_WIDTH]struct { usize, usize } = undefined;
    var prev_positions = std.ArrayList(struct { usize, usize }).initBuffer(&buffer);

    const top_line = line_it.next().?;
    const start_pos = std.mem.indexOfScalar(u8, top_line, 'S').?;
    timeline_count_map.put(start_pos, 1) catch unreachable;

    while (line_it.next()) |line| {
        prev_positions.clearRetainingCapacity();
        while (timeline_count_map.pop()) |kv| {
            prev_positions.appendAssumeCapacity(.{ kv.key, kv.value });
        }

        for (prev_positions.items) |prev| {
            const pos, const count = prev;
            if (line[pos] == '^') {
                const l_count = timeline_count_map.get(pos - 1) orelse 0; // TODO: Add bounds checking!
                timeline_count_map.put(pos - 1, l_count + count) catch unreachable;
                const r_count = timeline_count_map.get(pos + 1) orelse 0; // TODO: Add bounds checking!
                timeline_count_map.put(pos + 1, r_count + count) catch unreachable;
            } else {
                const m_count = timeline_count_map.get(pos) orelse 0; // TODO: Add bounds checking!
                timeline_count_map.put(pos, m_count + count) catch unreachable;
            }
        }
    }

    var timeline_count: usize = 0;
    while (timeline_count_map.pop()) |kv| {
        timeline_count += kv.value;
    }

    return timeline_count;
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

test "testing part 1 with test input" {
    var line_it = parse_input(test_input);
    const result = solve_part_1(&line_it);
    try std.testing.expectEqual(21, result);
}

test "testing part 2 with test input" {
    var line_it = parse_input(test_input);
    const result = solve_part_2(&line_it);
    try std.testing.expectEqual(40, result);
}
