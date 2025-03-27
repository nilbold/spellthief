// copyright (c) 2025 nil <nil@kobold.dev>
// SPDX-License-Identifier: MPL-2.0

const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const ArenaAllocator = std.heap.ArenaAllocator;
const DoublyLinkedList = std.DoublyLinkedList;

const Entity = @import("entity.zig").Entity;
const PagedArray = @import("../util.zig").PagedArray;
const FreeList = DoublyLinkedList(Entity);

/// represents a list of all active entities
pub const Registry = struct {
    dense: ArrayList(Entity),
    sparse: PagedArray(32, u32),
    free: FreeList = undefined,
    free_arena: ArenaAllocator,

    const Self = @This();
    const invalid = std.math.maxInt(u32);

    pub inline fn count(self: *Self) usize {
        return self.dense.items.len;
    }

    pub fn init(allocator: Allocator) Self {
        return .{
            .dense = ArrayList(Entity).init(allocator),
            .sparse = PagedArray(32, u32).init(allocator),
            .free = FreeList{},
            .free_arena = ArenaAllocator.init(allocator),
        };
    }

    pub fn deinit(self: *Self) void {
        self.free_arena.deinit();
        self.sparse.deinit();
        self.dense.deinit();
    }
};

pub fn createEntity(reg: *Registry) !Entity {
    var ent: Entity = undefined;
    if (reg.free.len == 0) {
        ent = .{ .id = @intCast(reg.dense.items.len) };
    } else {
        ent = reg.free.popFirst().?.data;
        ent.gen +%= 1;
    }

    try reg.dense.append(ent);
    try reg.sparse.insert(ent.id, @intCast(reg.dense.items.len - 1));
    return ent;
}

pub fn destroyEntity(reg: *Registry, ent: Entity) !void {
    if (reg.dense.items.len == 0) {
        return;
    }

    const tmp = reg.dense.pop().?;
    const i = reg.sparse.lookup(ent.id);
    try reg.sparse.insert(ent.id, Registry.invalid);

    // if this is the case, then the item popped was the entry to remove
    if (i < reg.dense.items.len) {
        reg.dense.items[i] = tmp;
        try reg.sparse.insert(tmp.id, i);
    }

    const node = try reg.free_arena.allocator().create(FreeList.Node);
    node.data = ent;
    reg.free.append(node);
}

test "entity registry" {
    const expect = std.testing.expect;

    var reg = try Registry.init(std.testing.allocator);
    defer reg.deinit();

    const ent1 = try createEntity(&reg);
    try expect(ent1.gen == 0);
    try expect(ent1.id == 0);

    const ent2 = try createEntity(&reg);
    try expect(ent2.gen == 0);
    try expect(ent2.id == 1);

    const ent3 = try createEntity(&reg);
    try expect(ent3.gen == 0);
    try expect(ent3.id == 2);

    try destroyEntity(&reg, ent2);
    try expect(reg.data.items[1].id == 2);
    try expect(reg.sparse.items[2] == 1);

    // should be reused ent2 id
    const ent4 = try createEntity(&reg);
    try expect(ent2.id == ent4.id);
    try expect(ent2.gen != ent4.gen);
}
