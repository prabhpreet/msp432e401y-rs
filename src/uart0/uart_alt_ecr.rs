#[doc = "Register `ECR` reader"]
pub struct R(crate::R<UART_ALT_ECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_ALT_ECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_ALT_ECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_ALT_ECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECR` writer"]
pub struct W(crate::W<UART_ALT_ECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_ALT_ECR_SPEC>;
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
impl From<crate::W<UART_ALT_ECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_ALT_ECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_ECR_DATA` reader - Error Clear"]
pub type UART_ECR_DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UART_ECR_DATA` writer - Error Clear"]
pub type UART_ECR_DATA_W<'a> = crate::FieldWriter<'a, u32, UART_ALT_ECR_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Error Clear"]
    #[inline(always)]
    pub fn uart_ecr_data(&self) -> UART_ECR_DATA_R {
        UART_ECR_DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Error Clear"]
    #[inline(always)]
    pub fn uart_ecr_data(&mut self) -> UART_ECR_DATA_W {
        UART_ECR_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Receive Status/Error Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_alt_ecr](index.html) module"]
pub struct UART_ALT_ECR_SPEC;
impl crate::RegisterSpec for UART_ALT_ECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_alt_ecr::R](R) reader structure"]
impl crate::Readable for UART_ALT_ECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_alt_ecr::W](W) writer structure"]
impl crate::Writable for UART_ALT_ECR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ECR to value 0"]
impl crate::Resettable for UART_ALT_ECR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
