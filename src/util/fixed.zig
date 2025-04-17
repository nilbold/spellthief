// copyright (c) 2025 nil <nil@kobold.dev>
// SPDX-License-Identifier: MPL-2.0

const std = @import("std");

pub fn FixedPoint(comptime T: type, comptime f: comptime_int) type {
    return struct {
        pub const Shift = f;
        pub const Scaling = 1 << f;

        pub const Zero = FP{ .raw = 0 };

        const Temp = blk: {
            const int = @typeInfo(T).int;
            break :blk std.meta.Int(int.signedness, int.bits * 2);
        };

        const FP = @This();

        raw: T,

        pub fn fromInt(value: T) FP {
            return .{ .raw = value << FP.Shift };
        }

        pub fn fromFloat(value: f32) FP {
            return .{ .raw = @intFromFloat(FP.Scaling * value) };
        }

        pub fn add(lhs: FP, rhs: FP) FP {
            return .{ .raw = lhs.raw + rhs.raw };
        }

        pub fn sub(lhs: FP, rhs: FP) FP {
            return .{ .raw = lhs.raw - rhs.raw };
        }

        pub fn mul(lhs: FP, rhs: FP) FP {
            const lhs2 = @as(FP.Temp, lhs.raw);
            const rhs2 = @as(FP.Temp, rhs.raw);
            return .{ .raw = @intCast((lhs2 * rhs2) >> FP.Shift) };
        }

        pub fn mul_truncate(lhs: FP, rhs: FP) FP {
            const lhs2 = @as(FP.Temp, lhs.raw);
            const rhs2 = @as(FP.Temp, rhs.raw);
            return .{ .raw = @truncate((lhs2 * rhs2) >> FP.Shift) };
        }

        pub fn div(lhs: FP, rhs: FP) FP {
            const lhs2 = @as(FP.Temp, lhs.raw);
            const rhs2 = @as(FP.Temp, rhs.raw);
            return .{ .raw = @intCast(@divTrunc(lhs2 << FP.Shift, rhs2)) };
        }
    };
}

test "basic arithmetic" {
    const expect = std.testing.expect;
    const i32_8 = FixedPoint(i32, 8);

    const from_float = i32_8.fromFloat(1.5);
    try expect(from_float.raw & 0xff == 128);
    try expect(from_float.raw >> i32_8.Shift == 1);

    var a = i32_8.Zero;
    a = a.add(i32_8.fromInt(5));
    try expect(a.raw >> i32_8.Shift == 5);

    a = a.add(i32_8.fromFloat(0.1));
    try expect(a.raw & 0xff == 25);

    var b = i32_8.fromFloat(0.3125);
    b = b.mul(i32_8.fromInt(20));
    try expect(b.raw & 0xff == 64);

    b = b.div(i32_8.fromInt(4));
    try expect(b.raw == (1 << i32_8.Shift) + 144);
}
