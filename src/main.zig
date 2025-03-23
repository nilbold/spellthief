// copyright (c) 2025 nil <nil@kobold.dev>
// SPDX-License-Identifier: MPL-2.0

const std = @import("std");
const options = @import("options");

const render = @import("render.zig");
const RenderState = render.RenderState;

const c = @cImport({
    @cDefine("SDL_DISABLE_OLD_NAMES", {});
    @cInclude("SDL3/SDL.h");
});

var gpa = std.heap.GeneralPurposeAllocator(.{}){};

pub fn main() !void {
    errdefer |err| if (err == error.SdlError) std.log.err("SDL error: {s}", .{c.SDL_GetError()});

    const allocator = gpa.allocator();

    std.debug.print("spellthief version {}\n", .{options.version});
    std.debug.print("sdl build version {d}.{d}.{d}\n", .{
        c.SDL_MAJOR_VERSION,
        c.SDL_MINOR_VERSION,
        c.SDL_MICRO_VERSION,
    });

    const window_w = 320 * 4;
    const window_h = 240 * 4;

    const render_state = try RenderState.init(allocator, window_w, window_h);
    defer render_state.deinit();

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

        try render.draw(&render_state);
    }
}

inline fn err(value: bool) error{SdlError}!void {
    if (!value) {
        return error.SdlError;
    }
}
