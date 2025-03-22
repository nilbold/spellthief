// copyright (c) 2025 nil <nil@kobold.dev>
// SPDX-License-Identifier: MPL-2.0

const std = @import("std");
const options = @import("options");

pub fn main() void {
    std.debug.print("placeholder ~ spellthief {}\n", .{options.version});
}
