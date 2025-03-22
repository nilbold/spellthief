const std = @import("std");

pub fn build(b: *std.Build) void {
    const version = std.SemanticVersion{ .major = 0, .minor = 0, .patch = 0 };

    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const run_step = b.step("run", "Run executable");

    const exe = b.addExecutable(.{
        .name = "spellthief",
        .version = version,
        .root_module = b.createModule(.{
            .root_source_file = b.path("src/main.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });

    const options = b.addOptions();
    exe.root_module.addOptions("options", options);

    options.addOption(std.SemanticVersion, "version", version);

    b.installArtifact(exe);

    const run_exe = b.addRunArtifact(exe);
    run_step.dependOn(&run_exe.step);
}
