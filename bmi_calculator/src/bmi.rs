pub fn caculate_bmi<T, U>(height: T, weight: U) -> f64
where
    T: Into<f64>,
    U: Into<f64>,
{
    let height_f64 = height.into();
    let weight_f64 = weight.into();
    weight_f64 / (height_f64 / 100.0).powi(2)
}