const std = @import("std");

const PUZZLE_INPUT = @embedFile("./puzzle_input/d9");
const MAX_NUM_POSITIONS = 500;

const Position = struct { x: u32, y: u32 };

var cache: *std.AutoHashMap(Position, bool) = undefined;

pub fn main() !void {
    var position_buf: [MAX_NUM_POSITIONS]Position = undefined;
    const positions = try parse_input(PUZZLE_INPUT, &position_buf);
    const p1 = solve_part_1(positions);
    std.debug.print("{}\n", .{p1});
    const p2 = solve_part_2(positions);
    std.debug.print("{}\n", .{p2});
}

fn area(pos1: Position, pos2: Position) u64 {
    const dx: u64 = @abs(@as(i64, @intCast(pos1.x)) - @as(i64, @intCast(pos2.x))) + 1;
    const dy: u64 = @abs(@as(i64, @intCast(pos1.y)) - @as(i64, @intCast(pos2.y))) + 1;
    return dx * dy;
}

fn solve_part_1(positions: []Position) u64 {
    var max_area: u64 = 0;
    for (positions, 0..) |pos1, pos_id| {
        for (positions[pos_id + 1 ..]) |pos2| {
            const curr_area = area(pos1, pos2);
            if (curr_area > max_area) {
                max_area = curr_area;
            }
        }
    }
    return max_area;
}

fn solve_part_2(positions: []Position) u64 {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    var map = std.AutoHashMap(Position, bool).init(allocator);
    cache = &map;
    defer cache.deinit();

    var max_area: u64 = 0;
    var num_to_check = (positions.len * (positions.len - 1)) / 2;
    for (positions, 0..) |pos1, pos_id| {
        for (positions[pos_id + 1 ..]) |pos2| {
            const curr_area = area(pos1, pos2);
            if (curr_area > max_area and is_rectangle_inside(pos1, pos2, positions)) {
                max_area = curr_area;
            }
            num_to_check -= 1;
            if (num_to_check & 0xFF == 0xFF) {
                std.debug.print("Number of rectangles to check: {}\n", .{num_to_check});
            }
        }
    }
    return max_area;
}

fn is_rectangle_inside(pos1: Position, pos2: Position, red_tiles: []Position) bool {
    const min_x, const max_x = if (pos1.x < pos2.x) .{ pos1.x, pos2.x } else .{ pos2.x, pos1.x };
    const min_y, const max_y = if (pos1.y < pos2.y) .{ pos1.y, pos2.y } else .{ pos2.y, pos1.y };

    // check top and bottom rows of rectangle
    for (min_x..max_x + 1) |x| {
        if (!is_position_inside(.{ .x = @intCast(x), .y = min_y }, red_tiles)) {
            return false;
        }
        if (!is_position_inside(.{ .x = @intCast(x), .y = max_y }, red_tiles)) {
            return false;
        }
    }

    // check left and right columns of rectangle
    for (min_y..max_y + 1) |y| {
        if (!is_position_inside(.{ .x = min_x, .y = @intCast(y) }, red_tiles)) {
            return false;
        }
        if (!is_position_inside(.{ .x = max_x, .y = @intCast(y) }, red_tiles)) {
            return false;
        }
    }

    // border of rectangle lies inside loop, so its interior does too!
    return true;
}

//
// ###########
// #  p1     #
// ######### #
//    p2   # #
// ######### #
// #  p3     #
// ###########
// Since the curve forms a circuit, there must be an even total number of
// horizontal lines. A given non-edge point lies in the interior if and only if
// there are an odd number of horizontal edges above/below it.
// p1,p3 above are interior points, and p2 is an exterior point.
fn is_position_inside(pos: Position, red_tiles: []Position) bool {
    if (cache.get(pos)) |is_inside| {
        return is_inside;
    }

    var num_horiz_edge_above: usize = 0;
    const is_inside: bool = for (red_tiles[0 .. red_tiles.len - 1], red_tiles[1..]) |tile1, tile2| {
        if (tile1.y != tile2.y) {
            // this is a vertical edge
            std.debug.assert(tile1.x == tile2.x);
            const x = tile1.x;
            const min, const max = if (tile1.y < tile2.y) .{ tile1.y, tile2.y } else .{ tile2.y, tile1.y };
            if (pos.x == x and pos.y >= min and pos.y <= max) {
                break true; // position lies in veritical edge
            } else if (pos.x == x and max < pos.y) {
                // handle this case
                //
                // ######
                //      #
                //      #######
                //
                //      . <---- point
                num_horiz_edge_above += 1; // hack to keep parity correct
            }
            continue; // skip vertical edge
        }

        // this is a horizontal edge
        const y = tile1.y;
        std.debug.assert(tile1.y == tile2.y);
        const min, const max = if (tile1.x < tile2.x) .{ tile1.x, tile2.x } else .{ tile2.x, tile1.x };
        if (y <= pos.y and pos.x >= min and pos.x <= max) {
            if (pos.y == y) {
                break true; // position lies in horizontal edge
            }
            num_horiz_edge_above += 1;
        }
    } else (num_horiz_edge_above % 2 == 1);

    cache.put(pos, is_inside) catch unreachable;
    return is_inside;
}

fn parse_input(comptime input: []const u8, big_enough_slice: []Position) ![]Position {
    const trimmed_input = std.mem.trim(u8, input, &[_]u8{'\n'});
    var line_it = std.mem.splitScalar(u8, trimmed_input, '\n');
    var result = big_enough_slice;
    result.len = 0;
    while (line_it.next()) |line| {
        var tokens = std.mem.splitScalar(u8, line, ',');
        const x = try std.fmt.parseUnsigned(u32, tokens.next().?, 10);
        const y = try std.fmt.parseUnsigned(u32, tokens.rest(), 10);
        result.len += 1;
        result[result.len - 1] = .{ .x = x, .y = y };
    }

    // add final wrap around point to make it easy to compute edges later
    result.len += 1;
    result[result.len - 1] = result[0];
    return result;
}

const test_input =
    \\7,1
    \\11,1
    \\11,7
    \\9,7
    \\9,5
    \\2,5
    \\2,3
    \\7,3
;

test "testing part 1 with test input" {
    var position_buf: [MAX_NUM_POSITIONS]Position = undefined;
    const positions = try parse_input(test_input, &position_buf);
    const result = solve_part_1(positions);
    try std.testing.expectEqual(50, result);
}

test "testing part 2 with test input" {
    var position_buf: [MAX_NUM_POSITIONS]Position = undefined;
    const positions = try parse_input(test_input, &position_buf);
    const result = solve_part_2(positions);
    try std.testing.expectEqual(24, result);
}
