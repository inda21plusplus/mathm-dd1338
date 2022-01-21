const std = @import("std");
const testing = std.testing;
const Allocator = std.mem.Allocator;

pub const ArenaAllocator = @import("ArenaAllocator.zig");
pub const MultiPoolAllocator = @import("MultiPoolAllocator.zig");
pub const PoolAllocator = @import("pool_allocator.zig").PoolAllocator;

const Buffer = @import("buffer.zig").Buffer;

const _ = @import("seg_tree.zig");

test "arena: two ints" {
    var arena = ArenaAllocator.init(testing.allocator);
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

test "arena: big allocations" {
    var arena = ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    var allocator = arena.allocator();

    const sizes = [_]usize{ 8, 4096, 23_478_894, 374_387, 2_481_274, 1, 1 };
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

test "arena: random operations" {
    var arena = ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    inline for ([_]type{ u64, u1024, u1, u5 }) |t| {
        try testRandomOperations(arena.allocator(), t);
    }
}

test "pool: two ints" {
    var pool = PoolAllocator(u64).init(testing.allocator);
    defer pool.deinit();
    var i = try pool.create();
    var j = try pool.create();
    i.* = 420;
    j.* = 1337;
    try testing.expect(i.* == 420);
    try testing.expect(j.* == 1337);
    pool.destroy(i);
    pool.destroy(j);
}

test "pool: two ints with public interface" {
    var pool = PoolAllocator(u64).init(testing.allocator);
    defer pool.deinit();
    var allocator = pool.allocator();
    var i = try allocator.create(u64);
    var j = try allocator.create(u64);
    i.* = 420;
    j.* = 1337;
    try testing.expect(i.* == 420);
    try testing.expect(j.* == 1337);
    allocator.destroy(i);
    allocator.destroy(j);
}

test "pool: different types" {
    const types = [_]type{
        u1,    u8,       u16,                    u32,
        u64,   u128,     u256,                   u512,
        u1024, [1024]u8, [256]std.mem.Allocator,
    };
    inline for (types) |t| {
        const is_int = @typeInfo(t) == .Int;

        var pool = PoolAllocator(t).init(testing.allocator);
        defer pool.deinit();
        var x = try pool.create();
        defer pool.destroy(x);
        if (is_int) x.* = @truncate(t, 12345);
        var allocator = pool.allocator();
        var y = try allocator.create(t);
        defer allocator.destroy(y);
        if (is_int) y.* = @truncate(t, 13241);

        if (is_int) try testing.expect(x.* == @truncate(t, 12345));
        if (is_int) try testing.expect(y.* == @truncate(t, 13241));
    }
}

test "pool: many allocations" {
    var pool = PoolAllocator(u64).init(testing.allocator);
    defer pool.deinit();
    var xs = [_]*u64{undefined} ** 4321;
    for (xs) |_, i| {
        xs[i] = try pool.create();
    }
    for (xs) |x| {
        pool.destroy(x);
    }
}

test "pool: random operations" {
    inline for ([_]type{ u1, u12, u8, u64, u1024 }) |t| {
        var pool = PoolAllocator(t).init(testing.allocator);
        defer pool.deinit();
        try testRandomOperations(pool.allocator(), t);
    }
}

fn testRandomOperations(allocator: Allocator, comptime T: type) !void {
    // NOTE: using set seed to make test reproducible
    var rng = std.rand.DefaultPrng.init(694201337);
    var r = rng.random();

    const Allocation = struct { ptr: *T, val: T };
    var allocations = std.ArrayList(Allocation).init(testing.allocator);
    defer allocations.deinit();

    var i: usize = 0;
    while (i < 300_000) : (i += 1) {
        if (r.boolean() or allocations.items.len == 0) {
            var x = try allocator.create(T);
            x.* = r.int(T);
            try allocations.append(.{ .ptr = x, .val = x.* });
        } else {
            var index = r.uintLessThanBiased(usize, allocations.items.len);
            var a = allocations.orderedRemove(index);
            try testing.expect(a.val == a.ptr.*);
            allocator.destroy(a.ptr);
        }
    }
}

test "multi pool: two ints" {
    var multi_pool = MultiPoolAllocator.init(testing.allocator);
    defer multi_pool.deinit();
    var allocator = multi_pool.allocator();
    var i = try allocator.create(u29);
    defer allocator.destroy(i);
    i.* = 1234;
    var j = try allocator.create(u29);
    defer allocator.destroy(j);
    j.* = 4321;
    try testing.expect(i.* == 1234);
    try testing.expect(j.* == 4321);
}

test "multi pool: random operations" {
    inline for ([_]type{ u1, u12, u8, u64, u1024 }) |t| {
        var multi_pool = MultiPoolAllocator.init(testing.allocator);
        defer multi_pool.deinit();
        try testRandomOperations(multi_pool.allocator(), t);
    }
}

test "buffer" {
    for ([_]usize{ 1, 4, 8, 32, 64, 1024 }) |len| {
        var buf = try Buffer(u8).init(len, null, testing.allocator);
        try testing.expect(buf.data().len >= len);
        buf.deinit(testing.allocator);
    }
}
