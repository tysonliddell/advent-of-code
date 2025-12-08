const std = @import("std");
const sum = @import("./util.zig").sum;
const mul = @import("./util.zig").mul;

const MAX_LINES = 10;

const Grid = []const []const u8;

pub fn main() !void {
    const input = @embedFile("./puzzle_input/d6");

    const p1, const p2 = solve_part_1_and_2(input);
    std.debug.print("{}\n", .{p1});
    std.debug.print("{}\n", .{p2});
}

fn solve_part_1_and_2(comptime input: []const u8) struct { u64, u64 } {
    const trimmed_input = std.mem.trim(u8, input, &[_]u8{'\n'});
    var lines_it = std.mem.splitScalar(u8, trimmed_input, '\n');

    var lines_buffer: [MAX_LINES][]const u8 = undefined;
    var lines = std.ArrayList([]const u8).initBuffer(&lines_buffer);
    while (lines_it.next()) |line| {
        lines.appendAssumeCapacity(line);
    }

    const num_operand_rows: usize = lines.items.len - 1;
    var hv_buffer: [MAX_LINES]u64 = .{0} ** MAX_LINES;
    var horiz_values = hv_buffer[0..num_operand_rows];

    const num_cols: usize = lines.items[0].len;
    var current_verical_solution: u64 = 0;
    var curr_op: u8 = '+';
    var total_vertical_solution: u64 = 0;
    var total_horizontal_solution: u64 = 0;
    for (0..num_cols + 1) |col| {
        if (is_end_of_a_problem(lines.items, col)) {
            total_vertical_solution += current_verical_solution;

            switch (curr_op) {
                '+' => total_horizontal_solution += sum(u64, horiz_values),
                '*' => total_horizontal_solution += mul(u64, horiz_values),
                else => unreachable,
            }

            @memset(horiz_values, 0);
            continue;
        }

        const op = lines.getLast()[col];
        if (op != ' ') {
            // we've just started on the first column of a problem
            curr_op = op;
            current_verical_solution = if (op == '+') 0 else 1;
        }

        // accumulate numbers for horizontal solution
        for (lines.items[0..num_operand_rows], 0..) |line, row| {
            if (line[col] != ' ') {
                horiz_values[row] *= 10;
                horiz_values[row] += line[col] - '0';
            }
        }

        // accumulate value for vertical solution
        const vertical_num = get_vertical_val_from_col(lines.items, col);
        switch (curr_op) {
            '+' => current_verical_solution += vertical_num,
            '*' => current_verical_solution *= vertical_num,
            else => unreachable,
        }
    }

    return .{ total_horizontal_solution, total_vertical_solution };
}

fn is_end_of_a_problem(grid: Grid, col: usize) bool {
    if (col >= grid[0].len) return true;

    for (0..grid.len) |row| {
        if (grid[row][col] != ' ') {
            return false;
        }
    }
    return true;
}

fn get_vertical_val_from_col(lines: Grid, col_idx: usize) u64 {
    var val: u64 = 0;
    for (0..lines.len - 1) |row| {
        const c: u8 = lines[row][col_idx];
        if (c >= '0' and c <= '9') {
            val *= 10;
            val += lines[row][col_idx] - '0';
        }
    }
    return val;
}

const test_input =
    \\123 328  51 64 
    \\ 45 64  387 23 
    \\  6 98  215 314
    \\*   +   *   +  
;

test "testing with test input" {
    const p1, const p2 = solve_part_1_and_2(test_input);
    try std.testing.expectEqual(4277556, p1);
    try std.testing.expectEqual(3263827, p2);
}
