// copyright (c) 2025 nil <nil@kobold.dev>
// SPDX-License-Identifier: MPL-2.0

const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

/// a paginated array, allocating chunks of page_size of the array
pub fn PagedArray(comptime page_size: comptime_int, comptime T: type) type {
    return struct {
        data: ArrayList(?Page),

        const Self = @This();
        const Array = [page_size]?T;
        const Page = struct { len: usize = 0, page: *Array };

        pub fn init(allocator: Allocator) Self {
            return Self{
                .data = ArrayList(?Page).init(allocator),
            };
        }

        pub fn deinit(self: *Self) void {
            const allocator = self.data.allocator;

            for (self.data.items) |page_data| {
                if (page_data != null) {
                    allocator.free(page_data.?.page);
                }
            }

            self.data.deinit();
        }

        /// lookup associated data
        ///
        /// panics when given an invalid index
        pub fn lookup(self: *Self, i: usize) T {
            const page_n = i / page_size;
            const page_i = i % page_size;

            return self.data.items[page_n].?.page[page_i].?;
        }

        /// insert item at index i, allocating new pages as appropriate
        pub fn insert(self: *Self, i: usize, item: T) Allocator.Error!void {
            const page_n = i / page_size;
            const page_i = i % page_size;
            const allocator = self.data.allocator;

            if (page_n >= self.data.items.len) {
                for (self.data.items.len..page_n + 1) |_| {
                    try self.data.append(null);
                }
            }

            const page_data = &self.data.items[page_n];

            if (page_data.* == null) {
                page_data.* = .{ .page = try allocator.create(Array) };
                page_data.*.?.page.* = .{null} ** page_size;
            }

            page_data.*.?.page[page_i] = item;
            page_data.*.?.len += 1;
        }

        /// remove item at index i
        pub fn remove(self: *Self, i: usize) void {
            const page_n = i / page_size;
            const page_i = i % page_size;
            const allocator = self.data.allocator;

            // fully assuming the page is allocated here, at least for now
            const page_data = &self.data.items[page_n];
            const array = page_data.*.?.page;

            if (array[page_i] == null) {
                return;
            }

            array[page_i] = null;
            page_data.*.?.len -= 1;

            // should consider shrinking the number of pages

            if (page_data.*.?.len == 0) {
                allocator.destroy(page_data.*.?.page);
                page_data.* = null;
            }
        }
    };
}

test "pagination" {
    const expect = std.testing.expect;

    var paged = PagedArray(32, u32).init(std.testing.allocator);
    defer paged.deinit();

    try paged.insert(0, 10);
    try paged.insert(2, 20);

    try expect(paged.data.items.len == 1);

    const page_data1 = paged.data.items[0];
    try expect(page_data1 != null);
    try expect(page_data1.?.len == 2);
    try expect(paged.lookup(0) == 10);
    try expect(page_data1.?.page[1] == null);
    try expect(paged.lookup(2) == 20);

    try paged.insert(32, 30);

    try expect(paged.data.items.len == 2);

    const page_data2 = paged.data.items[1];
    try expect(page_data2 != null);
    try expect(page_data2.?.len == 1);
    try expect(page_data2.?.page[0].? == 30);

    const page_ptr = &paged.data.items[0];
    paged.remove(2);
    try expect(page_ptr.*.?.len == 1);

    paged.remove(0);
    try expect(page_ptr.* == null);
}
