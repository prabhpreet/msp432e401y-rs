#[doc = "Register `RSR` reader"]
pub struct R(crate::R<RSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSR` writer"]
pub struct W(crate::W<RSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSR_SPEC>;
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
impl From<crate::W<RSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_RSR_FE` reader - UART Framing Error"]
pub type UART_RSR_FE_R = crate::BitReader<bool>;
#[doc = "Field `UART_RSR_FE` writer - UART Framing Error"]
pub type UART_RSR_FE_W<'a> = crate::BitWriter<'a, u32, RSR_SPEC, bool, 0>;
#[doc = "Field `UART_RSR_PE` reader - UART Parity Error"]
pub type UART_RSR_PE_R = crate::BitReader<bool>;
#[doc = "Field `UART_RSR_PE` writer - UART Parity Error"]
pub type UART_RSR_PE_W<'a> = crate::BitWriter<'a, u32, RSR_SPEC, bool, 1>;
#[doc = "Field `UART_RSR_BE` reader - UART Break Error"]
pub type UART_RSR_BE_R = crate::BitReader<bool>;
#[doc = "Field `UART_RSR_BE` writer - UART Break Error"]
pub type UART_RSR_BE_W<'a> = crate::BitWriter<'a, u32, RSR_SPEC, bool, 2>;
#[doc = "Field `UART_RSR_OE` reader - UART Overrun Error"]
pub type UART_RSR_OE_R = crate::BitReader<bool>;
#[doc = "Field `UART_RSR_OE` writer - UART Overrun Error"]
pub type UART_RSR_OE_W<'a> = crate::BitWriter<'a, u32, RSR_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - UART Framing Error"]
    #[inline(always)]
    pub fn uart_rsr_fe(&self) -> UART_RSR_FE_R {
        UART_RSR_FE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART Parity Error"]
    #[inline(always)]
    pub fn uart_rsr_pe(&self) -> UART_RSR_PE_R {
        UART_RSR_PE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART Break Error"]
    #[inline(always)]
    pub fn uart_rsr_be(&self) -> UART_RSR_BE_R {
        UART_RSR_BE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART Overrun Error"]
    #[inline(always)]
    pub fn uart_rsr_oe(&self) -> UART_RSR_OE_R {
        UART_RSR_OE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART Framing Error"]
    #[inline(always)]
    pub fn uart_rsr_fe(&mut self) -> UART_RSR_FE_W {
        UART_RSR_FE_W::new(self)
    }
    #[doc = "Bit 1 - UART Parity Error"]
    #[inline(always)]
    pub fn uart_rsr_pe(&mut self) -> UART_RSR_PE_W {
        UART_RSR_PE_W::new(self)
    }
    #[doc = "Bit 2 - UART Break Error"]
    #[inline(always)]
    pub fn uart_rsr_be(&mut self) -> UART_RSR_BE_W {
        UART_RSR_BE_W::new(self)
    }
    #[doc = "Bit 3 - UART Overrun Error"]
    #[inline(always)]
    pub fn uart_rsr_oe(&mut self) -> UART_RSR_OE_W {
        UART_RSR_OE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Receive Status/Error Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsr](index.html) module"]
pub struct RSR_SPEC;
impl crate::RegisterSpec for RSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsr::R](R) reader structure"]
impl crate::Readable for RSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rsr::W](W) writer structure"]
impl crate::Writable for RSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSR to value 0"]
impl crate::Resettable for RSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
