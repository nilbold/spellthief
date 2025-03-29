// copyright (c) 2025 nil <nil@kobold.dev>
// SPDX-License-Identifier: MPL-2.0

const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const Entity = @import("entity.zig").Entity;
const PagedArray = @import("../util.zig").PagedArray;

const Registry = @import("registry.zig").Registry;

pub fn EntityDataFields(comptime in: anytype) type {
    var fields: [in.len]std.builtin.Type.StructField = undefined;
    for (in, 0..) |t, i| {
        const field_type: type = ArrayList(t[1]);
        const field_name: [:0]const u8 = t[0][0..];
        fields[i] = .{
            .name = field_name,
            .type = field_type,
            .default_value_ptr = null,
            .is_comptime = false,
            .alignment = 0,
        };
    }
    return @Type(.{
        .@"struct" = .{
            .layout = .auto,
            .fields = fields[0..],
            .decls = &.{},
            .is_tuple = false,
        },
    });
}

pub fn EntityData(comptime in: anytype) type {
    return struct {
        data: DataFields = undefined,
        dense: ArrayList(u32),

        registry: *Registry,

        const Self = @This();
        const DataFields = EntityDataFields(in);

        pub const Accessor = acsr: {
            var fields: [in.len]std.builtin.Type.StructField = undefined;
            for (in, 0..) |t, i| {
                const field_name: [:0]const u8 = t[0][0..];
                fields[i] = .{
                    .name = field_name,
                    .type = *t[1],
                    .default_value_ptr = null,
                    .is_comptime = false,
                    .alignment = 0,
                };
            }
            break :acsr @Type(.{
                .@"struct" = .{
                    .layout = .auto,
                    .fields = fields[0..],
                    .decls = &.{},
                    .is_tuple = false,
                },
            });
        };

        pub inline fn count(self: *Self) usize {
            return self.dense.items.len;
        }

        pub fn init(allocator: Allocator, reg: *Registry) Self {
            var entity_data: EntityData(in) = .{
                .dense = ArrayList(u32).init(allocator),
                .registry = reg,
            };
            inline for (std.meta.fields(DataFields)) |f| {
                @field(entity_data.data, f.name) = @FieldType(DataFields, f.name).init(allocator);
            }

            return entity_data;
        }

        pub fn deinit(self: *Self) void {
            self.dense.deinit();
            inline for (std.meta.fields(DataFields)) |f| {
                @field(self.data, f.name).deinit();
            }
        }

        /// creates a new entity and returns a temporary accessor to its data
        ///
        /// the accessor can become invalid after additional calls to create
        pub fn create(self: *Self) !struct { Entity, Accessor } {
            const ent = try self.registry.createEntity();
            const i: u32 = @intCast(self.dense.items.len);

            try self.dense.append(i);
            try self.registry.data.insert(ent.id, i);

            var acsr: Accessor = undefined;

            inline for (std.meta.fields(DataFields)) |f| {
                try @field(self.data, f.name).append(undefined);
                @field(acsr, f.name) = &@field(self.data, f.name).items[i];
            }

            return .{ ent, acsr };
        }

        /// destroys an entity and its associated data
        pub fn destroy(self: *Self, ent: Entity) !void {
            // TODO
            // we should map entity to type to prevent destroying the wrong
            // entity through the wrong type

            const i = (try self.registry.destroyEntity(ent)).?;
            const tmp = self.dense.pop().?;

            if (i < self.dense.items.len) {
                self.dense.items[i] = tmp;
                try self.registry.data.insert(tmp, i);
            }

            inline for (std.meta.fields(DataFields)) |f| {
                _ = @field(self.data, f.name).swapRemove(i);
            }
        }
    };
}
