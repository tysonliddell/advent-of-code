const math = @import("std").math;

pub const Vec3 = struct {
    x: f64,
    y: f64,
    z: f64,

    const Self = @This();
    pub fn distance(self: Self, other: Self) f64 {
        return math.sqrt(math.pow(f64, self.x - other.x, 2) +
            math.pow(f64, self.y - other.y, 2) +
            math.pow(f64, self.z - other.z, 2));
    }

    pub fn eq(self: Self, other: Self) bool {
        return self.x == other.x and self.y == other.y and self.z == other.z;
    }
};
