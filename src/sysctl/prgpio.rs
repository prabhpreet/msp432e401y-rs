#[doc = "Register `PRGPIO` reader"]
pub struct R(crate::R<PRGPIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRGPIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRGPIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRGPIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRGPIO` writer"]
pub struct W(crate::W<PRGPIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRGPIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PRGPIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRGPIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PRGPIO_R0` reader - GPIO Port A Peripheral Ready"]
pub type SYSCTL_PRGPIO_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRGPIO_R0` writer - GPIO Port A Peripheral Ready"]
pub type SYSCTL_PRGPIO_R0_W<'a> = crate::BitWriter<'a, u32, PRGPIO_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_PRGPIO_R1` reader - GPIO Port B Peripheral Ready"]
pub type SYSCTL_PRGPIO_R1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRGPIO_R1` writer - GPIO Port B Peripheral Ready"]
pub type SYSCTL_PRGPIO_R1_W<'a> = crate::BitWriter<'a, u32, PRGPIO_SPEC, bool, 1>;
#[doc = "Field `SYSCTL_PRGPIO_R2` reader - GPIO Port C Peripheral Ready"]
pub type SYSCTL_PRGPIO_R2_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRGPIO_R2` writer - GPIO Port C Peripheral Ready"]
pub type SYSCTL_PRGPIO_R2_W<'a> = crate::BitWriter<'a, u32, PRGPIO_SPEC, bool, 2>;
#[doc = "Field `SYSCTL_PRGPIO_R3` reader - GPIO Port D Peripheral Ready"]
pub type SYSCTL_PRGPIO_R3_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRGPIO_R3` writer - GPIO Port D Peripheral Ready"]
pub type SYSCTL_PRGPIO_R3_W<'a> = crate::BitWriter<'a, u32, PRGPIO_SPEC, bool, 3>;
#[doc = "Field `SYSCTL_PRGPIO_R4` reader - GPIO Port E Peripheral Ready"]
pub type SYSCTL_PRGPIO_R4_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRGPIO_R4` writer - GPIO Port E Peripheral Ready"]
pub type SYSCTL_PRGPIO_R4_W<'a> = crate::BitWriter<'a, u32, PRGPIO_SPEC, bool, 4>;
#[doc = "Field `SYSCTL_PRGPIO_R5` reader - GPIO Port F Peripheral Ready"]
pub type SYSCTL_PRGPIO_R5_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRGPIO_R5` writer - GPIO Port F Peripheral Ready"]
pub type SYSCTL_PRGPIO_R5_W<'a> = crate::BitWriter<'a, u32, PRGPIO_SPEC, bool, 5>;
#[doc = "Field `SYSCTL_PRGPIO_R6` reader - GPIO Port G Peripheral Ready"]
pub type SYSCTL_PRGPIO_R6_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRGPIO_R6` writer - GPIO Port G Peripheral Ready"]
pub type SYSCTL_PRGPIO_R6_W<'a> = crate::BitWriter<'a, u32, PRGPIO_SPEC, bool, 6>;
#[doc = "Field `SYSCTL_PRGPIO_R7` reader - GPIO Port H Peripheral Ready"]
pub type SYSCTL_PRGPIO_R7_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRGPIO_R7` writer - GPIO Port H Peripheral Ready"]
pub type SYSCTL_PRGPIO_R7_W<'a> = crate::BitWriter<'a, u32, PRGPIO_SPEC, bool, 7>;
#[doc = "Field `SYSCTL_PRGPIO_R8` reader - GPIO Port J Peripheral Ready"]
pub type SYSCTL_PRGPIO_R8_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRGPIO_R8` writer - GPIO Port J Peripheral Ready"]
pub type SYSCTL_PRGPIO_R8_W<'a> = crate::BitWriter<'a, u32, PRGPIO_SPEC, bool, 8>;
#[doc = "Field `SYSCTL_PRGPIO_R9` reader - GPIO Port K Peripheral Ready"]
pub type SYSCTL_PRGPIO_R9_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRGPIO_R9` writer - GPIO Port K Peripheral Ready"]
pub type SYSCTL_PRGPIO_R9_W<'a> = crate::BitWriter<'a, u32, PRGPIO_SPEC, bool, 9>;
#[doc = "Field `SYSCTL_PRGPIO_R10` reader - GPIO Port L Peripheral Ready"]
pub type SYSCTL_PRGPIO_R10_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRGPIO_R10` writer - GPIO Port L Peripheral Ready"]
pub type SYSCTL_PRGPIO_R10_W<'a> = crate::BitWriter<'a, u32, PRGPIO_SPEC, bool, 10>;
#[doc = "Field `SYSCTL_PRGPIO_R11` reader - GPIO Port M Peripheral Ready"]
pub type SYSCTL_PRGPIO_R11_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRGPIO_R11` writer - GPIO Port M Peripheral Ready"]
pub type SYSCTL_PRGPIO_R11_W<'a> = crate::BitWriter<'a, u32, PRGPIO_SPEC, bool, 11>;
#[doc = "Field `SYSCTL_PRGPIO_R12` reader - GPIO Port N Peripheral Ready"]
pub type SYSCTL_PRGPIO_R12_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRGPIO_R12` writer - GPIO Port N Peripheral Ready"]
pub type SYSCTL_PRGPIO_R12_W<'a> = crate::BitWriter<'a, u32, PRGPIO_SPEC, bool, 12>;
#[doc = "Field `SYSCTL_PRGPIO_R13` reader - GPIO Port P Peripheral Ready"]
pub type SYSCTL_PRGPIO_R13_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRGPIO_R13` writer - GPIO Port P Peripheral Ready"]
pub type SYSCTL_PRGPIO_R13_W<'a> = crate::BitWriter<'a, u32, PRGPIO_SPEC, bool, 13>;
#[doc = "Field `SYSCTL_PRGPIO_R14` reader - GPIO Port Q Peripheral Ready"]
pub type SYSCTL_PRGPIO_R14_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRGPIO_R14` writer - GPIO Port Q Peripheral Ready"]
pub type SYSCTL_PRGPIO_R14_W<'a> = crate::BitWriter<'a, u32, PRGPIO_SPEC, bool, 14>;
impl R {
    #[doc = "Bit 0 - GPIO Port A Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r0(&self) -> SYSCTL_PRGPIO_R0_R {
        SYSCTL_PRGPIO_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO Port B Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r1(&self) -> SYSCTL_PRGPIO_R1_R {
        SYSCTL_PRGPIO_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO Port C Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r2(&self) -> SYSCTL_PRGPIO_R2_R {
        SYSCTL_PRGPIO_R2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO Port D Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r3(&self) -> SYSCTL_PRGPIO_R3_R {
        SYSCTL_PRGPIO_R3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO Port E Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r4(&self) -> SYSCTL_PRGPIO_R4_R {
        SYSCTL_PRGPIO_R4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO Port F Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r5(&self) -> SYSCTL_PRGPIO_R5_R {
        SYSCTL_PRGPIO_R5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO Port G Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r6(&self) -> SYSCTL_PRGPIO_R6_R {
        SYSCTL_PRGPIO_R6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO Port H Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r7(&self) -> SYSCTL_PRGPIO_R7_R {
        SYSCTL_PRGPIO_R7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO Port J Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r8(&self) -> SYSCTL_PRGPIO_R8_R {
        SYSCTL_PRGPIO_R8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO Port K Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r9(&self) -> SYSCTL_PRGPIO_R9_R {
        SYSCTL_PRGPIO_R9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO Port L Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r10(&self) -> SYSCTL_PRGPIO_R10_R {
        SYSCTL_PRGPIO_R10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO Port M Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r11(&self) -> SYSCTL_PRGPIO_R11_R {
        SYSCTL_PRGPIO_R11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIO Port N Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r12(&self) -> SYSCTL_PRGPIO_R12_R {
        SYSCTL_PRGPIO_R12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIO Port P Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r13(&self) -> SYSCTL_PRGPIO_R13_R {
        SYSCTL_PRGPIO_R13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO Port Q Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r14(&self) -> SYSCTL_PRGPIO_R14_R {
        SYSCTL_PRGPIO_R14_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO Port A Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r0(&mut self) -> SYSCTL_PRGPIO_R0_W {
        SYSCTL_PRGPIO_R0_W::new(self)
    }
    #[doc = "Bit 1 - GPIO Port B Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r1(&mut self) -> SYSCTL_PRGPIO_R1_W {
        SYSCTL_PRGPIO_R1_W::new(self)
    }
    #[doc = "Bit 2 - GPIO Port C Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r2(&mut self) -> SYSCTL_PRGPIO_R2_W {
        SYSCTL_PRGPIO_R2_W::new(self)
    }
    #[doc = "Bit 3 - GPIO Port D Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r3(&mut self) -> SYSCTL_PRGPIO_R3_W {
        SYSCTL_PRGPIO_R3_W::new(self)
    }
    #[doc = "Bit 4 - GPIO Port E Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r4(&mut self) -> SYSCTL_PRGPIO_R4_W {
        SYSCTL_PRGPIO_R4_W::new(self)
    }
    #[doc = "Bit 5 - GPIO Port F Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r5(&mut self) -> SYSCTL_PRGPIO_R5_W {
        SYSCTL_PRGPIO_R5_W::new(self)
    }
    #[doc = "Bit 6 - GPIO Port G Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r6(&mut self) -> SYSCTL_PRGPIO_R6_W {
        SYSCTL_PRGPIO_R6_W::new(self)
    }
    #[doc = "Bit 7 - GPIO Port H Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r7(&mut self) -> SYSCTL_PRGPIO_R7_W {
        SYSCTL_PRGPIO_R7_W::new(self)
    }
    #[doc = "Bit 8 - GPIO Port J Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r8(&mut self) -> SYSCTL_PRGPIO_R8_W {
        SYSCTL_PRGPIO_R8_W::new(self)
    }
    #[doc = "Bit 9 - GPIO Port K Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r9(&mut self) -> SYSCTL_PRGPIO_R9_W {
        SYSCTL_PRGPIO_R9_W::new(self)
    }
    #[doc = "Bit 10 - GPIO Port L Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r10(&mut self) -> SYSCTL_PRGPIO_R10_W {
        SYSCTL_PRGPIO_R10_W::new(self)
    }
    #[doc = "Bit 11 - GPIO Port M Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r11(&mut self) -> SYSCTL_PRGPIO_R11_W {
        SYSCTL_PRGPIO_R11_W::new(self)
    }
    #[doc = "Bit 12 - GPIO Port N Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r12(&mut self) -> SYSCTL_PRGPIO_R12_W {
        SYSCTL_PRGPIO_R12_W::new(self)
    }
    #[doc = "Bit 13 - GPIO Port P Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r13(&mut self) -> SYSCTL_PRGPIO_R13_W {
        SYSCTL_PRGPIO_R13_W::new(self)
    }
    #[doc = "Bit 14 - GPIO Port Q Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r14(&mut self) -> SYSCTL_PRGPIO_R14_W {
        SYSCTL_PRGPIO_R14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General-Purpose Input/Output Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prgpio](index.html) module"]
pub struct PRGPIO_SPEC;
impl crate::RegisterSpec for PRGPIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prgpio::R](R) reader structure"]
impl crate::Readable for PRGPIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prgpio::W](W) writer structure"]
impl crate::Writable for PRGPIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRGPIO to value 0"]
impl crate::Resettable for PRGPIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
