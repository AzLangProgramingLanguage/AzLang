const std = @import("std");
const builder = @import("builder");
const Allocator = std.mem.Allocator;
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

const InputError = error{
    TooLong, // input buferdən böyükdür
    WriteFailed, // stdout-a yaza bilmədi
    ReadFailed, // stdin-dən oxuya bilmədi
};
pub fn input_fixed(comptime size: usize) []u8 {
    const buf_size = @max(size, 1024);
    var stdin_buffer: [buf_size]u8 = undefined;
    var stdin_reader = std.fs.File.stdin().reader(&stdin_buffer);

    std.fs.File.stdout().writeAll("Eded girin: ") catch {
        std.debug.print("Xeta: ekrana yaza bilmedi!\n", .{});
        std.process.exit(1);
    };

    const user_input = stdin_reader.interface.takeDelimiter('\n') catch |err| switch (err) {
        error.StreamTooLong => {
            std.debug.print("Xeta: {d} simvoldan artiq giris!\n", .{size});
            std.process.exit(1);
        },
        error.ReadFailed => {
            std.debug.print("Xeta: oxuya bilmedi!\n", .{});
            std.process.exit(1);
        },
    };

    // ✅ copy et - stack slice qaytarma
    const data = user_input orelse return &[_]u8{};
    var result = stdin_buffer[0..data.len];
    _ = &result;
    return result;
}
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    var result = try input_alloc(allocator);
    // const result2 = input_fixed(1);
    // std.debug.print("{s}\n", .{result2});

    defer result.deinit(allocator);
    std.debug.print("Giris: {s}\n", .{result.items});
}
// test "simple test" {
//     const gpa = std.testing.allocator;
//     var list: std.ArrayList(i32) = .empty;
//     defer list.deinit(gpa); // Try commenting this out and see if zig detects the memory leak!
//     try list.append(gpa, 42);
//     try std.testing.expectEqual(@as(i32, 42), list.pop());
// }
//
// test "fuzz example" {
//     const Context = struct {
//         fn testOne(context: @This(), input: []const u8) anyerror!void {
//             _ = context;
//             // Try passing `--fuzz` to `zig build test` and see if it manages to fail this test case!
//             try std.testing.expect(!std.mem.eql(u8, "canyoufindme", input));
//         }
//     };
//     try std.testing.fuzz(Context{}, Context.testOne, .{});
// }
