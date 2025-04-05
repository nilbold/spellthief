// copyright (c) 2025 nil <nil@kobold.dev>
// SPDX-License-Identifier: MPL-2.0

const std = @import("std");

const SectorArray = std.BoundedArray(Sector, 8);
const PortalArray = std.BoundedArray(Sector.Portal, 7);

/// a sector is a singular section of the continuous game world
pub const Sector = struct {
    /// unique sector id
    id: u16,
    /// sector relative location to current
    rel: struct { i32, i32 },
    /// portals connecting this sector to others
    portals: PortalArray = PortalArray.init(0) catch unreachable,

    pub const Portal = packed struct { s: u16, p: u16, x: i32, y: i32 };
};

/// the world (map) state, comprised of linked sectors
pub const World = struct {
    /// the current sector id
    current: u16,
    /// all loaded sectors, with index 0 always being the current
    sectors: SectorArray,

    pub fn init() World {
        return World{
            .current = 0,
            .sectors = SectorArray.init(0) catch unreachable,
        };
    }

    /// get an adjoining sectors relative position by connected portal
    pub fn relative(self: *World, p: u16) struct { i32, i32 } {
        const s0 = self.sectors.get(0);
        const p1 = s0.portals.get(p);
        const p2 = lookup: {
            for (self.sectors.slice()) |s| {
                if (s.id == p1.s) {
                    break :lookup s.portals.get(p);
                }
            }
            // will panic if a portal has no matching portal, dangerous~
            // TODO: should do something cleaner here
            unreachable;
        };

        return .{ p1.x - p2.x, p1.y - p2.y };
    }
};
