// copyright (c) 2025 nil <nil@kobold.dev>
// SPDX-License-Identifier: MPL-2.0

const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

const c = @cImport({
    @cDefine("SDL_DISABLE_OLD_NAMES", {});
    @cInclude("SDL3/SDL.h");
    @cDefine("SDL_MAIN_HANDLED", {});
    @cInclude("SDL3/SDL_main.h");
});

/// RenderState provides SDL initialization and manages the underlaying window
/// and renderer.
pub const RenderState = struct {
    window: *c.SDL_Window = undefined,
    renderer: *c.SDL_Renderer = undefined,

    // just some rects for testing
    rects: ArrayList(c.SDL_Rect),

    pub fn init(allocator: Allocator, window_w: i32, window_h: i32) !RenderState {
        c.SDL_SetMainReady();
        try err(c.SDL_SetAppMetadata("spellthief", "0.0.0", "spellthief"));

        try err(c.SDL_Init(c.SDL_INIT_VIDEO | c.SDL_INIT_AUDIO | c.SDL_INIT_GAMEPAD));

        try err(c.SDL_SetHint(c.SDL_HINT_RENDER_VSYNC, "1"));

        var window: ?*c.SDL_Window = null;
        var renderer: ?*c.SDL_Renderer = null;
        try err(c.SDL_CreateWindowAndRenderer("spellthief", window_w, window_h, 0, &window, &renderer));

        return .{
            .window = window.?,
            .renderer = renderer.?,
            .rects = ArrayList(c.SDL_Rect).init(allocator),
        };
    }

    pub fn deinit(self: *const RenderState) void {
        c.SDL_DestroyRenderer(self.renderer);
        c.SDL_DestroyWindow(self.window);
        c.SDL_Quit();
    }
};

/// clears and renders the current render state to the screen.
pub fn draw(rs: *const RenderState) !void {
    try err(c.SDL_SetRenderDrawColor(rs.renderer, 0x11, 0x22, 0x33, 0xff));
    try err(c.SDL_RenderClear(rs.renderer));
    try err(c.SDL_RenderPresent(rs.renderer));
}

inline fn err(value: bool) error{SdlError}!void {
    if (!value) {
        return error.SdlError;
    }
}
