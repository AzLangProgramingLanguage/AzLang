const std = @import("std");
const c = @cImport({
    @cInclude("stdio.h");
});

pub fn input(allocator: std.mem.Allocator) ![]u8 {
    var c_ptr: [*c]u8 = null; 
    const result = c.scanf("%s",&c_ptr);
    if(result==-1) return error.InputFailed;
    if(c_ptr==null) return error.OutOfMemory;
    const span = std.mem.span(c_ptr);
    const zig_slice = try allocator.dupe(u8,span);
    c.free(c_ptr);
    return zig_slice;
}
