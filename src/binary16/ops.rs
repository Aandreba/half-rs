// TODO AVX512-FP16
cfg_if::cfg_if! {
    if #[cfg(all(feature = "use-intrinsics", any(target_arch = "arm", target_arch = "aarch64"), target_feature = "v8.2a"))] {
        use core::arch::asm;

        macro_rules! impl_arith {
            ($($op:ident as $fn:ident & $assign:ident),+) => {
                $(
                    #[inline(always)]
                    pub fn $fn (mut lhs: u16, rhs: u16) -> u16 {
                        unsafe {
                            asm!(
                                concat!(stringify!($op), " {0:h}, {0:h}, {1:h}"),
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
                                concat!(stringify!($op), " {0:h}, {0:h}, {1:h}"),
                                inout(vreg) lhs,
                                in(vreg) rhs,
                                options(pure, nomem, nostack)
                            );
                        }
                    }
                )+
            };
        }
    } else {
        macro_rules! impl_arith {
            ($($op:ident as $fn:ident & $assign:ident),+) => {
                $(
                    #[inline]
                    pub fn $fn (lhs: u16, rhs: u16) -> u16 {
                        return f32_to_f16(
                            f16_to_f32(lhs).$op(f16_to_f32(rhs))
                        )
                    }

                    #[inline(always)]
                    pub fn $assign (lhs: &mut u16, rhs: u16) {
                        *lhs = $fn(*lhs, rhs);
                    }
                )+
            }
        }
    }
}

macro_rules! impl_fallback {
    ($($op:tt as $fn:ident),+) => {
        $(
            #[inline]
            pub const fn $fn (lhs: u16, rhs: u16) -> u16 {
                use core::ops::*;
                use super::convert::*;
                return f32_to_f16_fallback(
                    f16_to_f32_fallback(lhs)
                    $op
                    f16_to_f32_fallback(rhs)
                )
            }
        )+
    }
}

/*impl_arith! {
    add as add_f16 & add_assign_f16,
    sub as sub_f16 & sub_assign_f16,
    mul as mul_f16 & mul_assign_f16,
    div as div_f16 & div_assign_f16
}*/

impl_fallback! {
    + as add_fallback,
    - as sub_fallback,
    * as mul_fallback,
    / as div_fallback
}