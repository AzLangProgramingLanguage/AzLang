const std = @import("std");
const c = @cImport({
    @cInclude("stdio.h");
});

pub fn input_alloc(allocator: std.mem.Allocator) !std.ArrayList(u8) {
    try std.fs.File.stdout().writeAll("Eded girin: ");
    var result: std.ArrayList(u8) = .empty;
    errdefer result.deinit(allocator);

    var chunk_buffer: [64]u8 = undefined;
    var stdin_reader = std.fs.File.stdin().reader(&chunk_buffer);

    while (true) {
        const chunk = stdin_reader.interface.takeDelimiter('\n') catch |err| switch (err) {
            error.StreamTooLong => {
                try result.appendSlice(allocator, stdin_reader.interface.buffered());
                stdin_reader.interface.toss(stdin_reader.interface.buffered().len);
                continue;
            },
            error.ReadFailed => return err,
        };

        if (chunk) |data| {
            try result.appendSlice(allocator, data);
            break;
        } else {
            break;
        }
    }
    return result;
}

pub fn input_fixed(stdin_buffer: []u8, size: usize) []u8 {
    var stdin_reader = std.fs.File.stdin().reader(stdin_buffer);

    const user_input = stdin_reader.interface.takeDelimiterInclusive('\n') catch |err| switch (err) {
        error.StreamTooLong => {
            std.debug.print("Xəta: {d} simvoldan artiq giris!\n", .{size});
            std.process.exit(1);
        },
        error.EndOfStream => {
            std.debug.print("Xəta: Sona çata bilmedi", .{});
            std.process.exit(1);
        },
        error.ReadFailed => {
            std.debug.print("Xəta: oxuya bilmedi!\n", .{});
            std.process.exit(1);
        },
    };
    return user_input;
}
