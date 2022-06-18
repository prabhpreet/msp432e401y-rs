#[doc = "Register `_9BITADDR` reader"]
pub struct R(crate::R<_9BITADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_9BITADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_9BITADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_9BITADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_9BITADDR` writer"]
pub struct W(crate::W<_9BITADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_9BITADDR_SPEC>;
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
impl From<crate::W<_9BITADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_9BITADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_9BITADDR_ADDR` reader - Self Address for 9-Bit Mode"]
pub type UART_9BITADDR_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UART_9BITADDR_ADDR` writer - Self Address for 9-Bit Mode"]
pub type UART_9BITADDR_ADDR_W<'a> = crate::FieldWriter<'a, u32, _9BITADDR_SPEC, u8, u8, 8, 0>;
#[doc = "Field `UART_9BITADDR_9BITEN` reader - Enable 9-Bit Mode"]
pub type UART_9BITADDR_9BITEN_R = crate::BitReader<bool>;
#[doc = "Field `UART_9BITADDR_9BITEN` writer - Enable 9-Bit Mode"]
pub type UART_9BITADDR_9BITEN_W<'a> = crate::BitWriter<'a, u32, _9BITADDR_SPEC, bool, 15>;
impl R {
    #[doc = "Bits 0:7 - Self Address for 9-Bit Mode"]
    #[inline(always)]
    pub fn uart_9bitaddr_addr(&self) -> UART_9BITADDR_ADDR_R {
        UART_9BITADDR_ADDR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 15 - Enable 9-Bit Mode"]
    #[inline(always)]
    pub fn uart_9bitaddr_9biten(&self) -> UART_9BITADDR_9BITEN_R {
        UART_9BITADDR_9BITEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Self Address for 9-Bit Mode"]
    #[inline(always)]
    pub fn uart_9bitaddr_addr(&mut self) -> UART_9BITADDR_ADDR_W {
        UART_9BITADDR_ADDR_W::new(self)
    }
    #[doc = "Bit 15 - Enable 9-Bit Mode"]
    #[inline(always)]
    pub fn uart_9bitaddr_9biten(&mut self) -> UART_9BITADDR_9BITEN_W {
        UART_9BITADDR_9BITEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART 9-Bit Self Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_9bitaddr](index.html) module"]
pub struct _9BITADDR_SPEC;
impl crate::RegisterSpec for _9BITADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_9bitaddr::R](R) reader structure"]
impl crate::Readable for _9BITADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_9bitaddr::W](W) writer structure"]
impl crate::Writable for _9BITADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _9BITADDR to value 0"]
impl crate::Resettable for _9BITADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
