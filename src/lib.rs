mod trig;

#[cfg(test)]
mod tests {
    use trig;
    #[test]
    fn test_sin() {
        assert!(trig::sin(0.0) == 0.0);
    }

    #[test]
    fn test_cos() {
        assert!(trig::cos(0.0) == 1.0);
    }

    #[test]
    fn test_tan() {
        assert!(trig::tan(0.0) == 0.0);
    }
}
