use super::FizzBuzzed;

pub trait FizzBuzzable<O: FizzBuzzed<Self>>: Clone + PartialEq + PartialOrd {
    fn succ(&self) -> Self;
    fn pred(&self) -> Self;
}

impl<O: FizzBuzzed<Self>> FizzBuzzable<O> for i8 {
    fn succ(&self) -> Self {
        self + 1
    }

    fn pred(&self) -> Self {
        self - 1
    }
}

impl<O: FizzBuzzed<Self>> FizzBuzzable<O> for i16 {
    fn succ(&self) -> Self {
        self + 1
    }

    fn pred(&self) -> Self {
        self - 1
    }
}

impl<O: FizzBuzzed<Self>> FizzBuzzable<O> for i32 {
    fn succ(&self) -> Self {
        self + 1
    }

    fn pred(&self) -> Self {
        self - 1
    }
}

impl<O: FizzBuzzed<Self>> FizzBuzzable<O> for i64 {
    fn succ(&self) -> Self {
        self + 1
    }

    fn pred(&self) -> Self {
        self - 1
    }
}

impl<O: FizzBuzzed<Self>> FizzBuzzable<O> for i128 {
    fn succ(&self) -> Self {
        self + 1
    }

    fn pred(&self) -> Self {
        self - 1
    }
}

impl<O: FizzBuzzed<Self>> FizzBuzzable<O> for isize {
    fn succ(&self) -> Self {
        self + 1
    }

    fn pred(&self) -> Self {
        self - 1
    }
}

impl<O: FizzBuzzed<Self>> FizzBuzzable<O> for u8 {
    fn succ(&self) -> Self {
        self + 1
    }

    fn pred(&self) -> Self {
        self - 1
    }
}

impl<O: FizzBuzzed<Self>> FizzBuzzable<O> for u16 {
    fn succ(&self) -> Self {
        self + 1
    }

    fn pred(&self) -> Self {
        self - 1
    }
}

impl<O: FizzBuzzed<Self>> FizzBuzzable<O> for u32 {
    fn succ(&self) -> Self {
        self + 1
    }

    fn pred(&self) -> Self {
        self - 1
    }
}

impl<O: FizzBuzzed<Self>> FizzBuzzable<O> for u64 {
    fn succ(&self) -> Self {
        self + 1
    }

    fn pred(&self) -> Self {
        self - 1
    }
}

impl<O: FizzBuzzed<Self>> FizzBuzzable<O> for u128 {
    fn succ(&self) -> Self {
        self + 1
    }

    fn pred(&self) -> Self {
        self - 1
    }
}

impl<O: FizzBuzzed<Self>> FizzBuzzable<O> for usize {
    fn succ(&self) -> Self {
        self + 1
    }

    fn pred(&self) -> Self {
        self - 1
    }
}