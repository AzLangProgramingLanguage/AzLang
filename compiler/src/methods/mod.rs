use std::fs;

pub fn methods_string() -> String {
    // Bu makro, dosya yolunu bu .rs dosyasının konumuna göre hesaplar
    let contents = include_str!("to_string.zig");
    contents.to_string()
}
