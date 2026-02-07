const std = @import("std");

pub fn main() !void {
    // 1. allocate a buffer for stdin reads
    var stdin_buffer: [512]u8 = undefined;

    // 2. get a reader for stdin, backed by our stdin buffer
    var stdin_reader_wrapper = std.fs.File.stdin().reader(&stdin_buffer);
    const reader: *std.Io.Reader = &stdin_reader_wrapper.interface;

    // 3. allocate a buffer for stdout writes
    var stdout_buffer: [512]u8 = undefined;

    // 4. get a writer for stdout operations, backed by our stdout buffer
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout: *std.Io.Writer = &stdout_writer.interface;

    // 5. prompt the user
    try stdout.writeAll("Type something: ");
    try stdout.flush(); // try commenting this out, notice the "Type something:" prompt won't appear, but you'll still be able to type something and hit enter, upon which it will appear

    // 6. read lines (delimiter = '\n')
    while (reader.takeDelimiterExclusive('\n')) |line| {
        // need to toss the delimiter explicitly since we're not reading it
        reader.toss(1);
        // `line` is a slice of bytes (excluding the delimiter)
        // do whatever you want with it

        try stdout.writeAll("You typed: ");
        try stdout.print("{s}", .{line});
        try stdout.writeAll("\n...\n");
        try stdout.writeAll("Type something: ");

        try stdout.flush();
    } else |err| switch (err) {
        error.EndOfStream => {
            // reached end
            // the normal case
        },
        error.StreamTooLong => {
            // the line was longer than the internal buffer
            return err;
        },
        error.ReadFailed => {
            // the read failed
            return err;
        },
    }
}
