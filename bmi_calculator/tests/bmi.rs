#[path = "../src/bmi.rs"]
mod bmi;

#[cfg(test)]
mod tests {
    use super::bmi;
    #[test]
    fn test_caculate_bmi() {
        assert_eq!(bmi::caculate_bmi(170, 60), 20.761245674740486);
        assert_eq!(bmi::caculate_bmi(180, 70), 21.604938271604937);
        assert_eq!(bmi::caculate_bmi(160, 50), 19.531249999999996);
        assert_eq!(bmi::caculate_bmi(150, 40), 17.77777777777778);
        assert_eq!(bmi::caculate_bmi(190, 80), 22.1606648199446);
    }
}