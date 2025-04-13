// copyright (c) 2025 nil <nil@kobold.dev>
// SPDX-License-Identifier: MPL-2.0

const std = @import("std");
const builtin = @import("builtin");
const options = @import("options");

const Game = @import("game/mod.zig").Game;

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

    var game = try Game.init(allocator, .{
        .width = 320,
        .height = 240,
        .scale = 3,
    });
    defer game.deinit();

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

        game.update();
        try game.render();

        // just so we don't thrash as much when vsync is disabled
        // TODO: a more elegant solution
        std.time.sleep(std.time.ns_per_ms);
    }
}

inline fn err(value: bool) error{SdlError}!void {
    if (!value) {
        return error.SdlError;
    }
}
