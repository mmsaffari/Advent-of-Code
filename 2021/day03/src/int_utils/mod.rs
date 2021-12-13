pub trait Utils {
    fn get_nth_bit(&self, n: usize) -> i16;
}

impl Utils for i16 {
    fn get_nth_bit(&self, n: usize) -> i16 {
        let bit: i16 = 1 << n;
        return (self & bit) / bit;
    }
}

impl Utils for &i16 {
    fn get_nth_bit(&self, n: usize) -> i16 {
        let bit: i16 = 1 << n;
        return (*self & bit) / bit;
    }
}