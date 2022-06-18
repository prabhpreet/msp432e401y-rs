#[doc = "Register `ILPR` reader"]
pub struct R(crate::R<ILPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ILPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ILPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ILPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ILPR` writer"]
pub struct W(crate::W<ILPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ILPR_SPEC>;
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
impl From<crate::W<ILPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ILPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_ILPR_ILPDVSR` reader - IrDA Low-Power Divisor"]
pub type UART_ILPR_ILPDVSR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UART_ILPR_ILPDVSR` writer - IrDA Low-Power Divisor"]
pub type UART_ILPR_ILPDVSR_W<'a> = crate::FieldWriter<'a, u32, ILPR_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - IrDA Low-Power Divisor"]
    #[inline(always)]
    pub fn uart_ilpr_ilpdvsr(&self) -> UART_ILPR_ILPDVSR_R {
        UART_ILPR_ILPDVSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IrDA Low-Power Divisor"]
    #[inline(always)]
    pub fn uart_ilpr_ilpdvsr(&mut self) -> UART_ILPR_ILPDVSR_W {
        UART_ILPR_ILPDVSR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART IrDA Low-Power Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ilpr](index.html) module"]
pub struct ILPR_SPEC;
impl crate::RegisterSpec for ILPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ilpr::R](R) reader structure"]
impl crate::Readable for ILPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ilpr::W](W) writer structure"]
impl crate::Writable for ILPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ILPR to value 0"]
impl crate::Resettable for ILPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
