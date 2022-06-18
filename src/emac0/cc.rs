#[doc = "Register `CC` reader"]
pub struct R(crate::R<CC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CC` writer"]
pub struct W(crate::W<CC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CC_SPEC>;
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
impl From<crate::W<CC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_CC_POL` reader - LED Polarity Control"]
pub type EMAC_CC_POL_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_CC_POL` writer - LED Polarity Control"]
pub type EMAC_CC_POL_W<'a> = crate::BitWriter<'a, u32, CC_SPEC, bool, 17>;
#[doc = "Field `EMAC_CC_PTPCEN` reader - PTP Clock Reference Enable"]
pub type EMAC_CC_PTPCEN_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_CC_PTPCEN` writer - PTP Clock Reference Enable"]
pub type EMAC_CC_PTPCEN_W<'a> = crate::BitWriter<'a, u32, CC_SPEC, bool, 18>;
impl R {
    #[doc = "Bit 17 - LED Polarity Control"]
    #[inline(always)]
    pub fn emac_cc_pol(&self) -> EMAC_CC_POL_R {
        EMAC_CC_POL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PTP Clock Reference Enable"]
    #[inline(always)]
    pub fn emac_cc_ptpcen(&self) -> EMAC_CC_PTPCEN_R {
        EMAC_CC_PTPCEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - LED Polarity Control"]
    #[inline(always)]
    pub fn emac_cc_pol(&mut self) -> EMAC_CC_POL_W {
        EMAC_CC_POL_W::new(self)
    }
    #[doc = "Bit 18 - PTP Clock Reference Enable"]
    #[inline(always)]
    pub fn emac_cc_ptpcen(&mut self) -> EMAC_CC_PTPCEN_W {
        EMAC_CC_PTPCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc](index.html) module"]
pub struct CC_SPEC;
impl crate::RegisterSpec for CC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc::R](R) reader structure"]
impl crate::Readable for CC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cc::W](W) writer structure"]
impl crate::Writable for CC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CC to value 0"]
impl crate::Resettable for CC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
