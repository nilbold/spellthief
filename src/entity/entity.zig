// copyright (c) 2025 nil <nil@kobold.dev>
// SPDX-License-Identifier: MPL-2.0

pub const EntityData = @import("meta.zig").EntityData;
pub const Registry = @import("registry.zig").Registry;

const data = @import("data.zig");

/// a generationally counted entity id
pub const Entity = packed struct { gen: u8 = 0, id: u24 };

pub const TestStatic = EntityData(.{.{ "spatial", data.Spatial }});
pub const TestMoving = EntityData(.{
    .{ "spatial", data.Spatial },
    .{ "physics", data.Physics },
});
