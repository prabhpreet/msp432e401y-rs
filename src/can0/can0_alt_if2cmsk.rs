#[doc = "Register `IF2CMSK` reader"]
pub struct R(crate::R<CAN0_ALT_IF2CMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN0_ALT_IF2CMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAN0_ALT_IF2CMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAN0_ALT_IF2CMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IF2CMSK` writer"]
pub struct W(crate::W<CAN0_ALT_IF2CMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN0_ALT_IF2CMSK_SPEC>;
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
impl From<crate::W<CAN0_ALT_IF2CMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAN0_ALT_IF2CMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN_IF2CMSK_TXRQST` reader - Access Transmission Request"]
pub type CAN_IF2CMSK_TXRQST_R = crate::BitReader<bool>;
#[doc = "Field `CAN_IF2CMSK_TXRQST` writer - Access Transmission Request"]
pub type CAN_IF2CMSK_TXRQST_W<'a> = crate::BitWriter<'a, u32, CAN0_ALT_IF2CMSK_SPEC, bool, 2>;
impl R {
    #[doc = "Bit 2 - Access Transmission Request"]
    #[inline(always)]
    pub fn can_if2cmsk_txrqst(&self) -> CAN_IF2CMSK_TXRQST_R {
        CAN_IF2CMSK_TXRQST_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Access Transmission Request"]
    #[inline(always)]
    pub fn can_if2cmsk_txrqst(&mut self) -> CAN_IF2CMSK_TXRQST_W {
        CAN_IF2CMSK_TXRQST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN IF2 Command Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can0_alt_if2cmsk](index.html) module"]
pub struct CAN0_ALT_IF2CMSK_SPEC;
impl crate::RegisterSpec for CAN0_ALT_IF2CMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can0_alt_if2cmsk::R](R) reader structure"]
impl crate::Readable for CAN0_ALT_IF2CMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can0_alt_if2cmsk::W](W) writer structure"]
impl crate::Writable for CAN0_ALT_IF2CMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IF2CMSK to value 0"]
impl crate::Resettable for CAN0_ALT_IF2CMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
