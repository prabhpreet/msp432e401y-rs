#[doc = "Register `TEST` reader"]
pub struct R(crate::R<TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEST` writer"]
pub struct W(crate::W<TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST_SPEC>;
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
impl From<crate::W<TEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_TEST_TESTSE0NAK` reader - Test_SE0_NAK Test Mode Enable"]
pub type USB_TEST_TESTSE0NAK_R = crate::BitReader<bool>;
#[doc = "Field `USB_TEST_TESTSE0NAK` writer - Test_SE0_NAK Test Mode Enable"]
pub type USB_TEST_TESTSE0NAK_W<'a> = crate::BitWriter<'a, u8, TEST_SPEC, bool, 0>;
#[doc = "Field `USB_TEST_TESTJ` reader - Test_J Mode Enable"]
pub type USB_TEST_TESTJ_R = crate::BitReader<bool>;
#[doc = "Field `USB_TEST_TESTJ` writer - Test_J Mode Enable"]
pub type USB_TEST_TESTJ_W<'a> = crate::BitWriter<'a, u8, TEST_SPEC, bool, 1>;
#[doc = "Field `USB_TEST_TESTK` reader - Test_K Mode Enable"]
pub type USB_TEST_TESTK_R = crate::BitReader<bool>;
#[doc = "Field `USB_TEST_TESTK` writer - Test_K Mode Enable"]
pub type USB_TEST_TESTK_W<'a> = crate::BitWriter<'a, u8, TEST_SPEC, bool, 2>;
#[doc = "Field `USB_TEST_TESTPKT` reader - Test Packet Mode Enable"]
pub type USB_TEST_TESTPKT_R = crate::BitReader<bool>;
#[doc = "Field `USB_TEST_TESTPKT` writer - Test Packet Mode Enable"]
pub type USB_TEST_TESTPKT_W<'a> = crate::BitWriter<'a, u8, TEST_SPEC, bool, 3>;
#[doc = "Field `USB_TEST_FORCEHS` reader - Force High-Speed Mode"]
pub type USB_TEST_FORCEHS_R = crate::BitReader<bool>;
#[doc = "Field `USB_TEST_FORCEHS` writer - Force High-Speed Mode"]
pub type USB_TEST_FORCEHS_W<'a> = crate::BitWriter<'a, u8, TEST_SPEC, bool, 4>;
#[doc = "Field `USB_TEST_FORCEFS` reader - Force Full-Speed Mode"]
pub type USB_TEST_FORCEFS_R = crate::BitReader<bool>;
#[doc = "Field `USB_TEST_FORCEFS` writer - Force Full-Speed Mode"]
pub type USB_TEST_FORCEFS_W<'a> = crate::BitWriter<'a, u8, TEST_SPEC, bool, 5>;
#[doc = "Field `USB_TEST_FIFOACC` reader - FIFO Access"]
pub type USB_TEST_FIFOACC_R = crate::BitReader<bool>;
#[doc = "Field `USB_TEST_FIFOACC` writer - FIFO Access"]
pub type USB_TEST_FIFOACC_W<'a> = crate::BitWriter<'a, u8, TEST_SPEC, bool, 6>;
#[doc = "Field `USB_TEST_FORCEH` reader - Force Host Mode"]
pub type USB_TEST_FORCEH_R = crate::BitReader<bool>;
#[doc = "Field `USB_TEST_FORCEH` writer - Force Host Mode"]
pub type USB_TEST_FORCEH_W<'a> = crate::BitWriter<'a, u8, TEST_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Test_SE0_NAK Test Mode Enable"]
    #[inline(always)]
    pub fn usb_test_testse0nak(&self) -> USB_TEST_TESTSE0NAK_R {
        USB_TEST_TESTSE0NAK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Test_J Mode Enable"]
    #[inline(always)]
    pub fn usb_test_testj(&self) -> USB_TEST_TESTJ_R {
        USB_TEST_TESTJ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Test_K Mode Enable"]
    #[inline(always)]
    pub fn usb_test_testk(&self) -> USB_TEST_TESTK_R {
        USB_TEST_TESTK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Test Packet Mode Enable"]
    #[inline(always)]
    pub fn usb_test_testpkt(&self) -> USB_TEST_TESTPKT_R {
        USB_TEST_TESTPKT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Force High-Speed Mode"]
    #[inline(always)]
    pub fn usb_test_forcehs(&self) -> USB_TEST_FORCEHS_R {
        USB_TEST_FORCEHS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Force Full-Speed Mode"]
    #[inline(always)]
    pub fn usb_test_forcefs(&self) -> USB_TEST_FORCEFS_R {
        USB_TEST_FORCEFS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FIFO Access"]
    #[inline(always)]
    pub fn usb_test_fifoacc(&self) -> USB_TEST_FIFOACC_R {
        USB_TEST_FIFOACC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Force Host Mode"]
    #[inline(always)]
    pub fn usb_test_forceh(&self) -> USB_TEST_FORCEH_R {
        USB_TEST_FORCEH_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Test_SE0_NAK Test Mode Enable"]
    #[inline(always)]
    pub fn usb_test_testse0nak(&mut self) -> USB_TEST_TESTSE0NAK_W {
        USB_TEST_TESTSE0NAK_W::new(self)
    }
    #[doc = "Bit 1 - Test_J Mode Enable"]
    #[inline(always)]
    pub fn usb_test_testj(&mut self) -> USB_TEST_TESTJ_W {
        USB_TEST_TESTJ_W::new(self)
    }
    #[doc = "Bit 2 - Test_K Mode Enable"]
    #[inline(always)]
    pub fn usb_test_testk(&mut self) -> USB_TEST_TESTK_W {
        USB_TEST_TESTK_W::new(self)
    }
    #[doc = "Bit 3 - Test Packet Mode Enable"]
    #[inline(always)]
    pub fn usb_test_testpkt(&mut self) -> USB_TEST_TESTPKT_W {
        USB_TEST_TESTPKT_W::new(self)
    }
    #[doc = "Bit 4 - Force High-Speed Mode"]
    #[inline(always)]
    pub fn usb_test_forcehs(&mut self) -> USB_TEST_FORCEHS_W {
        USB_TEST_FORCEHS_W::new(self)
    }
    #[doc = "Bit 5 - Force Full-Speed Mode"]
    #[inline(always)]
    pub fn usb_test_forcefs(&mut self) -> USB_TEST_FORCEFS_W {
        USB_TEST_FORCEFS_W::new(self)
    }
    #[doc = "Bit 6 - FIFO Access"]
    #[inline(always)]
    pub fn usb_test_fifoacc(&mut self) -> USB_TEST_FIFOACC_W {
        USB_TEST_FIFOACC_W::new(self)
    }
    #[doc = "Bit 7 - Force Host Mode"]
    #[inline(always)]
    pub fn usb_test_forceh(&mut self) -> USB_TEST_FORCEH_W {
        USB_TEST_FORCEH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Test Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test](index.html) module"]
pub struct TEST_SPEC;
impl crate::RegisterSpec for TEST_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [test::R](R) reader structure"]
impl crate::Readable for TEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [test::W](W) writer structure"]
impl crate::Writable for TEST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TEST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
