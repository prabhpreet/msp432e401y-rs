#[doc = "Register `TXFIFOADD` reader"]
pub struct R(crate::R<TXFIFOADD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXFIFOADD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXFIFOADD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXFIFOADD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXFIFOADD` writer"]
pub struct W(crate::W<TXFIFOADD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXFIFOADD_SPEC>;
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
impl From<crate::W<TXFIFOADD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXFIFOADD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_TXFIFOADD_ADDR` reader - Transmit/Receive Start Address"]
pub type USB_TXFIFOADD_ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `USB_TXFIFOADD_ADDR` writer - Transmit/Receive Start Address"]
pub type USB_TXFIFOADD_ADDR_W<'a> = crate::FieldWriter<'a, u16, TXFIFOADD_SPEC, u16, u16, 9, 0>;
impl R {
    #[doc = "Bits 0:8 - Transmit/Receive Start Address"]
    #[inline(always)]
    pub fn usb_txfifoadd_addr(&self) -> USB_TXFIFOADD_ADDR_R {
        USB_TXFIFOADD_ADDR_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Transmit/Receive Start Address"]
    #[inline(always)]
    pub fn usb_txfifoadd_addr(&mut self) -> USB_TXFIFOADD_ADDR_W {
        USB_TXFIFOADD_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Transmit FIFO Start Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfifoadd](index.html) module"]
pub struct TXFIFOADD_SPEC;
impl crate::RegisterSpec for TXFIFOADD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [txfifoadd::R](R) reader structure"]
impl crate::Readable for TXFIFOADD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txfifoadd::W](W) writer structure"]
impl crate::Writable for TXFIFOADD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXFIFOADD to value 0"]
impl crate::Resettable for TXFIFOADD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
