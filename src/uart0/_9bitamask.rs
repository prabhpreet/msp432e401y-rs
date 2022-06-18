#[doc = "Register `_9BITAMASK` reader"]
pub struct R(crate::R<_9BITAMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_9BITAMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_9BITAMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_9BITAMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_9BITAMASK` writer"]
pub struct W(crate::W<_9BITAMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_9BITAMASK_SPEC>;
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
impl From<crate::W<_9BITAMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_9BITAMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_9BITAMASK_MASK` reader - Self Address Mask for 9-Bit Mode"]
pub type UART_9BITAMASK_MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UART_9BITAMASK_MASK` writer - Self Address Mask for 9-Bit Mode"]
pub type UART_9BITAMASK_MASK_W<'a> = crate::FieldWriter<'a, u32, _9BITAMASK_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Self Address Mask for 9-Bit Mode"]
    #[inline(always)]
    pub fn uart_9bitamask_mask(&self) -> UART_9BITAMASK_MASK_R {
        UART_9BITAMASK_MASK_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Self Address Mask for 9-Bit Mode"]
    #[inline(always)]
    pub fn uart_9bitamask_mask(&mut self) -> UART_9BITAMASK_MASK_W {
        UART_9BITAMASK_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART 9-Bit Self Address Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_9bitamask](index.html) module"]
pub struct _9BITAMASK_SPEC;
impl crate::RegisterSpec for _9BITAMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_9bitamask::R](R) reader structure"]
impl crate::Readable for _9BITAMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_9bitamask::W](W) writer structure"]
impl crate::Writable for _9BITAMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _9BITAMASK to value 0"]
impl crate::Resettable for _9BITAMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
