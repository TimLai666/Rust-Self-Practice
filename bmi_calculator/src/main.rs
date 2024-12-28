mod bmi;

fn main() {
    let mut height_str = String::new();
    let mut weight_str = String::new();
    println!("輸入身高(公分)：");
    std::io::stdin().read_line(&mut height_str).expect("身高讀取失敗");
    println!("輸入體重(公斤)：");
    std::io::stdin().read_line(&mut weight_str).expect("體重讀取失敗");
    let height: f64 = height_str.trim().parse().expect("身高轉換失敗");
    let weight: f64 = weight_str.trim().parse().expect("體重轉換失敗");

    println!("你的身高是 {} 公分, 體重是 {} 公斤", height, weight);
    let bmi = bmi::caculate_bmi(height, weight);

    let message = if bmi < 18.5 {
        "體重過輕"
    } else if bmi < 24.0 {
        "正常範圍"
    } else if bmi < 27.0 {
        "過重"
    } else if bmi < 30.0 {
        "輕度肥胖"
    } else if bmi < 35.0 {
        "中度肥胖"
    } else {
        "重度肥胖"
    };

    println!("你的 BMI 是 {:.2}, {}", bmi, message);
}

