#[doc = "Register `PPTIMER` reader"]
pub struct R(crate::R<PPTIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPTIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPTIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPTIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPTIMER` writer"]
pub struct W(crate::W<PPTIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPTIMER_SPEC>;
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
impl From<crate::W<PPTIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPTIMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PPTIMER_P0` reader - 16/32-Bit General-Purpose Timer 0 Present"]
pub type SYSCTL_PPTIMER_P0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPTIMER_P0` writer - 16/32-Bit General-Purpose Timer 0 Present"]
pub type SYSCTL_PPTIMER_P0_W<'a> = crate::BitWriter<'a, u32, PPTIMER_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_PPTIMER_P1` reader - 16/32-Bit General-Purpose Timer 1 Present"]
pub type SYSCTL_PPTIMER_P1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPTIMER_P1` writer - 16/32-Bit General-Purpose Timer 1 Present"]
pub type SYSCTL_PPTIMER_P1_W<'a> = crate::BitWriter<'a, u32, PPTIMER_SPEC, bool, 1>;
#[doc = "Field `SYSCTL_PPTIMER_P2` reader - 16/32-Bit General-Purpose Timer 2 Present"]
pub type SYSCTL_PPTIMER_P2_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPTIMER_P2` writer - 16/32-Bit General-Purpose Timer 2 Present"]
pub type SYSCTL_PPTIMER_P2_W<'a> = crate::BitWriter<'a, u32, PPTIMER_SPEC, bool, 2>;
#[doc = "Field `SYSCTL_PPTIMER_P3` reader - 16/32-Bit General-Purpose Timer 3 Present"]
pub type SYSCTL_PPTIMER_P3_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPTIMER_P3` writer - 16/32-Bit General-Purpose Timer 3 Present"]
pub type SYSCTL_PPTIMER_P3_W<'a> = crate::BitWriter<'a, u32, PPTIMER_SPEC, bool, 3>;
#[doc = "Field `SYSCTL_PPTIMER_P4` reader - 16/32-Bit General-Purpose Timer 4 Present"]
pub type SYSCTL_PPTIMER_P4_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPTIMER_P4` writer - 16/32-Bit General-Purpose Timer 4 Present"]
pub type SYSCTL_PPTIMER_P4_W<'a> = crate::BitWriter<'a, u32, PPTIMER_SPEC, bool, 4>;
#[doc = "Field `SYSCTL_PPTIMER_P5` reader - 16/32-Bit General-Purpose Timer 5 Present"]
pub type SYSCTL_PPTIMER_P5_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPTIMER_P5` writer - 16/32-Bit General-Purpose Timer 5 Present"]
pub type SYSCTL_PPTIMER_P5_W<'a> = crate::BitWriter<'a, u32, PPTIMER_SPEC, bool, 5>;
#[doc = "Field `SYSCTL_PPTIMER_P6` reader - 16/32-Bit General-Purpose Timer 6 Present"]
pub type SYSCTL_PPTIMER_P6_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPTIMER_P6` writer - 16/32-Bit General-Purpose Timer 6 Present"]
pub type SYSCTL_PPTIMER_P6_W<'a> = crate::BitWriter<'a, u32, PPTIMER_SPEC, bool, 6>;
#[doc = "Field `SYSCTL_PPTIMER_P7` reader - 16/32-Bit General-Purpose Timer 7 Present"]
pub type SYSCTL_PPTIMER_P7_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPTIMER_P7` writer - 16/32-Bit General-Purpose Timer 7 Present"]
pub type SYSCTL_PPTIMER_P7_W<'a> = crate::BitWriter<'a, u32, PPTIMER_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - 16/32-Bit General-Purpose Timer 0 Present"]
    #[inline(always)]
    pub fn sysctl_pptimer_p0(&self) -> SYSCTL_PPTIMER_P0_R {
        SYSCTL_PPTIMER_P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 16/32-Bit General-Purpose Timer 1 Present"]
    #[inline(always)]
    pub fn sysctl_pptimer_p1(&self) -> SYSCTL_PPTIMER_P1_R {
        SYSCTL_PPTIMER_P1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 16/32-Bit General-Purpose Timer 2 Present"]
    #[inline(always)]
    pub fn sysctl_pptimer_p2(&self) -> SYSCTL_PPTIMER_P2_R {
        SYSCTL_PPTIMER_P2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 16/32-Bit General-Purpose Timer 3 Present"]
    #[inline(always)]
    pub fn sysctl_pptimer_p3(&self) -> SYSCTL_PPTIMER_P3_R {
        SYSCTL_PPTIMER_P3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 16/32-Bit General-Purpose Timer 4 Present"]
    #[inline(always)]
    pub fn sysctl_pptimer_p4(&self) -> SYSCTL_PPTIMER_P4_R {
        SYSCTL_PPTIMER_P4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 16/32-Bit General-Purpose Timer 5 Present"]
    #[inline(always)]
    pub fn sysctl_pptimer_p5(&self) -> SYSCTL_PPTIMER_P5_R {
        SYSCTL_PPTIMER_P5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 16/32-Bit General-Purpose Timer 6 Present"]
    #[inline(always)]
    pub fn sysctl_pptimer_p6(&self) -> SYSCTL_PPTIMER_P6_R {
        SYSCTL_PPTIMER_P6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 16/32-Bit General-Purpose Timer 7 Present"]
    #[inline(always)]
    pub fn sysctl_pptimer_p7(&self) -> SYSCTL_PPTIMER_P7_R {
        SYSCTL_PPTIMER_P7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 16/32-Bit General-Purpose Timer 0 Present"]
    #[inline(always)]
    pub fn sysctl_pptimer_p0(&mut self) -> SYSCTL_PPTIMER_P0_W {
        SYSCTL_PPTIMER_P0_W::new(self)
    }
    #[doc = "Bit 1 - 16/32-Bit General-Purpose Timer 1 Present"]
    #[inline(always)]
    pub fn sysctl_pptimer_p1(&mut self) -> SYSCTL_PPTIMER_P1_W {
        SYSCTL_PPTIMER_P1_W::new(self)
    }
    #[doc = "Bit 2 - 16/32-Bit General-Purpose Timer 2 Present"]
    #[inline(always)]
    pub fn sysctl_pptimer_p2(&mut self) -> SYSCTL_PPTIMER_P2_W {
        SYSCTL_PPTIMER_P2_W::new(self)
    }
    #[doc = "Bit 3 - 16/32-Bit General-Purpose Timer 3 Present"]
    #[inline(always)]
    pub fn sysctl_pptimer_p3(&mut self) -> SYSCTL_PPTIMER_P3_W {
        SYSCTL_PPTIMER_P3_W::new(self)
    }
    #[doc = "Bit 4 - 16/32-Bit General-Purpose Timer 4 Present"]
    #[inline(always)]
    pub fn sysctl_pptimer_p4(&mut self) -> SYSCTL_PPTIMER_P4_W {
        SYSCTL_PPTIMER_P4_W::new(self)
    }
    #[doc = "Bit 5 - 16/32-Bit General-Purpose Timer 5 Present"]
    #[inline(always)]
    pub fn sysctl_pptimer_p5(&mut self) -> SYSCTL_PPTIMER_P5_W {
        SYSCTL_PPTIMER_P5_W::new(self)
    }
    #[doc = "Bit 6 - 16/32-Bit General-Purpose Timer 6 Present"]
    #[inline(always)]
    pub fn sysctl_pptimer_p6(&mut self) -> SYSCTL_PPTIMER_P6_W {
        SYSCTL_PPTIMER_P6_W::new(self)
    }
    #[doc = "Bit 7 - 16/32-Bit General-Purpose Timer 7 Present"]
    #[inline(always)]
    pub fn sysctl_pptimer_p7(&mut self) -> SYSCTL_PPTIMER_P7_W {
        SYSCTL_PPTIMER_P7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "16/32-Bit General-Purpose Timer Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pptimer](index.html) module"]
pub struct PPTIMER_SPEC;
impl crate::RegisterSpec for PPTIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pptimer::R](R) reader structure"]
impl crate::Readable for PPTIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pptimer::W](W) writer structure"]
impl crate::Writable for PPTIMER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PPTIMER to value 0"]
impl crate::Resettable for PPTIMER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
