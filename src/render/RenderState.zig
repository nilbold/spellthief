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
const RenderState = @This();

window: *c.SDL_Window = undefined,
renderer: *c.SDL_Renderer = undefined,

window_w: i32,
window_h: i32,
scaling: f32,

// just some rects for testing
rects: ArrayList(c.SDL_FRect),

pub fn init(allocator: Allocator, window_w: i32, window_h: i32, scaling: i32) !RenderState {
    c.SDL_SetMainReady();
    try err(c.SDL_SetAppMetadata("spellthief", "0.0.0", "spellthief"));

    try err(c.SDL_Init(c.SDL_INIT_VIDEO | c.SDL_INIT_AUDIO | c.SDL_INIT_GAMEPAD));

    try err(c.SDL_SetHint(c.SDL_HINT_RENDER_VSYNC, "1"));

    var window: ?*c.SDL_Window = null;
    var renderer: ?*c.SDL_Renderer = null;
    try err(c.SDL_CreateWindowAndRenderer("spellthief", window_w * scaling, window_h * scaling, 0, &window, &renderer));

    return .{
        .window = window.?,
        .renderer = renderer.?,
        .window_w = window_w,
        .window_h = window_h,
        .scaling = @floatFromInt(scaling),
        .rects = ArrayList(c.SDL_FRect).init(allocator),
    };
}

pub fn deinit(self: *RenderState) void {
    self.rects.clearAndFree();
    c.SDL_DestroyRenderer(self.renderer);
    c.SDL_DestroyWindow(self.window);
    c.SDL_Quit();
}

/// clears and renders the current render state to the screen.
pub fn draw(rs: *RenderState) !void {
    try err(c.SDL_SetRenderDrawColor(rs.renderer, 0x11, 0x22, 0x33, c.SDL_ALPHA_OPAQUE));
    try err(c.SDL_RenderClear(rs.renderer));

    try err(c.SDL_SetRenderScale(rs.renderer, rs.scaling, rs.scaling));
    try err(c.SDL_SetRenderDrawColor(rs.renderer, 0xff, 0xff, 0xff, c.SDL_ALPHA_OPAQUE));
    try err(c.SDL_RenderRects(rs.renderer, rs.rects.items.ptr, @intCast(rs.rects.items.len)));

    try err(c.SDL_RenderPresent(rs.renderer));

    rs.rects.clearRetainingCapacity();
}

pub fn rect(rs: *RenderState, x: i32, y: i32, w: i32, h: i32) !void {
    const ox, const oy = origin(rs.window_w, rs.window_h, x, y, w, h);
    try rs.rects.append(c.SDL_FRect{ .x = @floatFromInt(ox), .y = @floatFromInt(oy), .w = @floatFromInt(w), .h = @floatFromInt(h) });
}

inline fn err(value: bool) error{SdlError}!void {
    if (!value) {
        return error.SdlError;
    }
}

inline fn origin(window_w: i32, window_h: i32, x: i32, y: i32, w: i32, h: i32) struct { i32, i32 } {
    return .{ x + @divTrunc(window_w - w, 2), -y + @divTrunc(window_h - h, 2) };
}
