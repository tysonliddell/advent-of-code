const math = @import("std").math;

pub const Vec3 = struct {
    x: f64,
    y: f64,
    z: f64,

    const Self = @This();
    pub fn squared_distance(self: Self, other: Self) f64 {
        const dx = self.x - other.x;
        const dy = self.y - other.y;
        const dz = self.z - other.z;
        return dx * dx + dy * dy + dz * dz;
    }

    pub fn eq(self: Self, other: Self) bool {
        return self.x == other.x and self.y == other.y and self.z == other.z;
    }
};
