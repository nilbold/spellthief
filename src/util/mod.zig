// copyright (c) 2025 nil <nil@kobold.dev>
// SPDX-License-Identifier: MPL-2.0

pub const PagedArray = @import("paged.zig").PagedArray;
pub const FixedPoint = @import("fixed.zig").FixedPoint;

pub const i32_8 = FixedPoint(i32, 8);
pub const u32_8 = FixedPoint(u32, 8);

test {
    _ = PagedArray;
    _ = FixedPoint;
}
