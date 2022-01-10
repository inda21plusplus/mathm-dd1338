const std = @import("std");
const mem = std.mem;
const Allocator = mem.Allocator;

const Buffer = @import("buffer.zig");

pub fn PoolAllocator(comptime Child: type) type {
    if (@sizeOf(Child) < @sizeOf(*usize)) {
        @compileError("Pointer must fit inside PoolAllocator's child type");
    }

    return struct {
        underlying: Allocator,
        last_freed: ?*align(@alignOf(Child)) usize,
        buffer: ?*Buffer,
        current_index: usize,

        const This = @This();

        pub fn init(underlying: Allocator) This {
            return This{
                .underlying = underlying,
                .last_freed = null,
                .buffer = null,
                .current_index = 0,
            };
        }

        pub fn deinit(this: *This) void {
            while (this.buffer) |buffer| {
                var prev = buffer.prev;
                buffer.deinit(this.underlying);
                this.buffer = prev;
            }
        }

        pub fn create(this: *This) error{OutOfMemory}!*Child {
            if (this.last_freed) |last_freed| {
                var prev = @intToPtr(?*align(@alignOf(Child)) usize, last_freed.*);
                var child_ptr = @ptrCast(*Child, last_freed);
                this.last_freed = prev;
                return child_ptr;
            }

            var buffer = if (this.bufferFitsChildAtEnd()) this.buffer.? else try this.createBuffer();

            var data_ptr = buffer.data().ptr;
            var start_ptr = @ptrCast([*]Child, @alignCast(@alignOf(Child), data_ptr));
            var child_ptr = &start_ptr[this.current_index];

            this.current_index += 1;

            return child_ptr;
        }

        pub fn new(this: *This, child: Child) error{OutOfMemory}!*Child {
            var ptr = this.create();
            ptr.* = child;
            return ptr;
        }

        pub fn destroy(this: *This, child_ptr: *Child) void {
            var ptr = @ptrCast(*align(@alignOf(Child)) usize, child_ptr);
            ptr.* = @ptrToInt(this.last_freed);
            this.last_freed = @ptrCast(?*align(@alignOf(Child)) usize, ptr);
        }

        fn _alloc(this: *This, len: usize, ptr_align: u29, len_align: u29, ret_addr: usize) error{OutOfMemory}![]u8 {
            _ = ret_addr;

            if (len != @sizeOf(Child) or ptr_align > @alignOf(Child) or len_align > @sizeOf(Child)) {
                unreachable; // A `PoolAllocator(Child)` allocator can only be used to allocate `Child`s
            }

            var child_ptr = try this.create();
            var byte_ptr = @ptrCast([*]u8, child_ptr);

            return byte_ptr[0..@sizeOf(Child)];
        }

        fn _free(this: *This, buf: []u8, buf_align: u29, ret_addr: usize) void {
            _ = ret_addr;
            _ = buf_align;

            var ptr = buf.ptr;
            var child_ptr = @ptrCast(*Child, @alignCast(@alignOf(Child), ptr));
            this.destroy(child_ptr);
        }

        pub fn allocator(this: *This) Allocator {
            return Allocator.init(this, _alloc, Allocator.NoResize(This).noResize, _free);
        }

        fn createBuffer(this: *This) error{OutOfMemory}!*Buffer {
            // TODO: when passing @sizeOf(Child) as len there was a segfault.
            // The allocator should suck in that case but not segfault.
            // TODO: make size of each buffer customizable by user?
            this.buffer = try Buffer.init(@sizeOf(Child) * 1024, this.buffer, this.underlying);
            this.current_index = 0;
            return this.buffer.?;
        }

        fn bufferFitsChildAtEnd(this: *This) bool {
            if (this.buffer) |buffer| {
                // NOTE: `buffer.len` might not be a multiple `@sizeOf(Child)`
                return buffer.len <= (this.current_index + 1) * @sizeOf(Child);
            } else {
                return false;
            }
        }
    };
}
