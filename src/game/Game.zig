// copyright (c) 2025 nil <nil@kobold.dev>
// SPDX-License-Identifier: MPL-2.0

const Game = @This();

const std = @import("std");
const Allocator = std.mem.Allocator;

const RenderState = @import("../render/mod.zig").RenderState;
const Pool = @import("../entity/mod.zig").Pool;
const World = @import("../world.zig").World;
const Sector = @import("../world.zig").Sector;
const Player = @import("Player.zig").Player;

ticks: Ticks,
rs: RenderState,
world: World,

player: Player,

entities: EntityPool,

const EntityPool = Pool(struct { x: i32, y: i32 });

pub fn init(allocator: Allocator, opt: GameOptions) !Game {
    const rs = try RenderState.init(allocator, opt.width, opt.height, opt.scale);

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

    var ticks = Ticks{};
    try ticks.init();

    // just for a quick sanity test
    var entities = try EntityPool.init(allocator);
    _ = try entities.create(.{ .x = -20, .y = 20 });
    _ = try entities.create(.{ .x = 20, .y = 20 });

    return .{
        .ticks = ticks,
        .rs = rs,
        .world = world,
        .player = .{ .loc = s0.id, .pos = .{ 0, 0 } },
        .entities = entities,
    };
}

pub fn deinit(self: *Game) void {
    self.entities.deinit();
    self.rs.deinit();
}

pub fn update(self: *Game) void {
    self.ticks.accumulate();

    while (self.ticks.consume()) {
        // update tick
    }
}

pub fn render(self: *Game) !void {
    for (self.world.sectors.slice()) |s| {
        try self.rs.rect(s.rel[0], s.rel[1], 20, 20);
    }

    var ent_iter = self.entities.iter();
    while (ent_iter.next()) |ent| {
        const ent_data = self.entities.storage.get(ent.id);
        try self.rs.rect(ent_data.x, ent_data.y, 8, 8);
    }

    {
        const x, const y = self.player.pos;
        try self.rs.rect(x, y, 10, 10);
    }

    try self.rs.draw();
}

const GameOptions = struct {
    width: i32,
    height: i32,
    scale: i32 = 1,
};

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
