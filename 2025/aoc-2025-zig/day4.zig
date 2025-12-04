const std = @import("std");

pub fn main() !void {
    const input = @embedFile("./puzzle_input/d4");

    const mut_grid_p1 = try parse_input(input);
    const p1 = solve_part_1(mut_grid_p1);
    std.debug.print("{}\n", .{p1});

    const mut_grid_p2 = try parse_input(input);
    const p2 = solve_part_2(mut_grid_p2);
    std.debug.print("{}\n", .{p2});
}

inline fn is_accessible(row: usize, col: usize, grid: [][]u8) bool {
    const height = grid.len;
    const width = grid[0].len;
    var count_rolls: u8 = 0;
    if (row > 0) {
        if (col > 0) {
            if (grid[row - 1][col - 1] == '@') {
                count_rolls += 1;
            }
        }
        if (grid[row - 1][col] == '@') {
            count_rolls += 1;
        }
        if (col < width - 1) {
            if (grid[row - 1][col + 1] == '@') {
                count_rolls += 1;
            }
        }
    }

    if (col > 0) {
        if (grid[row][col - 1] == '@') {
            count_rolls += 1;
        }
    }
    if (col < width - 1) {
        if (grid[row][col + 1] == '@') {
            count_rolls += 1;
        }
    }

    if (row < height - 1) {
        if (col > 0) {
            if (grid[row + 1][col - 1] == '@') {
                count_rolls += 1;
            }
        }
        if (grid[row + 1][col] == '@') {
            count_rolls += 1;
        }
        if (col < width - 1) {
            if (grid[row + 1][col + 1] == '@') {
                count_rolls += 1;
            }
        }
    }

    return count_rolls < 4;
}

fn print_grid(grid: [][]u8) void {
    for (grid) |line| {
        std.debug.print("{s}\n", .{line});
    }
}

fn remove_paper(grid: [][]u8) usize {
    const rows = grid.len;
    const cols = grid[0].len;

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    var positions = std.ArrayList([2]usize).initCapacity(allocator, 100) catch unreachable;

    for (0..rows) |r| {
        for (0..cols) |c| {
            if (grid[r][c] == '@' and is_accessible(r, c, grid)) {
                positions.append(allocator, .{ r, c }) catch unreachable;
            }
        }
    }

    for (positions.items) |pos| {
        const r, const c = pos;
        grid[r][c] = '.';
    }

    return positions.items.len;
}

fn solve_part_1(grid: [][]u8) usize {
    return remove_paper(grid);
}

fn solve_part_2(grid: [][]u8) usize {
    var total: usize = 0;
    while (true) {
        const num_removed = remove_paper(grid);
        if (num_removed == 0) break;
        total += num_removed;
    }
    return total;
}

fn parse_input(comptime input: []const u8) ![][]u8 {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    var grid = try std.ArrayList([]u8).initCapacity(allocator, 100);

    const trimmed_input = std.mem.trim(u8, input, &[_]u8{'\n'});
    var line_it = std.mem.splitScalar(u8, trimmed_input, '\n');
    while (line_it.next()) |line| {
        var mut_line = try std.ArrayList(u8).initCapacity(allocator, 100);
        for (line) |c| {
            try mut_line.append(allocator, c);
        }
        try grid.append(allocator, try mut_line.toOwnedSlice(allocator));
    }

    return grid.toOwnedSlice(allocator);
}

const test_input =
    \\..@@.@@@@.
    \\@@@.@.@.@@
    \\@@@@@.@.@@
    \\@.@@@@..@.
    \\@@.@@@@.@@
    \\.@@@@@@@.@
    \\.@.@.@.@@@
    \\@.@@@.@@@@
    \\.@@@@@@@@.
    \\@.@.@@@.@.
;

test "testing part 1 with test input" {
    const grid = try parse_input(test_input);
    const result = solve_part_1(grid);
    try std.testing.expectEqual(13, result);
}

test "testing part 2 with test input" {
    const grid = try parse_input(test_input);
    const result = solve_part_2(grid);
    try std.testing.expectEqual(43, result);
}
