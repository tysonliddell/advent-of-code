const math = @import("std").math;

fn as_u64(v: i64) u64 {
    return @intCast(v);
}

pub const Vec3 = struct {
    x: i32,
    y: i32,
    z: i32,

    const Self = @This();
    pub fn squared_distance(self: Self, other: Self) u64 {
        const dx = @as(i64, self.x - other.x);
        const dy = @as(i64, self.y - other.y);
        const dz = @as(i64, self.z - other.z);
        return as_u64(dx * dx) + as_u64(dy * dy) + as_u64(dz * dz);
    }

    pub fn eq(self: Self, other: Self) bool {
        return self.x == other.x and self.y == other.y and self.z == other.z;
    }
};
