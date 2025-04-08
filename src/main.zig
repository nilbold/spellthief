// copyright (c) 2025 nil <nil@kobold.dev>
// SPDX-License-Identifier: MPL-2.0

const std = @import("std");
const builtin = @import("builtin");
const options = @import("options");

const render = @import("render.zig");
const entity = @import("entity/entity.zig");
const RenderState = render.RenderState;

const World = @import("world.zig").World;
const Sector = @import("world.zig").Sector;

const game = @import("game.zig");

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

    //const window_half_w = window_w / 2;
    //const window_half_h = window_h / 2;

    var render_state = try RenderState.init(allocator, window_w, window_h, scaling);
    defer render_state.deinit();

    var registry = entity.Registry.init(allocator);
    defer registry.deinit();

    var static = try entity.TestStatic.init(allocator, &registry);
    defer static.deinit();

    var moving = try entity.TestMoving.init(allocator, &registry);
    defer moving.deinit();

    var world = World.init();

    const s0 = try world.sectors.addOne();
    s0.id = 0;
    try s0.portals.append(Sector.Portal{ .s = 1, .p = 0, .x = 50, .y = 0 });
    s0.rel = .{ 0, 0 };

    world.current = s0.id;

    const s1 = try world.sectors.addOne();
    s1.id = 1;
    try s1.portals.append(Sector.Portal{ .s = 0, .p = 0, .x = -50, .y = 0 });
    s1.rel = world.relative(0);

    try game.init();

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

        game.update(&world);
        try game.render(&render_state, &world);
    }
}

inline fn err(value: bool) error{SdlError}!void {
    if (!value) {
        return error.SdlError;
    }
}
