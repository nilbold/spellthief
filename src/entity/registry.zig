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
    /// the dense portion of the sparse to dense sparse set
    dense: ArrayList(Entity),
    /// the sparse portion of the sparse to dense sparse set
    sparse: PagedArray(page_size, u32),
    /// contains data indices for each entity
    ///
    /// this is shared with every individual entity type
    data: PagedArray(page_size, u32),
    /// a list of entity ids to be recycled
    free: FreeList = undefined,
    free_arena: ArenaAllocator,

    const Self = @This();
    const page_size = 256;

    pub inline fn count(self: *Self) usize {
        return self.dense.items.len;
    }

    pub fn init(allocator: Allocator) Self {
        return .{
            .dense = ArrayList(Entity).init(allocator),
            .sparse = PagedArray(page_size, u32).init(allocator),
            .data = PagedArray(page_size, u32).init(allocator),
            .free = FreeList{},
            .free_arena = ArenaAllocator.init(allocator),
        };
    }

    pub fn deinit(self: *Self) void {
        self.free_arena.deinit();
        self.data.deinit();
        self.sparse.deinit();
        self.dense.deinit();
    }

    pub fn createEntity(self: *Self) !Entity {
        var ent: Entity = undefined;
        if (self.free.len == 0) {
            ent = .{ .id = @intCast(self.dense.items.len) };
        } else {
            ent = self.free.popFirst().?.data;
            ent.gen +%= 1;
        }

        try self.dense.append(ent);
        try self.sparse.insert(ent.id, @intCast(self.dense.items.len - 1));
        return ent;
    }

    /// destroys an entity, returning its data index for additional cleanup
    pub fn destroyEntity(self: *Self, ent: Entity) !?u32 {
        if (self.dense.items.len == 0) {
            return null;
        }

        const tmp = self.dense.pop().?;
        const i = self.sparse.lookup(ent.id);
        const data_index = self.data.remove(ent.id);
        _ = self.sparse.remove(ent.id);

        // if this is the case, then the item popped was the entry to remove
        if (i < self.dense.items.len) {
            self.dense.items[i] = tmp;
            try self.sparse.insert(tmp.id, i);
        }

        const node = try self.free_arena.allocator().create(FreeList.Node);
        node.data = ent;
        self.free.append(node);

        return data_index;
    }
};

test "entity registry" {
    const expect = std.testing.expect;

    var reg = try Registry.init(std.testing.allocator);
    defer reg.deinit();

    const ent1 = try reg.createEntity();
    try expect(ent1.gen == 0);
    try expect(ent1.id == 0);

    const ent2 = try reg.createEntity();
    try expect(ent2.gen == 0);
    try expect(ent2.id == 1);

    const ent3 = try reg.createEntity();
    try expect(ent3.gen == 0);
    try expect(ent3.id == 2);

    _ = try reg.destroyEntity(ent2);
    try expect(reg.data.items[1].id == 2);
    try expect(reg.sparse.items[2] == 1);

    // should be reused ent2 id
    const ent4 = try reg.createEntity();
    try expect(ent2.id == ent4.id);
    try expect(ent2.gen != ent4.gen);
}
