// copyright (c) 2025 nil <nil@kobold.dev>
// SPDX-License-Identifier: MPL-2.0

const std = @import("std");
const builtin = @import("builtin");
const options = @import("options");

const render = @import("render.zig");
const entity = @import("entity/entity.zig");
const RenderState = render.RenderState;

const c = @cImport({
    @cDefine("SDL_DISABLE_OLD_NAMES", {});
    @cInclude("SDL3/SDL.h");
});

var debug_allocator: std.heap.DebugAllocator(.{}) = .init;

pub fn main() !void {
    errdefer |err| if (err == error.SdlError) std.log.err("SDL error: {s}", .{c.SDL_GetError()});

    const allocator, const is_debug = allocator: {
        break :allocator switch (builtin.mode) {
            .Debug, .ReleaseSafe => .{ debug_allocator.allocator(), true },
            .ReleaseFast, .ReleaseSmall => .{ std.heap.smp_allocator, false },
        };
    };
    defer if (is_debug) {
        if (debug_allocator.deinit() == std.heap.Check.leak) {
            std.debug.print("memory leak detected\n", .{});
        }
    };

    std.debug.print("spellthief version {}\n", .{options.version});
    std.debug.print("sdl build version {d}.{d}.{d}\n", .{
        c.SDL_MAJOR_VERSION,
        c.SDL_MINOR_VERSION,
        c.SDL_MICRO_VERSION,
    });

    const window_w = 320;
    const window_h = 240;
    const scaling = 3;

    var render_state = try RenderState.init(allocator, window_w, window_h, scaling);
    defer render_state.deinit();

    var registry = entity.Registry.init(allocator);
    defer registry.deinit();

    var test_entities = entity.Test.init(allocator, &registry);
    defer test_entities.deinit();

    const ent, const ent_data = try test_entities.create();
    ent_data.spatial.* = .{ .x = 0, .y = 0 };

    std.debug.print("entities: {}\n", .{registry.count()});
    std.debug.print("id: {} | {}\n", .{ ent.id, ent_data.spatial.* });

    try test_entities.destroy(ent);

    main_loop: while (true) {
        var event: c.SDL_Event = undefined;
        while (c.SDL_PollEvent(&event)) {
            switch (event.type) {
                c.SDL_EVENT_QUIT => {
                    break :main_loop;
                },
                else => {},
            }
        }

        std.time.sleep(10 * std.time.ns_per_ms);

        try render.rect(&render_state, 0, 0, 10, 10);
        try render.rect(&render_state, 20, 20, 10, 10);
        try render.rect(&render_state, 20, -20, 10, 10);
        try render.rect(&render_state, -20, -20, 10, 10);
        try render.rect(&render_state, -20, 20, 10, 10);

        try render.draw(&render_state);
    }
}

inline fn err(value: bool) error{SdlError}!void {
    if (!value) {
        return error.SdlError;
    }
}
