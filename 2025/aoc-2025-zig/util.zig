const std = @import("std");

pub inline fn int_cast(T: type, v: anytype) T {
    return @intCast(v);
}

pub fn grid_neighbours(row: usize, col: usize, height: usize, width: usize) [][2]usize {
    var neighbours: [8][2]usize = undefined;
    var neighbours_count: usize = 0;
    comptime var r_offset = -1;
    inline while (r_offset <= 1) : (r_offset += 1) {
        comptime var c_offset = -1;
        inline while (c_offset <= 1) : (c_offset += 1) {
            if (r_offset == 0 and c_offset == 0) {
                continue;
            }

            const r: i64 = int_cast(i64, row) + r_offset;
            const c: i64 = int_cast(i64, col) + c_offset;
            if (r >= 0 and r < height and c >= 0 and c < width) {
                neighbours[neighbours_count] = .{ int_cast(usize, r), int_cast(usize, c) };
                neighbours_count += 1;
            }
        }
    }
    return neighbours[0..neighbours_count];
}

pub fn sum(T: type, values: []const T) T {
    var total: T = 0;
    for (values) |v| {
        total += v;
    }
    return total;
}

pub fn mul(T: type, values: []const T) T {
    var total: T = 1;
    for (values) |v| {
        total *= v;
    }
    return total;
}
