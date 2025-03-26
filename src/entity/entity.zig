// copyright (c) 2025 nil <nil@kobold.dev>
// SPDX-License-Identifier: MPL-2.0

const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const PagedArray = @import("../util.zig").PagedArray;
const registry = @import("registry.zig");

pub const Registry = registry.Registry;

/// a generationally counted entity id
pub const Entity = packed struct { gen: u8 = 0, id: u24 };

pub fn create(reg: *Registry, impl: *const Interface) !Entity {
    const ent = try registry.createEntity(reg);
    try impl.insert(ent);
    return ent;
}

const Interface = struct {
    insertFn: *const fn (*const Interface, Entity) anyerror!void,

    pub fn insert(impl: *const Interface, ent: Entity) !void {
        try impl.insertFn(impl, ent);
    }
};

pub const Spatial = struct { x: i32, y: i32 };
pub const TestData = struct {
    spatial: ArrayList(Spatial),
    sparse: PagedArray(32, u32),

    len: usize,
    impl: Interface,

    const Self = @This();

    pub fn init(allocator: Allocator) Self {
        return .{
            .spatial = ArrayList(Spatial).init(allocator),
            .sparse = PagedArray(32, u32).init(allocator),
            .len = 0,
            .impl = Interface{ .insertFn = insert },
        };
    }

    pub fn deinit(self: *Self) void {
        self.sparse.deinit();
        self.data.deinit();
    }

    fn insert(impl: *const Interface, ent: Entity) !void {
        const self: *Self = @fieldParentPtr("impl", @constCast(impl));

        try self.sparse.insert(ent.id, @intCast(self.len));
        try self.spatial.append(.{ .x = 0, .y = 0 });
        self.len += 1;
    }
};
