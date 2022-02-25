//! A memory allocator that works by "splitting up" areas of memory into equally
//! large parts until a "small enough" buffer can be handed out. Can be used as
//! a "general purpose" allocator, since it can be used to allocate buffers of
//! different sizes and any allocated memory can be freed and reused.

const std = @import("std");
const mem = std.mem;
const Allocator = mem.Allocator;
const DynamicBitSetUnmanaged = std.DynamicBitSetUnmanaged;

const Buffer = @import("buffer.zig").Buffer;

const BuddyAllocator = @This();

const log2_smallest_alloc = 3;
const smallest_alloc = std.math.pow(usize, 2, log2_smallest_alloc);

underlying: Allocator,
buffer: ?[]u8,
state: DynamicBitSetUnmanaged,

pub fn init(underlying: Allocator) BuddyAllocator {
    return BuddyAllocator{
        .underlying = underlying,
        .buffer = null,
        .state = DynamicBitSetUnmanaged.initEmpty(underlying, 0) catch unreachable,
    };
}

pub fn deinit(this: *BuddyAllocator) void {
    if (this.buffer) |b| this.underlying.free(b);
    this.state.deinit(this.underlying);
}

pub fn allocator(this: *BuddyAllocator) Allocator {
    return Allocator.init(this, _alloc, _resize, _free);
}

fn getOrderFromLen(len: usize) usize {
    return @typeInfo(@TypeOf(len)).Int.bits - @clz(usize, len - 1) - log2_smallest_alloc;
}

fn _alloc(this: *BuddyAllocator, len: usize, ptr_align: u29, len_align: u29, ret_addr: usize) error{OutOfMemory}![]u8 {
    _ = ret_addr;

    var buffer = this.buffer orelse b: {
        this.buffer = try this.underlying.alloc(u8, 8096);
        break :b this.buffer;
    };
    _ = buffer;

    // Round up to nearest power of two.
    const bits = @typeInfo(@TypeOf(len)).Int.bits - @clz(usize, len - 1);
    const needed_len = @as(usize, 1) << @intCast(u6, bits);
    _ = needed_len;

    var bit = 0;
    while (true) {
        const left = 2 * bit + 1;
        const right = 2 * bit + 2;
        
    }

    _ = ptr_align;
    _ = len_align;
    unreachable;
}

fn _resize(this: *BuddyAllocator, buf: []u8, buf_align: u29, new_len: usize, len_align: u29, ret_addr: usize) ?usize {
    _ = buf_align;
    _ = len_align;
    _ = ret_addr;

    _ = this;
    _ = buf;
    _ = new_len;
    unreachable;
}

fn _free(this: *BuddyAllocator, buf: []u8, buf_align: u29, ret_addr: usize) void {
    _ = buf_align;
    _ = ret_addr;

    _ = this;
    _ = buf;
    unreachable;
}
