// 程式進入點，每個 Rust 程式都從 main 函數開始執行
fn main() {
    // 不可變變數，一旦賦值後就不能再改變
    let x = 5;    
    // 可變變數，使用 mut 關鍵字宣告，可以重新賦值
    let mut y = 10; 
    y += 5;  // 對可變變數進行修改
    
    // 使用 println! 巨集來格式化輸出
    println!("x = {}, y = {}", x, y);

    // 常數，必須明確指定型別，且名稱約定使用全大寫
    const PI: f64 = 3.141592; 
    println!("PI = {}", PI);

    // 有號整數 (i32)，可以儲存正負數
    let a: i32 = -100; 
    // 無號整數 (u64)，只能儲存正整數，使用底線增加可讀性
    let b: u64 = 123_456; 
    println!("a = {}, b = {}", a, b);

    // 雙精度浮點數 (f64)，預設的浮點數型別
    let c: f64 = 3.14; 
    // 單精度浮點數 (f32)
    let d: f32 = 2.71828; 
    println!("c = {}, d = {}", c, d);

    // 布林值，只有 true 或 false 兩種可能
    let is_rust_run = true; 
    println!("is_rust_run = {}", is_rust_run);

    // 字元類型，使用單引號，可以儲存一個 Unicode 純量值
    let heart = '❤'; 
    println!("heart = {}", heart);

    // 字串字面值，不可變的引用
    let name = "Rust";
    println!("name = {}", name);
    println!("長度 = {}", name.len());

    // String 類型，可變的字串
    let mut s = String::from("hello");
    println!("s = {}", s);
    // 在字串後方附加字串切片
    s.push_str(" world");
    println!("s = {}", s);

    // 陣列，固定長度的同質集合
    let a = [1, 2, 3, 4, 5];
    // 使用 :? 除錯格式化輸出
    println!("a = {:?}", a);

    // 動態陣列 (Vector)，可以動態調整大小
    let mut v = vec![1, 2, 3, 4, 5];
    println!("v = {:?}", v);
    // 在 Vector 後方新增元素
    v.push(6);
    println!("v = {:?}", v);

    // 元組 (Tuple)，可以儲存不同類型的值
    let t: (i32, f64, char) = (1, 2.0, 'a');
    println!("t = {:?}", t);

    // 解構元組，將元組中的值分別賦值給變數
    let (x, y, z) = t;
    println!("x = {}, y = {}, z = {}", x, y, z);

    //華式轉攝氏
    let f = 98.6;
    let c = fahrenheit_to_celsius(f);
    println!("{}°F is {}°C", f, c);

    //平均值
    average();
}

// 華氏溫度轉攝氏溫度的函數
// 參數 f: f64 - 華氏溫度值
// 返回值: f64 - 轉換後的攝氏溫度值
// 公式: (華氏 - 32) × 5/9 = 攝氏
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}
    
// 計算數值向量的平均值
// 這個函數會：
// 1. 建立一個包含數值的向量
// 2. 計算所有數值的總和
// 3. 計算平均值（總和除以元素數量）
// 4. 印出計算結果
fn average() {
    // 建立一個包含整數的向量
    let nums = vec![10, 20, 30, 40];
    
    // 使用迭代器的 sum() 方法計算總和
    // 需要明確指定 sum 的型別為 i32
    let sum: i32 = nums.iter().sum();
    
    // 計算平均值，將整數轉換為 f64 以獲得浮點數結果
    // 使用 as 關鍵字進行型別轉換
    let avg = sum as f64 / nums.len() as f64;
    
    // 印出計算結果，使用中文冒號
    println!("平均值：{}", avg);
}