#[doc = "Register `SCGCUART` reader"]
pub struct R(crate::R<SCGCUART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGCUART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGCUART_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGCUART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGCUART` writer"]
pub struct W(crate::W<SCGCUART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGCUART_SPEC>;
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
impl From<crate::W<SCGCUART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGCUART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SCGCUART_S0` reader - UART Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCUART_S0` writer - UART Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S0_W<'a> = crate::BitWriter<'a, u32, SCGCUART_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_SCGCUART_S1` reader - UART Module 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCUART_S1` writer - UART Module 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S1_W<'a> = crate::BitWriter<'a, u32, SCGCUART_SPEC, bool, 1>;
#[doc = "Field `SYSCTL_SCGCUART_S2` reader - UART Module 2 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S2_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCUART_S2` writer - UART Module 2 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S2_W<'a> = crate::BitWriter<'a, u32, SCGCUART_SPEC, bool, 2>;
#[doc = "Field `SYSCTL_SCGCUART_S3` reader - UART Module 3 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S3_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCUART_S3` writer - UART Module 3 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S3_W<'a> = crate::BitWriter<'a, u32, SCGCUART_SPEC, bool, 3>;
#[doc = "Field `SYSCTL_SCGCUART_S4` reader - UART Module 4 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S4_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCUART_S4` writer - UART Module 4 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S4_W<'a> = crate::BitWriter<'a, u32, SCGCUART_SPEC, bool, 4>;
#[doc = "Field `SYSCTL_SCGCUART_S5` reader - UART Module 5 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S5_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCUART_S5` writer - UART Module 5 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S5_W<'a> = crate::BitWriter<'a, u32, SCGCUART_SPEC, bool, 5>;
#[doc = "Field `SYSCTL_SCGCUART_S6` reader - UART Module 6 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S6_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCUART_S6` writer - UART Module 6 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S6_W<'a> = crate::BitWriter<'a, u32, SCGCUART_SPEC, bool, 6>;
#[doc = "Field `SYSCTL_SCGCUART_S7` reader - UART Module 7 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S7_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCUART_S7` writer - UART Module 7 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S7_W<'a> = crate::BitWriter<'a, u32, SCGCUART_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - UART Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcuart_s0(&self) -> SYSCTL_SCGCUART_S0_R {
        SYSCTL_SCGCUART_S0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART Module 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcuart_s1(&self) -> SYSCTL_SCGCUART_S1_R {
        SYSCTL_SCGCUART_S1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART Module 2 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcuart_s2(&self) -> SYSCTL_SCGCUART_S2_R {
        SYSCTL_SCGCUART_S2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART Module 3 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcuart_s3(&self) -> SYSCTL_SCGCUART_S3_R {
        SYSCTL_SCGCUART_S3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART Module 4 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcuart_s4(&self) -> SYSCTL_SCGCUART_S4_R {
        SYSCTL_SCGCUART_S4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART Module 5 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcuart_s5(&self) -> SYSCTL_SCGCUART_S5_R {
        SYSCTL_SCGCUART_S5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART Module 6 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcuart_s6(&self) -> SYSCTL_SCGCUART_S6_R {
        SYSCTL_SCGCUART_S6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART Module 7 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcuart_s7(&self) -> SYSCTL_SCGCUART_S7_R {
        SYSCTL_SCGCUART_S7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcuart_s0(&mut self) -> SYSCTL_SCGCUART_S0_W {
        SYSCTL_SCGCUART_S0_W::new(self)
    }
    #[doc = "Bit 1 - UART Module 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcuart_s1(&mut self) -> SYSCTL_SCGCUART_S1_W {
        SYSCTL_SCGCUART_S1_W::new(self)
    }
    #[doc = "Bit 2 - UART Module 2 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcuart_s2(&mut self) -> SYSCTL_SCGCUART_S2_W {
        SYSCTL_SCGCUART_S2_W::new(self)
    }
    #[doc = "Bit 3 - UART Module 3 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcuart_s3(&mut self) -> SYSCTL_SCGCUART_S3_W {
        SYSCTL_SCGCUART_S3_W::new(self)
    }
    #[doc = "Bit 4 - UART Module 4 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcuart_s4(&mut self) -> SYSCTL_SCGCUART_S4_W {
        SYSCTL_SCGCUART_S4_W::new(self)
    }
    #[doc = "Bit 5 - UART Module 5 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcuart_s5(&mut self) -> SYSCTL_SCGCUART_S5_W {
        SYSCTL_SCGCUART_S5_W::new(self)
    }
    #[doc = "Bit 6 - UART Module 6 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcuart_s6(&mut self) -> SYSCTL_SCGCUART_S6_W {
        SYSCTL_SCGCUART_S6_W::new(self)
    }
    #[doc = "Bit 7 - UART Module 7 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcuart_s7(&mut self) -> SYSCTL_SCGCUART_S7_W {
        SYSCTL_SCGCUART_S7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcuart](index.html) module"]
pub struct SCGCUART_SPEC;
impl crate::RegisterSpec for SCGCUART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgcuart::R](R) reader structure"]
impl crate::Readable for SCGCUART_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgcuart::W](W) writer structure"]
impl crate::Writable for SCGCUART_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCGCUART to value 0"]
impl crate::Resettable for SCGCUART_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
