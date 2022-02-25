const std = @import("std");
const Allocator = std.mem.Allocator;
const Order = std.math.Order;
const PriorityQueue = std.PriorityQueue;

const Buffer = @import("buffer.zig").Buffer;
const SegTree = @import("seg_tree.zig").SegTree;

const FreeSlice = union {
    empty: ?*FreeSlice,
    some: []u8,
};

fn max(a: usize, b: usize) usize {
    return std.math.max(a, b);
}

fn lessThan(ctx: void, a: usize, b: usize) Order {
    _ = ctx;
    return std.math.order(a, b);
}

fn greaterThan(ctx: void, a: usize, b: usize) Order {
    _ = ctx;
    return std.math.order(b, a);
}

const LtQueue = PriorityQueue(usize, void, lessThan);
const GtQueue = PriorityQueue(usize, void, greaterThan);

const This = @This();

underlying: Allocator,
buffer: ?*Buffer(u8) = null,
free_slices: []FreeSlice = &[0]FreeSlice{},
size_queue: GtQueue,
ptr_queue: LtQueue,
empty_slices: ?*FreeSlice = null,

pub fn init(underlying: Allocator) This {
    return .{ .underlying = underlying };
}

pub fn deinit(this: *This) void {
    while (this.buffer) |buffer| {
        var prev = buffer.prev;
        buffer.deinit(this.underlying);
        this.buffer = prev;
    }
}

pub fn allocator(this: *This) Allocator {
    return Allocator.init(this, alloc, resize, free);
}

fn alloc(this: *This, len: usize, ptr_align: u29, len_align: u29, ret_addr: usize) error{OutOfMemory}![]u8 {
    _ = this;
    _ = len;
    _ = ptr_align;
    _ = len_align;
    _ = ret_addr;
    unreachable;
}

fn resize(this: *This, buf: []u8, buf_align: u29, new_len: usize, len_align: u29, ret_addr: usize) ?usize {
    _ = buf_align;
    _ = len_align;
    _ = ret_addr;

    _ = this;
    _ = buf;
    _ = new_len;
    unreachable;
}

fn free(this: *This, buf: []u8, buf_align: u29, ret_addr: usize) void {
    _ = buf_align;
    _ = ret_addr;

    _ = this;
    _ = buf;
    unreachable;
}
