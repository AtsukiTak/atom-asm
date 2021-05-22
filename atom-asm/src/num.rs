pub trait NumExt {
    /// 指定された数の倍数のうち、
    /// self以上の最も小さい数を返す
    fn aligned(self, align: Self) -> Self;
}

macro_rules! impl_numext {
    ($t:ty) => {
        impl NumExt for $t {
            fn aligned(self, align: Self) -> Self {
                (self / align + (self % align > 0) as $t) * align
            }
        }
    };
}

impl_numext!(u32);
impl_numext!(u64);
