const std = @import("std");
const mem = std.mem;
const Allocator = mem.Allocator;

const Buffer = @import("buffer.zig").Buffer;

/// A memory allocator made to allocate a lot of objects of the same type.
/// Allocation is most often O(1), but sometimes has to call the underlying
/// allocator. Freeing is always O(1). A `PoolAllocator` of a type smaller than
/// a pointer will still make each allocation take the size of a pointer.
///
/// ## Example:
/// ```
/// var pool = PoolAllocator(u1024).init(std.heap.page_allocator);
/// defer pool.deinit();
///
/// var x = try pool.create();
/// pool.destroy(x);
///
/// var allocator = pool.allocator();
///
/// var y = try allocator.create(u1024);
/// allocator.destroy(y);
///
/// // This will crash:
/// var z = allocator.create(u64);
/// ```
pub fn PoolAllocator(comptime T: type) type {
    const BufferData = union {
        taken: T,
        free: ?*@This(),
    };

    return struct {
        underlying: Allocator,
        buffer: ?*Buffer(BufferData),
        last_freed: ?*BufferData,
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

        pub fn create(this: *This) error{OutOfMemory}!*T {
            if (this.last_freed) |last_freed| {
                var prev = last_freed.free;
                last_freed.* = .{ .taken = undefined };
                var child_ptr = &last_freed.taken;
                this.last_freed = prev;
                return child_ptr;
            }

            var buffer = if (this.bufferFitsChildAtEnd()) this.buffer.? else try this.createBuffer();

            var ptr = &buffer.data()[this.current_index];
            ptr.* = .{ .taken = undefined };

            this.current_index += 1;

            return &ptr.taken;
        }

        pub fn destroy(this: *This, child_ptr: *T) void {
            // Here i'm relying on this assert to never fail and it seems like
            // it doesn't.
            //
            // var bd = BufferData{ .taken = undefined };
            // var c_ptr = &bd.taken;
            // var a = @ptrToInt(&bd);
            // var b = @ptrToInt(@ptrCast(*BufferData, @alignCast(@alignOf(BufferData), c_ptr)));
            // std.debug.assert(a == b);

            var ptr = @ptrCast(*BufferData, @alignCast(@alignOf(BufferData), child_ptr));
            ptr.* = .{ .free = this.last_freed };
            this.last_freed = ptr;
        }

        fn _alloc(
            this: *This,
            len: usize,
            ptr_align: u29,
            len_align: u29,
            ret_addr: usize,
        ) error{OutOfMemory}![]u8 {
            _ = ret_addr;

            if (len != @sizeOf(T) or ptr_align > @alignOf(T) or len_align > @sizeOf(T)) {
                unreachable; // A `PoolAllocator(T)` allocator can only be used to allocate `T`s
            }

            var child_ptr = try this.create();
            var byte_ptr = @ptrCast([*]u8, child_ptr);

            return byte_ptr[0..@sizeOf(T)];
        }

        pub fn _free(this: *This, buf: []u8, buf_align: u29, ret_addr: usize) void {
            _ = ret_addr;
            _ = buf_align;

            var ptr = buf.ptr;
            var child_ptr = @ptrCast(*T, @alignCast(@alignOf(T), ptr));
            this.destroy(child_ptr);
        }

        pub fn allocator(this: *This) Allocator {
            return Allocator.init(this, _alloc, Allocator.NoResize(This).noResize, _free);
        }

        fn createBuffer(this: *This) error{OutOfMemory}!*Buffer(BufferData) {
            this.buffer = try Buffer(BufferData).init(
                2040,
                this.buffer,
                this.underlying,
            );
            this.current_index = 0;
            return this.buffer.?;
        }

        fn bufferFitsChildAtEnd(this: *This) bool {
            if (this.buffer) |buffer| {
                return buffer.data().len < this.current_index;
            } else {
                return false;
            }
        }
    };
}
