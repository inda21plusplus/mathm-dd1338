const std = @import("std");
const testing = std.testing;

pub const ArenaAllocator = @import("arena.zig");

test "arena allocator" {
    var arena = ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    var allocator = arena.allocator();
    var i = try allocator.create(u32);
    var j = try allocator.create(u32);
    i.* = 420;
    j.* = 1337;
    try testing.expect(i.* == 420);
    try testing.expect(j.* == 1337);
    allocator.destroy(i);
    allocator.destroy(j);
}
