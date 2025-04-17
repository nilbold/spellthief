const std = @import("std");

pub fn build(b: *std.Build) void {
    const version = std.SemanticVersion{ .major = 0, .minor = 0, .patch = 0 };

    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "spellthief",
        .version = version,
        .root_module = b.createModule(.{
            .root_source_file = b.path("src/main.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });

    if (target.result.os.tag == .windows and optimize != .Debug) {
        exe.subsystem = .Windows;
    }

    const sdl_dep = b.dependency("sdl", .{
        .target = target,
        // lets force SDL into a release mode, the current version has some
        // issues rith .Debug and causes mouse out of window frame panics
        .optimize = .ReleaseSmall,
        .preferred_link_mode = .static,
    });
    const sdl_lib = sdl_dep.artifact("SDL3");
    exe.root_module.linkLibrary(sdl_lib);

    const options = b.addOptions();
    exe.root_module.addOptions("options", options);

    options.addOption(std.SemanticVersion, "version", version);

    b.installArtifact(exe);

    const run_exe = b.addRunArtifact(exe);
    run_exe.step.dependOn(b.getInstallStep());

    const run = b.step("run", "Run executable");
    run.dependOn(&run_exe.step);

    const test_exe = b.addTest(.{
        .name = "unit tests",
        .root_module = exe.root_module,
    });

    const tests = b.step("test", "Run unit tests");

    b.installArtifact(test_exe);
    const run_tests = b.addRunArtifact(test_exe);
    tests.dependOn(&run_tests.step);
}
