#[doc = "Register `FBRD` reader"]
pub struct R(crate::R<FBRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FBRD` writer"]
pub struct W(crate::W<FBRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBRD_SPEC>;
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
impl From<crate::W<FBRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBRD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_FBRD_DIVFRAC` reader - Fractional Baud-Rate Divisor"]
pub type UART_FBRD_DIVFRAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UART_FBRD_DIVFRAC` writer - Fractional Baud-Rate Divisor"]
pub type UART_FBRD_DIVFRAC_W<'a> = crate::FieldWriter<'a, u32, FBRD_SPEC, u8, u8, 6, 0>;
impl R {
    #[doc = "Bits 0:5 - Fractional Baud-Rate Divisor"]
    #[inline(always)]
    pub fn uart_fbrd_divfrac(&self) -> UART_FBRD_DIVFRAC_R {
        UART_FBRD_DIVFRAC_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Fractional Baud-Rate Divisor"]
    #[inline(always)]
    pub fn uart_fbrd_divfrac(&mut self) -> UART_FBRD_DIVFRAC_W {
        UART_FBRD_DIVFRAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Fractional Baud-Rate Divisor\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbrd](index.html) module"]
pub struct FBRD_SPEC;
impl crate::RegisterSpec for FBRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fbrd::R](R) reader structure"]
impl crate::Readable for FBRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fbrd::W](W) writer structure"]
impl crate::Writable for FBRD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FBRD to value 0"]
impl crate::Resettable for FBRD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
