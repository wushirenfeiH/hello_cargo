/*
 * @Description: ***页面
 * @Date: 2023-10-07 11:54:29
 * @Author: hkr
 * @LastEditors: hkr
 */
// no_mangle: 禁用对符号(symbol)名编码，表示在Javascript中依然采用fbin函数名调用本函数。
#[no_mangle]
// extern关键字表示该函数可以在Javascript中调用。
pub extern "C" fn fibGenerator(x: i32) -> i32 {
    if x <= 1 {
        return 1;
    } else {
        return fibGenerator(x - 1) + fibGenerator(x - 2);
    }
}
