const std = @import("std");
const mul = @import("./util.zig").mul;
const Vec3 = @import("./linalg.zig").Vec3;

const PUZZLE_INPUT = @embedFile("./puzzle_input/d8");

const MAX_NUM_JUNCTION_BOXES = 1000;
const MAX_NUM_CONNECTIONS = MAX_NUM_JUNCTION_BOXES * (MAX_NUM_JUNCTION_BOXES - 1);
var alloc_buffer: [200_000]u8 = undefined;

const LineIterator = std.mem.SplitIterator(u8, .scalar);

pub fn main() !void {
    var line_it = parse_input(PUZZLE_INPUT);
    const p1 = solve_part_1(&line_it, 1000, 3);
    std.debug.print("{}\n", .{p1});

    line_it.reset();
    const p2 = solve_part_2(&line_it);
    std.debug.print("{}\n", .{p2});
}

const Connection = struct {
    distance: f64,
    j1_id: usize,
    j2_id: usize,

    const Self = @This();
    pub fn lessThan(_: void, a: Self, b: Self) bool {
        return a.distance < b.distance;
    }
};

fn qLessThan(context: void, a: Connection, b: Connection) std.math.Order {
    _ = context;
    return std.math.order(a.distance, b.distance).invert();
}

fn point_from_str(s: []const u8) !Vec3 {
    var it = std.mem.splitScalar(u8, s, ',');
    const x = try std.fmt.parseFloat(f64, it.next().?);
    const y = try std.fmt.parseFloat(f64, it.next().?);
    const z = try std.fmt.parseFloat(f64, it.next().?);
    return Vec3{ .x = x, .y = y, .z = z };
}

fn solve_part_1(line_it: *LineIterator, num_connections: usize, num_circuits: usize) u64 {
    var fba = std.heap.FixedBufferAllocator.init(&alloc_buffer);
    const allocator = fba.allocator();

    var junctions = std.ArrayList(Vec3).initCapacity(allocator, MAX_NUM_JUNCTION_BOXES) catch unreachable;
    defer junctions.clearAndFree(allocator);

    while (line_it.next()) |line| {
        const point = point_from_str(line) catch unreachable;
        junctions.appendAssumeCapacity(point);
    }

    // fill piority queue with 1000 'best' connections
    var connections_max_heap = std.PriorityQueue(Connection, void, qLessThan).init(allocator, {});
    defer connections_max_heap.clearAndFree();
    connections_max_heap.ensureTotalCapacity(num_connections) catch unreachable;

    for (junctions.items, 0..) |junction1, j1_id| {
        for (junctions.items[j1_id + 1 ..], j1_id + 1..) |junction2, j2_id| {
            const distance = junction1.squared_distance(junction2);
            if (connections_max_heap.items.len == num_connections) {
                if (distance < connections_max_heap.peek().?.distance) {
                    _ = connections_max_heap.remove();
                }
            }

            if (connections_max_heap.items.len < num_connections) {
                connections_max_heap.add(Connection{
                    .distance = distance,
                    .j1_id = j1_id,
                    .j2_id = j2_id,
                }) catch unreachable;
            }
        }
    }

    // drain the queue and compute the sizes of the 3 largest circuits
    var junction_to_circuit_id = std.ArrayList(usize).initCapacity(allocator, junctions.items.len) catch unreachable;
    defer junction_to_circuit_id.clearAndFree(allocator);

    for (0..junctions.items.len) |junction_id| {
        junction_to_circuit_id.appendAssumeCapacity(junction_id);
    }

    while (connections_max_heap.removeOrNull()) |conn| {
        const circuit_id_1 = junction_to_circuit_id.items[conn.j1_id];
        const circuit_id_2 = junction_to_circuit_id.items[conn.j2_id];
        if (circuit_id_1 != circuit_id_2) {
            for (junction_to_circuit_id.items, 0..junction_to_circuit_id.items.len) |c_id, j_id| {
                if (c_id == circuit_id_2) {
                    junction_to_circuit_id.items[j_id] = circuit_id_1;
                }
            }
        }
    }

    var circuit_sizes = std.ArrayList(usize).initCapacity(allocator, junctions.items.len) catch unreachable;
    defer circuit_sizes.clearAndFree(allocator);
    circuit_sizes.appendNTimesAssumeCapacity(0, junctions.items.len);

    for (junction_to_circuit_id.items) |c_id| {
        circuit_sizes.items[c_id] += 1;
    }

    std.mem.sortUnstable(usize, circuit_sizes.items, {}, std.sort.desc(usize));
    return mul(usize, circuit_sizes.items[0..num_circuits]);
}

