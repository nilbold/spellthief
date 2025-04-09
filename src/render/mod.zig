// copyright (c) 2025 nil <nil@kobold.dev>
// SPDX-License-Identifier: MPL-2.0

pub const RenderState = @import("RenderState.zig");

test {
    const std = @import("std");

    std.testing.refAllDeclsRecursive(RenderState);
}
