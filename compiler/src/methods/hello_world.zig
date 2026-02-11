// C fonksiyonunun imzasını Zig'e tanıtıyoruz
extern fn c_hello() void;

pub fn main() void {
    // C fonksiyonunu çağırıyoruz
    c_hello();
}
