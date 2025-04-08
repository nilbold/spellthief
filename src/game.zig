// copyright (c) 2025 nil <nil@kobold.dev>
// SPDX-License-Identifier: MPL-2.0

const std = @import("std");

const render_ = @import("render.zig");
const RenderState = render_.RenderState;
const World = @import("world.zig").World;

var ticks: Ticks = Ticks{};

pub fn init() !void {
    try ticks.init();
}

pub fn update(_: *World) void {
    ticks.accumulate();

    while (ticks.consume()) {
        // update tick
    }
}

pub fn render(render_state: *RenderState, world: *World) !void {
    for (world.sectors.slice()) |s| {
        try render_.rect(render_state, s.rel[0], s.rel[1], 20, 20);
    }

    try render_.draw(render_state);
}

const Ticks = struct {
    const tickrate = 48;
    const ns_per_tick = time.ns_per_s / tickrate;

    acc: u64 = 0,
    lag: u8 = 0,
    timer: Timer = undefined,

    const time = std.time;
    const math = std.math;
    const Timer = time.Timer;

    fn init(self: *Ticks) !void {
        self.timer = try Timer.start();
    }

    fn reset(self: *Ticks) void {
        self.timer.reset();
    }

    fn consume(self: *Ticks) bool {
        if (self.acc >= Ticks.ns_per_tick) {
            self.acc -= Ticks.ns_per_tick;
            return true;
        }
        const facc: f64 = @floatFromInt(self.acc);
        self.lag = @intFromFloat(math.lerp(0.0, 255.0, facc / Ticks.ns_per_tick));
        return false;
    }

    fn accumulate(self: *Ticks) void {
        const delta: u64 = self.timer.lap();
        // TODO: could cap this for unreasonable delays between updates
        self.acc += delta;
    }
};
