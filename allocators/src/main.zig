const std = @import("std");
const testing = std.testing;

pub const ArenaAllocator = @import("arena.zig");
pub const PoolAllocator = @import("pool.zig").PoolAllocator;

const Buffer = @import("buffer.zig");

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
    // NOTE: using set seed to make test reproducible
    var gen = std.rand.DefaultPrng.init(6969);
    var rng = gen.random();
    var pool = PoolAllocator(u64).init(testing.allocator);
    defer pool.deinit();
    const Allocation = struct {
        ptr: *u64,
        val: u64,
    };
    var allocations = std.ArrayList(Allocation).init(testing.allocator);
    defer allocations.deinit();
    var i: usize = 0;
    while (i < 3_000_000) : (i += 1) {
        if (rng.boolean() and allocations.items.len > 0) {
            var index = rng.uintLessThanBiased(usize, allocations.items.len);
            var a = allocations.orderedRemove(index);
            try testing.expect(a.val == a.ptr.*);
            pool.destroy(a.ptr);
        } else {
            var x = try pool.create();
            x.* = rng.int(u64);
            try allocations.append(.{ .ptr = x, .val = x.* });
        }
    }
}

test "buffer" {
    for ([_]usize{ 1, 4, 8, 32, 64, 1024 }) |len| {
        var buf = try Buffer.init(len, null, testing.allocator);
        try testing.expect(buf.len >= len);
        buf.deinit(testing.allocator);
    }
}