fn solve_part_2(line_it: *LineIterator) u64 {
    var fba = std.heap.FixedBufferAllocator.init(&alloc_buffer);
    const allocator = fba.allocator();

    var junctions = std.ArrayList(Vec3).initCapacity(allocator, MAX_NUM_JUNCTION_BOXES) catch unreachable;
    var best_connections = std.ArrayList(Connection).initCapacity(allocator, MAX_NUM_JUNCTION_BOXES) catch unreachable;
    var junction_connections = std.ArrayList(Connection).initCapacity(allocator, MAX_NUM_JUNCTION_BOXES) catch unreachable;
    var new_connections = std.ArrayList(Connection).initCapacity(allocator, MAX_NUM_JUNCTION_BOXES) catch unreachable;
    var jid_to_circuit_id: [MAX_NUM_JUNCTION_BOXES]usize = undefined;
    defer junctions.clearAndFree(allocator);
    defer best_connections.clearAndFree(allocator);
    defer junction_connections.clearAndFree(allocator);
    defer new_connections.clearAndFree(allocator);

    var pBest_connections = &best_connections;
    var pJunction_connections = &junction_connections;
    var pNew_connections = &new_connections;

    while (line_it.next()) |line| {
        const point = point_from_str(line) catch unreachable;
        junctions.appendAssumeCapacity(point);
    }

    for (junctions.items[1..], 1..) |junction_to_add, jid| {
        std.mem.swap(@TypeOf(pBest_connections), &pBest_connections, &pNew_connections);

        // get all possible new connections when adding this junction
        pJunction_connections.clearRetainingCapacity();
        for (junctions.items[0..jid], 0..) |other_point, other_jid| {
            pJunction_connections.appendAssumeCapacity(Connection{
                .distance = junction_to_add.squared_distance(other_point),
                .j1_id = jid,
                .j2_id = other_jid,
            });
        }

        std.mem.sortUnstable(Connection, pJunction_connections.items, {}, Connection.lessThan);

        // recalulate best connections
        pNew_connections.clearRetainingCapacity();
        var bc_left = pBest_connections.items[0..];
        var jc_left = pJunction_connections.items[0..];

        const jid_to_circuit_id_slice = jid_to_circuit_id[0 .. jid + 1];
        for (jid_to_circuit_id_slice, 0..) |_, i| {
            jid_to_circuit_id_slice[i] = i;
        }
        while (pNew_connections.items.len < jid) {
            const d1 = if (bc_left.len > 0) bc_left[0].distance else std.math.inf(f64);
            const d2 = if (jc_left.len > 0) jc_left[0].distance else std.math.inf(f64);
            var conn: Connection = undefined;
            if (d1 < d2) {
                conn = bc_left[0];
                bc_left = bc_left[1..];
            } else {
                conn = jc_left[0];
                jc_left = jc_left[1..];
            }
            const j1, const j2 = .{ conn.j1_id, conn.j2_id };
            if (jid_to_circuit_id_slice[j1] != jid_to_circuit_id_slice[j2]) {
                const old_cid = jid_to_circuit_id_slice[j2];
                const new_cid = jid_to_circuit_id_slice[j1];
                for (jid_to_circuit_id_slice, 0..) |c, j| {
                    if (c == old_cid) {
                        jid_to_circuit_id_slice[j] = new_cid;
                    }
                }
                pNew_connections.appendAssumeCapacity(conn);
            }
        }
    }

    const last_connection = pBest_connections.getLast();
    const p1 = junctions.items[last_connection.j1_id];
    const p2 = junctions.items[last_connection.j2_id];
    return @intFromFloat(p1.x * p2.x);
}

fn parse_input(comptime input: []const u8) LineIterator {
    const trimmed_input = std.mem.trim(u8, input, &[_]u8{'\n'});
    return std.mem.splitScalar(u8, trimmed_input, '\n');
}

const test_input =
    \\162,817,812
    \\57,618,57
    \\906,360,560
    \\592,479,940
    \\352,342,300
    \\466,668,158
    \\542,29,236
    \\431,825,988
    \\739,650,466
    \\52,470,668
    \\216,146,977
    \\819,987,18
    \\117,168,530
    \\805,96,715
    \\346,949,466
    \\970,615,88
    \\941,993,340
    \\862,61,35
    \\984,92,344
    \\425,690,689
;

test "testing part 1 with test input" {
    var line_it = parse_input(test_input);
    const result = solve_part_1(&line_it, 10, 3);
    try std.testing.expectEqual(40, result);
}

test "testing part 2 with test input" {
    var line_it = parse_input(test_input);
    const result = solve_part_2(&line_it);
    try std.testing.expectEqual(25272, result);
}
