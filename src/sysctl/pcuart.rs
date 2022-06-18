#[doc = "Register `PCUART` reader"]
pub struct R(crate::R<PCUART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCUART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCUART_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCUART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCUART` writer"]
pub struct W(crate::W<PCUART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCUART_SPEC>;
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
impl From<crate::W<PCUART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCUART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PCUART_P0` reader - UART Module 0 Power Control"]
pub type SYSCTL_PCUART_P0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PCUART_P0` writer - UART Module 0 Power Control"]
pub type SYSCTL_PCUART_P0_W<'a> = crate::BitWriter<'a, u32, PCUART_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_PCUART_P1` reader - UART Module 1 Power Control"]
pub type SYSCTL_PCUART_P1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PCUART_P1` writer - UART Module 1 Power Control"]
pub type SYSCTL_PCUART_P1_W<'a> = crate::BitWriter<'a, u32, PCUART_SPEC, bool, 1>;
#[doc = "Field `SYSCTL_PCUART_P2` reader - UART Module 2 Power Control"]
pub type SYSCTL_PCUART_P2_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PCUART_P2` writer - UART Module 2 Power Control"]
pub type SYSCTL_PCUART_P2_W<'a> = crate::BitWriter<'a, u32, PCUART_SPEC, bool, 2>;
#[doc = "Field `SYSCTL_PCUART_P3` reader - UART Module 3 Power Control"]
pub type SYSCTL_PCUART_P3_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PCUART_P3` writer - UART Module 3 Power Control"]
pub type SYSCTL_PCUART_P3_W<'a> = crate::BitWriter<'a, u32, PCUART_SPEC, bool, 3>;
#[doc = "Field `SYSCTL_PCUART_P4` reader - UART Module 4 Power Control"]
pub type SYSCTL_PCUART_P4_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PCUART_P4` writer - UART Module 4 Power Control"]
pub type SYSCTL_PCUART_P4_W<'a> = crate::BitWriter<'a, u32, PCUART_SPEC, bool, 4>;
#[doc = "Field `SYSCTL_PCUART_P5` reader - UART Module 5 Power Control"]
pub type SYSCTL_PCUART_P5_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PCUART_P5` writer - UART Module 5 Power Control"]
pub type SYSCTL_PCUART_P5_W<'a> = crate::BitWriter<'a, u32, PCUART_SPEC, bool, 5>;
#[doc = "Field `SYSCTL_PCUART_P6` reader - UART Module 6 Power Control"]
pub type SYSCTL_PCUART_P6_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PCUART_P6` writer - UART Module 6 Power Control"]
pub type SYSCTL_PCUART_P6_W<'a> = crate::BitWriter<'a, u32, PCUART_SPEC, bool, 6>;
#[doc = "Field `SYSCTL_PCUART_P7` reader - UART Module 7 Power Control"]
pub type SYSCTL_PCUART_P7_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PCUART_P7` writer - UART Module 7 Power Control"]
pub type SYSCTL_PCUART_P7_W<'a> = crate::BitWriter<'a, u32, PCUART_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - UART Module 0 Power Control"]
    #[inline(always)]
    pub fn sysctl_pcuart_p0(&self) -> SYSCTL_PCUART_P0_R {
        SYSCTL_PCUART_P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART Module 1 Power Control"]
    #[inline(always)]
    pub fn sysctl_pcuart_p1(&self) -> SYSCTL_PCUART_P1_R {
        SYSCTL_PCUART_P1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART Module 2 Power Control"]
    #[inline(always)]
    pub fn sysctl_pcuart_p2(&self) -> SYSCTL_PCUART_P2_R {
        SYSCTL_PCUART_P2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART Module 3 Power Control"]
    #[inline(always)]
    pub fn sysctl_pcuart_p3(&self) -> SYSCTL_PCUART_P3_R {
        SYSCTL_PCUART_P3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART Module 4 Power Control"]
    #[inline(always)]
    pub fn sysctl_pcuart_p4(&self) -> SYSCTL_PCUART_P4_R {
        SYSCTL_PCUART_P4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART Module 5 Power Control"]
    #[inline(always)]
    pub fn sysctl_pcuart_p5(&self) -> SYSCTL_PCUART_P5_R {
        SYSCTL_PCUART_P5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART Module 6 Power Control"]
    #[inline(always)]
    pub fn sysctl_pcuart_p6(&self) -> SYSCTL_PCUART_P6_R {
        SYSCTL_PCUART_P6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART Module 7 Power Control"]
    #[inline(always)]
    pub fn sysctl_pcuart_p7(&self) -> SYSCTL_PCUART_P7_R {
        SYSCTL_PCUART_P7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART Module 0 Power Control"]
    #[inline(always)]
    pub fn sysctl_pcuart_p0(&mut self) -> SYSCTL_PCUART_P0_W {
        SYSCTL_PCUART_P0_W::new(self)
    }
    #[doc = "Bit 1 - UART Module 1 Power Control"]
    #[inline(always)]
    pub fn sysctl_pcuart_p1(&mut self) -> SYSCTL_PCUART_P1_W {
        SYSCTL_PCUART_P1_W::new(self)
    }
    #[doc = "Bit 2 - UART Module 2 Power Control"]
    #[inline(always)]
    pub fn sysctl_pcuart_p2(&mut self) -> SYSCTL_PCUART_P2_W {
        SYSCTL_PCUART_P2_W::new(self)
    }
    #[doc = "Bit 3 - UART Module 3 Power Control"]
    #[inline(always)]
    pub fn sysctl_pcuart_p3(&mut self) -> SYSCTL_PCUART_P3_W {
        SYSCTL_PCUART_P3_W::new(self)
    }
    #[doc = "Bit 4 - UART Module 4 Power Control"]
    #[inline(always)]
    pub fn sysctl_pcuart_p4(&mut self) -> SYSCTL_PCUART_P4_W {
        SYSCTL_PCUART_P4_W::new(self)
    }
    #[doc = "Bit 5 - UART Module 5 Power Control"]
    #[inline(always)]
    pub fn sysctl_pcuart_p5(&mut self) -> SYSCTL_PCUART_P5_W {
        SYSCTL_PCUART_P5_W::new(self)
    }
    #[doc = "Bit 6 - UART Module 6 Power Control"]
    #[inline(always)]
    pub fn sysctl_pcuart_p6(&mut self) -> SYSCTL_PCUART_P6_W {
        SYSCTL_PCUART_P6_W::new(self)
    }
    #[doc = "Bit 7 - UART Module 7 Power Control"]
    #[inline(always)]
    pub fn sysctl_pcuart_p7(&mut self) -> SYSCTL_PCUART_P7_W {
        SYSCTL_PCUART_P7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcuart](index.html) module"]
pub struct PCUART_SPEC;
impl crate::RegisterSpec for PCUART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcuart::R](R) reader structure"]
impl crate::Readable for PCUART_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcuart::W](W) writer structure"]
impl crate::Writable for PCUART_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCUART to value 0"]
impl crate::Resettable for PCUART_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
