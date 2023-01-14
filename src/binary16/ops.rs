// TODO AVX512-FP16
cfg_if::cfg_if! {
    if #[cfg(all(feature = "use-intrinsics", any(target_arch = "arm", target_arch = "aarch64"), target_feature = "v8.2a"))] {
        use core::arch::asm;

        macro_rules! impl_arith {
            ($($op:ident as $assign:ident),+) => {
                $(
                    #[inline(always)]
                    pub fn $op (mut lhs: u16, rhs: u16) -> u16 {
                        unsafe {
                            asm!(
                                concat!("f", stringify!($op), " {0:h}, {0:h}, {1:h}"),
                                inout(vreg) lhs,
                                in(vreg) rhs,
                                options(pure, nomem, nostack)
                            );
                            return lhs
                        }
                    }
        
                    #[inline(always)]
                    pub fn $assign (lhs: &mut u16, rhs: u16) {
                        unsafe {
                            asm!(
                                concat!("f", stringify!($op), " {0:h}, {0:h}, {1:h}"),
                                inout(vreg) lhs,
                                in(vreg) rhs,
                                options(pure, nomem, nostack)
                            );
                        }
                    }
                )+
            };
        }

        macro_rules! impl_mono {
            ($($op:ident),+) => {
                $(
                    #[inline(always)]
                    pub fn $op (mut lhs: u16) -> u16 {
                        unsafe {
                            asm!(
                                concat!("f", stringify!($op), " {0:h}, {0:h}"),
                                inout(vreg) lhs,
                                options(pure, nomem, nostack)
                            );
                            return lhs
                        }
                    }
                )+
            }
        }
    } else {
        macro_rules! impl_arith {
            ($($op:ident as $assign:ident),+) => {
                $(
                    #[inline]
                    pub fn $op (lhs: u16, rhs: u16) -> u16 {
                        use core::ops::*;
                        use super::convert::*;
                        return f32_to_f16(
                            f16_to_f32(lhs).$op(f16_to_f32(rhs))
                        )
                    }

                    #[inline(always)]
                    pub fn $assign (lhs: &mut u16, rhs: u16) {
                        *lhs = $op(*lhs, rhs);
                    }
                )+
            }
        }

        macro_rules! impl_mono {
            ($($op:ident),+) => {
                $(
                    #[inline]
                    pub fn $op (lhs: u16) -> u16 {
                        use super::convert::*;
                        return f32_to_f16(f16_to_f32(lhs).$op())
                    }
                )+
            }
        }
    }
}

/*macro_rules! impl_fallback {
    ($($op:tt as $fn:ident),+) => {
        $(
            #[inline]
            pub const fn $fn (lhs: u16, rhs: u16) -> u16 {
                use super::convert::*;
                return f32_to_f16_fallback(
                    f16_to_f32_fallback(lhs)
                    $op
                    f16_to_f32_fallback(rhs)
                )
            }
        )+
    }
}*/

impl_arith! {
    add as add_assign,
    sub as sub_assign,
    mul as mul_assign,
    div as div_assign
}

impl_mono! {
    sqrt
}