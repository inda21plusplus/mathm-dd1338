const std = @import("std");
const testing = std.testing;

pub const ArenaAllocator = @import("arena.zig");

test "two ints" {
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

test "big allocations" {
    var arena = ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    var allocator = arena.allocator();

    const sizes = [_]usize{ 8, 4096, 23_478_894, 374_387, 2_481_274, 1_000_000_000, 1, 1 };
    var buffers: [sizes.len][]u8 = undefined;

    for (sizes) |n, i| {
        buffers[i] = try allocator.alloc(u8, n);
    }

    for (buffers) |buf, i| {
        std.mem.set(u8, buf, @intCast(u8, i));
    }

    for (buffers) |buf, i| {
        for (buf) |c| {
            try testing.expect(c == i);
        }
    }
}
