// copyright (c) 2025 nil <nil@kobold.dev>
// SPDX-License-Identifier: MPL-2.0

pub const Registry = @import("registry").Registry;

/// a generationally counted entity id
pub const Entity = packed struct { gen: u8 = 0, id: u24 };
