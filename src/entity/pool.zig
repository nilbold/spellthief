// copyright (c) 2025 nil <nil@kobold.dev>
// SPDX-License-Identifier: MPL-2.0

const std = @import("std");

pub fn Pool(comptime T: type) type {
    const Allocator = std.mem.Allocator;
    const Order = std.math.Order;

    return struct {
        const Self = @This();

        fn lowest(context: void, a: Entity, b: Entity) Order {
            _ = context;
            return std.math.order(a.id, b.id);
        }

        const Storage = std.MultiArrayList(T);
        const Generation = std.ArrayList(u16);

        const Entity = packed struct(u32) {
            const Resource = T;
            gen: u16 = 0,
            id: u16,
        };

        allocator: Allocator,
        count: u16 = 0,
        storage: Storage = .{},
        alive: std.DynamicBitSet,
        generation: Generation,

        pub fn init(allocator: Allocator) !Self {
            return Self{
                .allocator = allocator,
                .generation = Generation.init(allocator),
                .alive = try std.DynamicBitSet.initEmpty(allocator, 32),
            };
        }

        pub fn deinit(self: *Self) void {
            self.alive.deinit();
            self.generation.deinit();
            self.storage.deinit(self.allocator);
        }

        pub fn create(self: *Self, data: T) !Entity {
            const cap = self.alive.capacity();
            if (self.count >= cap) {
                try self.alive.resize(cap * 2, false);
            }

            const index: u16 = iter: {
                var i = self.alive.iterator(.{ .kind = .unset });
                break :iter @intCast(i.next().?);
            };

            if (index > self.storage.len) {
                @panic("first free entity index exceeds storage");
            }

            if (index == self.storage.len) {
                try self.storage.append(self.allocator, data);
                try self.generation.append(0);
            } else {
                self.storage.set(index, data);
            }

            self.alive.set(index);
            self.count += 1;

            const gen = self.generation.items[index];

            return Entity{ .id = index, .gen = gen };
        }

        pub fn destroy(self: *Self, ent: Entity) !void {
            // NOTE: existing entity data is not cleaned up, only the
            // handle to it is invalidated
            self.generation.items[ent.id] +%= 1;
            self.alive.unset(ent.id);
            self.count -= 1;
        }

        pub fn valid(self: *Self, ent: Entity) bool {
            const gen = self.generation.items;
            if (ent.id >= gen.len) {
                return false;
            }
            return gen[ent.id] == ent.gen;
        }

        pub fn clear(self: *Self) void {
            const range = .{ .start = 0, .end = self.alive.capacity() };
            self.alive.setRangeValue(range, false);
            self.count = 0;
        }
    };
}
