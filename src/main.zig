// copyright (c) 2025 nil <nil@kobold.dev>
// SPDX-License-Identifier: MPL-2.0

const std = @import("std");
const options = @import("options");

const c = @cImport({
    @cDefine("SDL_DISABLE_OLD_NAMES", {});
    @cInclude("SDL3/SDL.h");
    @cInclude("SDL3/SDL_revision.h");
    @cDefine("SDL_MAIN_HANDLED", {});
    @cInclude("SDL3/SDL_main.h");
});

pub fn main() !void {
    errdefer |err| if (err == error.SdlError) std.log.err("SDL error: {s}", .{c.SDL_GetError()});

    std.debug.print("spellthief version {}\n", .{options.version});
    std.debug.print("sdl build version {d}.{d}.{d}\n", .{
        c.SDL_MAJOR_VERSION,
        c.SDL_MINOR_VERSION,
        c.SDL_MICRO_VERSION,
    });

    c.SDL_SetMainReady();
    try err(c.SDL_SetAppMetadata("spellthief", "0.0.0", "spellthief"));

    try err(c.SDL_Init(c.SDL_INIT_VIDEO | c.SDL_INIT_AUDIO | c.SDL_INIT_GAMEPAD));
    defer c.SDL_Quit();

    const window_w = 320 * 4;
    const window_h = 240 * 4;
    try err(c.SDL_SetHint(c.SDL_HINT_RENDER_VSYNC, "1"));

    const window: *c.SDL_Window, const renderer: *c.SDL_Renderer = wr: {
        var window: ?*c.SDL_Window = null;
        var renderer: ?*c.SDL_Renderer = null;
        try err(c.SDL_CreateWindowAndRenderer("spellthief", window_w, window_h, 0, &window, &renderer));

        break :wr .{ window.?, renderer.? };
    };
    defer c.SDL_DestroyRenderer(renderer);
    defer c.SDL_DestroyWindow(window);

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

        try err(c.SDL_SetRenderDrawColor(renderer, 0x11, 0x22, 0x33, 0xff));
        try err(c.SDL_RenderClear(renderer));
        try err(c.SDL_RenderPresent(renderer));
    }
}

inline fn err(value: bool) error{SdlError}!void {
    if (!value) {
        return error.SdlError;
    }
}
