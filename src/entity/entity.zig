// copyright (c) 2025 nil <nil@kobold.dev>
// SPDX-License-Identifier: MPL-2.0

const std = @import("std");

pub const Registry = @import("registry.zig").Registry;

/// a generationally counted entity id
pub const Entity = packed struct(u32) { gen: u8 = 0, id: u24 };

pub fn Entities(comptime T: type) type {
    return struct {
        allocator: std.mem.Allocator,
        registry: *Registry,

        data: std.MultiArrayList(T) = .{},
        dense: std.ArrayList(u32),

        pub const Slice = struct {
            index: u32,
            ents: *Entities(T),
            data_slice: std.MultiArrayList(T).Slice,

            pub fn next(s: *Slice) ?Entity {
                const registry = s.ents.registry;

                if (s.index == s.ents.dense.items.len) {
                    s.index = 0;
                    return null;
                }
                defer s.index += 1;

                return registry.entity_lookup(s.ents.dense.items[s.index]);
            }

            pub inline fn get(s: *Slice) T {
                return s.data_slice.get(s.index - 1);
            }

            pub inline fn set(s: *Slice, value: T) void {
                s.data_slice.set(s.index - 1, value);
            }
        };

        pub fn init(allocator: std.mem.Allocator, registry: *Registry) !@This() {
            return @This(){
                .allocator = allocator,
                .registry = registry,
                .dense = std.ArrayList(u32).init(allocator),
            };
        }

        pub fn deinit(self: *@This()) void {
            self.dense.deinit();
            self.data.deinit(self.allocator);
        }

        pub fn create(self: *@This(), value: T) !Entity {
            const allocator = self.allocator;

            const i: u32 = @intCast(self.dense.items.len);
            const ent = try self.registry.createEntity();
            try self.registry.insert(ent.id, i);

            try self.dense.append(ent.id);
            try self.data.append(allocator, value);

            return ent;
        }

        pub fn destroy(self: *@This(), ent: Entity) !void {
            const i = try self.registry.destroyEntity(ent).?;

            _ = self.dense.swapRemove(i);
            _ = self.data.swapRemove(i);

            if (i < self.data.items.len) {
                try self.registry.insert(ent.id, self.dense.items[i]);
            }
        }

        pub fn slice(self: *@This()) Slice {
            return Slice{
                .index = 0,
                .ents = self,
                .data_slice = self.data.slice(),
            };
        }
    };
}

pub const TestStatic = Entities(struct { x: i32, y: i32 });
pub const TestMoving = Entities(struct { x: i32, y: i32, vx: i32, vy: i32 });
