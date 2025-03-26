// copyright (c) 2025 nil <nil@kobold.dev>
// SPDX-License-Identifier: MPL-2.0

const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const ArenaAllocator = std.heap.ArenaAllocator;
const DoublyLinkedList = std.DoublyLinkedList;

const Entity = @import("entity.zig").Entity;
const FreeList = DoublyLinkedList(Entity);

/// represents a list of all active entities
pub const Registry = struct {
    data: ArrayList(Entity) = undefined,
    sparse: ArrayList(u32) = undefined,
    free: FreeList = undefined,
    free_arena: ArenaAllocator,

    const invalid = std.math.maxInt(u32);

    pub fn init(allocator: Allocator) !Registry {
        return .{
            .data = try ArrayList(Entity).initCapacity(allocator, 64),
            .sparse = try ArrayList(u32).initCapacity(allocator, 64),
            .free = FreeList{},
            .free_arena = ArenaAllocator.init(allocator),
        };
    }

    pub fn deinit(reg: *Registry) void {
        reg.free_arena.deinit();
        reg.sparse.deinit();
        reg.data.deinit();
    }
};

pub fn createEntity(reg: *Registry) !Entity {
    var ent: Entity = undefined;
    if (reg.free.len == 0) {
        ent = .{ .id = @intCast(reg.data.items.len) };
    } else {
        ent = reg.free.popFirst().?.data;
        ent.gen +%= 1;
    }

    try reg.data.append(ent);
    try reg.sparse.insert(ent.id, @intCast(reg.data.items.len - 1));
    return ent;
}

pub fn destroyEntity(reg: *Registry, ent: Entity) !void {
    if (reg.data.items.len == 0) {
        return;
    }

    const index = reg.sparse.items[ent.id];
    const replace = reg.data.pop().?;

    reg.data.items[index] = replace;
    reg.sparse.items[ent.id] = Registry.invalid;
    reg.sparse.items[replace.id] = index;

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
