mod trig;

#[cfg(test)]
mod tests {
    use trig;
    #[test]
    fn it_works() {
        assert!(trig::sin(0.0) == 0.0);
    }
}
