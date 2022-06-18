#[doc = "Register `RCGCUART` reader"]
pub struct R(crate::R<RCGCUART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCGCUART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCGCUART_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCGCUART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCGCUART` writer"]
pub struct W(crate::W<RCGCUART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCGCUART_SPEC>;
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
impl From<crate::W<RCGCUART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCGCUART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_RCGCUART_R0` reader - UART Module 0 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCUART_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RCGCUART_R0` writer - UART Module 0 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCUART_R0_W<'a> = crate::BitWriter<'a, u32, RCGCUART_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_RCGCUART_R1` reader - UART Module 1 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCUART_R1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RCGCUART_R1` writer - UART Module 1 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCUART_R1_W<'a> = crate::BitWriter<'a, u32, RCGCUART_SPEC, bool, 1>;
#[doc = "Field `SYSCTL_RCGCUART_R2` reader - UART Module 2 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCUART_R2_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RCGCUART_R2` writer - UART Module 2 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCUART_R2_W<'a> = crate::BitWriter<'a, u32, RCGCUART_SPEC, bool, 2>;
#[doc = "Field `SYSCTL_RCGCUART_R3` reader - UART Module 3 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCUART_R3_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RCGCUART_R3` writer - UART Module 3 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCUART_R3_W<'a> = crate::BitWriter<'a, u32, RCGCUART_SPEC, bool, 3>;
#[doc = "Field `SYSCTL_RCGCUART_R4` reader - UART Module 4 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCUART_R4_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RCGCUART_R4` writer - UART Module 4 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCUART_R4_W<'a> = crate::BitWriter<'a, u32, RCGCUART_SPEC, bool, 4>;
#[doc = "Field `SYSCTL_RCGCUART_R5` reader - UART Module 5 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCUART_R5_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RCGCUART_R5` writer - UART Module 5 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCUART_R5_W<'a> = crate::BitWriter<'a, u32, RCGCUART_SPEC, bool, 5>;
#[doc = "Field `SYSCTL_RCGCUART_R6` reader - UART Module 6 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCUART_R6_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RCGCUART_R6` writer - UART Module 6 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCUART_R6_W<'a> = crate::BitWriter<'a, u32, RCGCUART_SPEC, bool, 6>;
#[doc = "Field `SYSCTL_RCGCUART_R7` reader - UART Module 7 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCUART_R7_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RCGCUART_R7` writer - UART Module 7 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCUART_R7_W<'a> = crate::BitWriter<'a, u32, RCGCUART_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - UART Module 0 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcuart_r0(&self) -> SYSCTL_RCGCUART_R0_R {
        SYSCTL_RCGCUART_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART Module 1 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcuart_r1(&self) -> SYSCTL_RCGCUART_R1_R {
        SYSCTL_RCGCUART_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART Module 2 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcuart_r2(&self) -> SYSCTL_RCGCUART_R2_R {
        SYSCTL_RCGCUART_R2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART Module 3 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcuart_r3(&self) -> SYSCTL_RCGCUART_R3_R {
        SYSCTL_RCGCUART_R3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART Module 4 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcuart_r4(&self) -> SYSCTL_RCGCUART_R4_R {
        SYSCTL_RCGCUART_R4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART Module 5 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcuart_r5(&self) -> SYSCTL_RCGCUART_R5_R {
        SYSCTL_RCGCUART_R5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART Module 6 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcuart_r6(&self) -> SYSCTL_RCGCUART_R6_R {
        SYSCTL_RCGCUART_R6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART Module 7 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcuart_r7(&self) -> SYSCTL_RCGCUART_R7_R {
        SYSCTL_RCGCUART_R7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART Module 0 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcuart_r0(&mut self) -> SYSCTL_RCGCUART_R0_W {
        SYSCTL_RCGCUART_R0_W::new(self)
    }
    #[doc = "Bit 1 - UART Module 1 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcuart_r1(&mut self) -> SYSCTL_RCGCUART_R1_W {
        SYSCTL_RCGCUART_R1_W::new(self)
    }
    #[doc = "Bit 2 - UART Module 2 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcuart_r2(&mut self) -> SYSCTL_RCGCUART_R2_W {
        SYSCTL_RCGCUART_R2_W::new(self)
    }
    #[doc = "Bit 3 - UART Module 3 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcuart_r3(&mut self) -> SYSCTL_RCGCUART_R3_W {
        SYSCTL_RCGCUART_R3_W::new(self)
    }
    #[doc = "Bit 4 - UART Module 4 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcuart_r4(&mut self) -> SYSCTL_RCGCUART_R4_W {
        SYSCTL_RCGCUART_R4_W::new(self)
    }
    #[doc = "Bit 5 - UART Module 5 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcuart_r5(&mut self) -> SYSCTL_RCGCUART_R5_W {
        SYSCTL_RCGCUART_R5_W::new(self)
    }
    #[doc = "Bit 6 - UART Module 6 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcuart_r6(&mut self) -> SYSCTL_RCGCUART_R6_W {
        SYSCTL_RCGCUART_R6_W::new(self)
    }
    #[doc = "Bit 7 - UART Module 7 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcuart_r7(&mut self) -> SYSCTL_RCGCUART_R7_W {
        SYSCTL_RCGCUART_R7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgcuart](index.html) module"]
pub struct RCGCUART_SPEC;
impl crate::RegisterSpec for RCGCUART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcgcuart::R](R) reader structure"]
impl crate::Readable for RCGCUART_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcgcuart::W](W) writer structure"]
impl crate::Writable for RCGCUART_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCGCUART to value 0"]
impl crate::Resettable for RCGCUART_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
