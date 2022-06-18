#[doc = "Register `PRTIMER` reader"]
pub struct R(crate::R<PRTIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRTIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRTIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRTIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRTIMER` writer"]
pub struct W(crate::W<PRTIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRTIMER_SPEC>;
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
impl From<crate::W<PRTIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRTIMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PRTIMER_R0` reader - 16/32-Bit General-Purpose Timer 0 Peripheral Ready"]
pub type SYSCTL_PRTIMER_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRTIMER_R0` writer - 16/32-Bit General-Purpose Timer 0 Peripheral Ready"]
pub type SYSCTL_PRTIMER_R0_W<'a> = crate::BitWriter<'a, u32, PRTIMER_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_PRTIMER_R1` reader - 16/32-Bit General-Purpose Timer 1 Peripheral Ready"]
pub type SYSCTL_PRTIMER_R1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRTIMER_R1` writer - 16/32-Bit General-Purpose Timer 1 Peripheral Ready"]
pub type SYSCTL_PRTIMER_R1_W<'a> = crate::BitWriter<'a, u32, PRTIMER_SPEC, bool, 1>;
#[doc = "Field `SYSCTL_PRTIMER_R2` reader - 16/32-Bit General-Purpose Timer 2 Peripheral Ready"]
pub type SYSCTL_PRTIMER_R2_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRTIMER_R2` writer - 16/32-Bit General-Purpose Timer 2 Peripheral Ready"]
pub type SYSCTL_PRTIMER_R2_W<'a> = crate::BitWriter<'a, u32, PRTIMER_SPEC, bool, 2>;
#[doc = "Field `SYSCTL_PRTIMER_R3` reader - 16/32-Bit General-Purpose Timer 3 Peripheral Ready"]
pub type SYSCTL_PRTIMER_R3_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRTIMER_R3` writer - 16/32-Bit General-Purpose Timer 3 Peripheral Ready"]
pub type SYSCTL_PRTIMER_R3_W<'a> = crate::BitWriter<'a, u32, PRTIMER_SPEC, bool, 3>;
#[doc = "Field `SYSCTL_PRTIMER_R4` reader - 16/32-Bit General-Purpose Timer 4 Peripheral Ready"]
pub type SYSCTL_PRTIMER_R4_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRTIMER_R4` writer - 16/32-Bit General-Purpose Timer 4 Peripheral Ready"]
pub type SYSCTL_PRTIMER_R4_W<'a> = crate::BitWriter<'a, u32, PRTIMER_SPEC, bool, 4>;
#[doc = "Field `SYSCTL_PRTIMER_R5` reader - 16/32-Bit General-Purpose Timer 5 Peripheral Ready"]
pub type SYSCTL_PRTIMER_R5_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRTIMER_R5` writer - 16/32-Bit General-Purpose Timer 5 Peripheral Ready"]
pub type SYSCTL_PRTIMER_R5_W<'a> = crate::BitWriter<'a, u32, PRTIMER_SPEC, bool, 5>;
#[doc = "Field `SYSCTL_PRTIMER_R6` reader - 16/32-Bit General-Purpose Timer 6 Peripheral Ready"]
pub type SYSCTL_PRTIMER_R6_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRTIMER_R6` writer - 16/32-Bit General-Purpose Timer 6 Peripheral Ready"]
pub type SYSCTL_PRTIMER_R6_W<'a> = crate::BitWriter<'a, u32, PRTIMER_SPEC, bool, 6>;
#[doc = "Field `SYSCTL_PRTIMER_R7` reader - 16/32-Bit General-Purpose Timer 7 Peripheral Ready"]
pub type SYSCTL_PRTIMER_R7_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRTIMER_R7` writer - 16/32-Bit General-Purpose Timer 7 Peripheral Ready"]
pub type SYSCTL_PRTIMER_R7_W<'a> = crate::BitWriter<'a, u32, PRTIMER_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - 16/32-Bit General-Purpose Timer 0 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prtimer_r0(&self) -> SYSCTL_PRTIMER_R0_R {
        SYSCTL_PRTIMER_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 16/32-Bit General-Purpose Timer 1 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prtimer_r1(&self) -> SYSCTL_PRTIMER_R1_R {
        SYSCTL_PRTIMER_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 16/32-Bit General-Purpose Timer 2 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prtimer_r2(&self) -> SYSCTL_PRTIMER_R2_R {
        SYSCTL_PRTIMER_R2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 16/32-Bit General-Purpose Timer 3 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prtimer_r3(&self) -> SYSCTL_PRTIMER_R3_R {
        SYSCTL_PRTIMER_R3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 16/32-Bit General-Purpose Timer 4 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prtimer_r4(&self) -> SYSCTL_PRTIMER_R4_R {
        SYSCTL_PRTIMER_R4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 16/32-Bit General-Purpose Timer 5 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prtimer_r5(&self) -> SYSCTL_PRTIMER_R5_R {
        SYSCTL_PRTIMER_R5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 16/32-Bit General-Purpose Timer 6 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prtimer_r6(&self) -> SYSCTL_PRTIMER_R6_R {
        SYSCTL_PRTIMER_R6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 16/32-Bit General-Purpose Timer 7 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prtimer_r7(&self) -> SYSCTL_PRTIMER_R7_R {
        SYSCTL_PRTIMER_R7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 16/32-Bit General-Purpose Timer 0 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prtimer_r0(&mut self) -> SYSCTL_PRTIMER_R0_W {
        SYSCTL_PRTIMER_R0_W::new(self)
    }
    #[doc = "Bit 1 - 16/32-Bit General-Purpose Timer 1 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prtimer_r1(&mut self) -> SYSCTL_PRTIMER_R1_W {
        SYSCTL_PRTIMER_R1_W::new(self)
    }
    #[doc = "Bit 2 - 16/32-Bit General-Purpose Timer 2 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prtimer_r2(&mut self) -> SYSCTL_PRTIMER_R2_W {
        SYSCTL_PRTIMER_R2_W::new(self)
    }
    #[doc = "Bit 3 - 16/32-Bit General-Purpose Timer 3 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prtimer_r3(&mut self) -> SYSCTL_PRTIMER_R3_W {
        SYSCTL_PRTIMER_R3_W::new(self)
    }
    #[doc = "Bit 4 - 16/32-Bit General-Purpose Timer 4 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prtimer_r4(&mut self) -> SYSCTL_PRTIMER_R4_W {
        SYSCTL_PRTIMER_R4_W::new(self)
    }
    #[doc = "Bit 5 - 16/32-Bit General-Purpose Timer 5 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prtimer_r5(&mut self) -> SYSCTL_PRTIMER_R5_W {
        SYSCTL_PRTIMER_R5_W::new(self)
    }
    #[doc = "Bit 6 - 16/32-Bit General-Purpose Timer 6 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prtimer_r6(&mut self) -> SYSCTL_PRTIMER_R6_W {
        SYSCTL_PRTIMER_R6_W::new(self)
    }
    #[doc = "Bit 7 - 16/32-Bit General-Purpose Timer 7 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prtimer_r7(&mut self) -> SYSCTL_PRTIMER_R7_W {
        SYSCTL_PRTIMER_R7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "16/32-Bit General-Purpose Timer Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prtimer](index.html) module"]
pub struct PRTIMER_SPEC;
impl crate::RegisterSpec for PRTIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prtimer::R](R) reader structure"]
impl crate::Readable for PRTIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prtimer::W](W) writer structure"]
impl crate::Writable for PRTIMER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRTIMER to value 0"]
impl crate::Resettable for PRTIMER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
