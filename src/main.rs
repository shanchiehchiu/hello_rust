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

    //day3 練習
    day3practice();
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

//day3 練習
fn day3practice() {
    let score = 95;
    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B");
    } else if score >= 70 {
        println!("Grade: C");
    } else if score >= 60 {
        println!("Grade: D");
    } else {
        println!("Grade: F");
    }

    match score {
        90..=100 => println!("Grade: A"),
        80..=89 => println!("Grade: B"),
        70..=79 => println!("Grade: C"),
        60..=69 => println!("Grade: D"),
        _ => println!("Grade: F"),
    }

    //•	match 是 Rust 的超強 switch，可處理範圍、條件、解構。
    // 1. 基本數字匹配
    let number = 13;
    match number {
        1 => println!("一"),
        2 | 3 | 5 | 7 | 11 => println!("小質數"),
        13..=19 => println!("青少年數字"),
        n if n % 2 == 0 => println!("偶數: {}", n),
        _ => println!("其他數字"),
    }

    // 2. 元組解構
    let point = (3, 4);
    match point {
        (0, 0) => println!("原點"),
        (x, 0) => println!("在 x 軸上，x = {}", x),
        (0, y) => println!("在 y 軸上，y = {}", y),
        (x, y) => println!("在座標 ({}, {})", x, y),
    }

    // 3. 結構體解構
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 10, y: 20 };
    match point {
        Point { x: 0, y: 0 } => println!("原點"),
        Point { x, y: 0 } => println!("在 x 軸上，x = {}", x),
        Point { x: 0, y } => println!("在 y 軸上，y = {}", y),
        Point { x, y } => println!("在座標 ({}, {})", x, y),
    }

    // 4. 枚舉匹配
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::Write(String::from("你好，世界！"));
    match msg {
        Message::Quit => println!("退出"),
        Message::Move { x, y } => println!("移動到 ({}, {})", x, y),
        Message::Write(text) => println!("文字訊息: {}", text),
        Message::ChangeColor(r, g, b) => println!("顏色變更為 RGB({}, {}, {})", r, g, b),
    }

    // 5. 使用 @ 綁定
    let x = 15;
    match x {
        n @ 1..=10 => println!("小數字: {}", n),
        n @ 11..=100 => println!("中數字: {}", n),
        n => println!("大數字: {}", n),
    }

    // 6. 模式守衛 (Pattern Guards)
    let pair = (2, -2);
    match pair {
        (x, y) if x == y => println!("兩個數相等"),
        (x, y) if x + y == 0 => println!("兩個數互為相反數"),
        (x, _) if x % 2 == 1 => println!("第一個數是奇數"),
        _ => println!("沒有匹配到任何模式"),
    }
    
    // 使用所有 Message 枚舉變體
    let messages = [
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("你好，世界！")),
        Message::ChangeColor(255, 0, 0),
    ];

    // 處理每個訊息
    for msg in messages.iter() {
        match msg {
            Message::Quit => println!("收到退出訊息"),
            Message::Move { x, y } => println!("移動到座標 ({}, {})", x, y),
            Message::Write(text) => println!("文字訊息: {}", text),
            Message::ChangeColor(r, g, b) => println!("顏色變更為 RGB({}, {}, {})", r, g, b),
        }
    }
}


