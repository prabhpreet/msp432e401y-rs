#[doc = "Register `DCGCTIMER` reader"]
pub struct R(crate::R<DCGCTIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCGCTIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCGCTIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCGCTIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCGCTIMER` writer"]
pub struct W(crate::W<DCGCTIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCGCTIMER_SPEC>;
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
impl From<crate::W<DCGCTIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCGCTIMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_DCGCTIMER_D0` reader - 16/32-Bit General-Purpose Timer 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCTIMER_D0` writer - 16/32-Bit General-Purpose Timer 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D0_W<'a> = crate::BitWriter<'a, u32, DCGCTIMER_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_DCGCTIMER_D1` reader - 16/32-Bit General-Purpose Timer 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCTIMER_D1` writer - 16/32-Bit General-Purpose Timer 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D1_W<'a> = crate::BitWriter<'a, u32, DCGCTIMER_SPEC, bool, 1>;
#[doc = "Field `SYSCTL_DCGCTIMER_D2` reader - 16/32-Bit General-Purpose Timer 2 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D2_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCTIMER_D2` writer - 16/32-Bit General-Purpose Timer 2 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D2_W<'a> = crate::BitWriter<'a, u32, DCGCTIMER_SPEC, bool, 2>;
#[doc = "Field `SYSCTL_DCGCTIMER_D3` reader - 16/32-Bit General-Purpose Timer 3 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D3_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCTIMER_D3` writer - 16/32-Bit General-Purpose Timer 3 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D3_W<'a> = crate::BitWriter<'a, u32, DCGCTIMER_SPEC, bool, 3>;
#[doc = "Field `SYSCTL_DCGCTIMER_D4` reader - 16/32-Bit General-Purpose Timer 4 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D4_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCTIMER_D4` writer - 16/32-Bit General-Purpose Timer 4 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D4_W<'a> = crate::BitWriter<'a, u32, DCGCTIMER_SPEC, bool, 4>;
#[doc = "Field `SYSCTL_DCGCTIMER_D5` reader - 16/32-Bit General-Purpose Timer 5 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D5_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCTIMER_D5` writer - 16/32-Bit General-Purpose Timer 5 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D5_W<'a> = crate::BitWriter<'a, u32, DCGCTIMER_SPEC, bool, 5>;
#[doc = "Field `SYSCTL_DCGCTIMER_D6` reader - 16/32-Bit General-Purpose Timer 6 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D6_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCTIMER_D6` writer - 16/32-Bit General-Purpose Timer 6 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D6_W<'a> = crate::BitWriter<'a, u32, DCGCTIMER_SPEC, bool, 6>;
#[doc = "Field `SYSCTL_DCGCTIMER_D7` reader - 16/32-Bit General-Purpose Timer 7 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D7_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCTIMER_D7` writer - 16/32-Bit General-Purpose Timer 7 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D7_W<'a> = crate::BitWriter<'a, u32, DCGCTIMER_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - 16/32-Bit General-Purpose Timer 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d0(&self) -> SYSCTL_DCGCTIMER_D0_R {
        SYSCTL_DCGCTIMER_D0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 16/32-Bit General-Purpose Timer 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d1(&self) -> SYSCTL_DCGCTIMER_D1_R {
        SYSCTL_DCGCTIMER_D1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 16/32-Bit General-Purpose Timer 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d2(&self) -> SYSCTL_DCGCTIMER_D2_R {
        SYSCTL_DCGCTIMER_D2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 16/32-Bit General-Purpose Timer 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d3(&self) -> SYSCTL_DCGCTIMER_D3_R {
        SYSCTL_DCGCTIMER_D3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 16/32-Bit General-Purpose Timer 4 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d4(&self) -> SYSCTL_DCGCTIMER_D4_R {
        SYSCTL_DCGCTIMER_D4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 16/32-Bit General-Purpose Timer 5 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d5(&self) -> SYSCTL_DCGCTIMER_D5_R {
        SYSCTL_DCGCTIMER_D5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 16/32-Bit General-Purpose Timer 6 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d6(&self) -> SYSCTL_DCGCTIMER_D6_R {
        SYSCTL_DCGCTIMER_D6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 16/32-Bit General-Purpose Timer 7 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d7(&self) -> SYSCTL_DCGCTIMER_D7_R {
        SYSCTL_DCGCTIMER_D7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 16/32-Bit General-Purpose Timer 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d0(&mut self) -> SYSCTL_DCGCTIMER_D0_W {
        SYSCTL_DCGCTIMER_D0_W::new(self)
    }
    #[doc = "Bit 1 - 16/32-Bit General-Purpose Timer 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d1(&mut self) -> SYSCTL_DCGCTIMER_D1_W {
        SYSCTL_DCGCTIMER_D1_W::new(self)
    }
    #[doc = "Bit 2 - 16/32-Bit General-Purpose Timer 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d2(&mut self) -> SYSCTL_DCGCTIMER_D2_W {
        SYSCTL_DCGCTIMER_D2_W::new(self)
    }
    #[doc = "Bit 3 - 16/32-Bit General-Purpose Timer 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d3(&mut self) -> SYSCTL_DCGCTIMER_D3_W {
        SYSCTL_DCGCTIMER_D3_W::new(self)
    }
    #[doc = "Bit 4 - 16/32-Bit General-Purpose Timer 4 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d4(&mut self) -> SYSCTL_DCGCTIMER_D4_W {
        SYSCTL_DCGCTIMER_D4_W::new(self)
    }
    #[doc = "Bit 5 - 16/32-Bit General-Purpose Timer 5 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d5(&mut self) -> SYSCTL_DCGCTIMER_D5_W {
        SYSCTL_DCGCTIMER_D5_W::new(self)
    }
    #[doc = "Bit 6 - 16/32-Bit General-Purpose Timer 6 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d6(&mut self) -> SYSCTL_DCGCTIMER_D6_W {
        SYSCTL_DCGCTIMER_D6_W::new(self)
    }
    #[doc = "Bit 7 - 16/32-Bit General-Purpose Timer 7 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d7(&mut self) -> SYSCTL_DCGCTIMER_D7_W {
        SYSCTL_DCGCTIMER_D7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "16/32-Bit General-Purpose Timer Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgctimer](index.html) module"]
pub struct DCGCTIMER_SPEC;
impl crate::RegisterSpec for DCGCTIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcgctimer::R](R) reader structure"]
impl crate::Readable for DCGCTIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcgctimer::W](W) writer structure"]
impl crate::Writable for DCGCTIMER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCGCTIMER to value 0"]
impl crate::Resettable for DCGCTIMER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
