#[doc = "Register `DCGCUART` reader"]
pub struct R(crate::R<DCGCUART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCGCUART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCGCUART_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCGCUART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCGCUART` writer"]
pub struct W(crate::W<DCGCUART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCGCUART_SPEC>;
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
impl From<crate::W<DCGCUART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCGCUART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_DCGCUART_D0` reader - UART Module 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCUART_D0` writer - UART Module 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D0_W<'a> = crate::BitWriter<'a, u32, DCGCUART_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_DCGCUART_D1` reader - UART Module 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCUART_D1` writer - UART Module 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D1_W<'a> = crate::BitWriter<'a, u32, DCGCUART_SPEC, bool, 1>;
#[doc = "Field `SYSCTL_DCGCUART_D2` reader - UART Module 2 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D2_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCUART_D2` writer - UART Module 2 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D2_W<'a> = crate::BitWriter<'a, u32, DCGCUART_SPEC, bool, 2>;
#[doc = "Field `SYSCTL_DCGCUART_D3` reader - UART Module 3 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D3_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCUART_D3` writer - UART Module 3 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D3_W<'a> = crate::BitWriter<'a, u32, DCGCUART_SPEC, bool, 3>;
#[doc = "Field `SYSCTL_DCGCUART_D4` reader - UART Module 4 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D4_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCUART_D4` writer - UART Module 4 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D4_W<'a> = crate::BitWriter<'a, u32, DCGCUART_SPEC, bool, 4>;
#[doc = "Field `SYSCTL_DCGCUART_D5` reader - UART Module 5 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D5_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCUART_D5` writer - UART Module 5 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D5_W<'a> = crate::BitWriter<'a, u32, DCGCUART_SPEC, bool, 5>;
#[doc = "Field `SYSCTL_DCGCUART_D6` reader - UART Module 6 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D6_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCUART_D6` writer - UART Module 6 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D6_W<'a> = crate::BitWriter<'a, u32, DCGCUART_SPEC, bool, 6>;
#[doc = "Field `SYSCTL_DCGCUART_D7` reader - UART Module 7 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D7_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCUART_D7` writer - UART Module 7 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D7_W<'a> = crate::BitWriter<'a, u32, DCGCUART_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - UART Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcuart_d0(&self) -> SYSCTL_DCGCUART_D0_R {
        SYSCTL_DCGCUART_D0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcuart_d1(&self) -> SYSCTL_DCGCUART_D1_R {
        SYSCTL_DCGCUART_D1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART Module 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcuart_d2(&self) -> SYSCTL_DCGCUART_D2_R {
        SYSCTL_DCGCUART_D2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART Module 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcuart_d3(&self) -> SYSCTL_DCGCUART_D3_R {
        SYSCTL_DCGCUART_D3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART Module 4 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcuart_d4(&self) -> SYSCTL_DCGCUART_D4_R {
        SYSCTL_DCGCUART_D4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART Module 5 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcuart_d5(&self) -> SYSCTL_DCGCUART_D5_R {
        SYSCTL_DCGCUART_D5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART Module 6 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcuart_d6(&self) -> SYSCTL_DCGCUART_D6_R {
        SYSCTL_DCGCUART_D6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART Module 7 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcuart_d7(&self) -> SYSCTL_DCGCUART_D7_R {
        SYSCTL_DCGCUART_D7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcuart_d0(&mut self) -> SYSCTL_DCGCUART_D0_W {
        SYSCTL_DCGCUART_D0_W::new(self)
    }
    #[doc = "Bit 1 - UART Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcuart_d1(&mut self) -> SYSCTL_DCGCUART_D1_W {
        SYSCTL_DCGCUART_D1_W::new(self)
    }
    #[doc = "Bit 2 - UART Module 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcuart_d2(&mut self) -> SYSCTL_DCGCUART_D2_W {
        SYSCTL_DCGCUART_D2_W::new(self)
    }
    #[doc = "Bit 3 - UART Module 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcuart_d3(&mut self) -> SYSCTL_DCGCUART_D3_W {
        SYSCTL_DCGCUART_D3_W::new(self)
    }
    #[doc = "Bit 4 - UART Module 4 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcuart_d4(&mut self) -> SYSCTL_DCGCUART_D4_W {
        SYSCTL_DCGCUART_D4_W::new(self)
    }
    #[doc = "Bit 5 - UART Module 5 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcuart_d5(&mut self) -> SYSCTL_DCGCUART_D5_W {
        SYSCTL_DCGCUART_D5_W::new(self)
    }
    #[doc = "Bit 6 - UART Module 6 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcuart_d6(&mut self) -> SYSCTL_DCGCUART_D6_W {
        SYSCTL_DCGCUART_D6_W::new(self)
    }
    #[doc = "Bit 7 - UART Module 7 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcuart_d7(&mut self) -> SYSCTL_DCGCUART_D7_W {
        SYSCTL_DCGCUART_D7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgcuart](index.html) module"]
pub struct DCGCUART_SPEC;
impl crate::RegisterSpec for DCGCUART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcgcuart::R](R) reader structure"]
impl crate::Readable for DCGCUART_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcgcuart::W](W) writer structure"]
impl crate::Writable for DCGCUART_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCGCUART to value 0"]
impl crate::Resettable for DCGCUART_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
