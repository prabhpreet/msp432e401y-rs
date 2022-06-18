#[doc = "Register `RWUFF` reader"]
pub struct R(crate::R<RWUFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RWUFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RWUFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RWUFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RWUFF` writer"]
pub struct W(crate::W<RWUFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RWUFF_SPEC>;
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
impl From<crate::W<RWUFF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RWUFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_RWUFF_WAKEUPFIL` reader - Remote Wake-Up Frame Filter"]
pub type EMAC_RWUFF_WAKEUPFIL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMAC_RWUFF_WAKEUPFIL` writer - Remote Wake-Up Frame Filter"]
pub type EMAC_RWUFF_WAKEUPFIL_W<'a> = crate::FieldWriter<'a, u32, RWUFF_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Remote Wake-Up Frame Filter"]
    #[inline(always)]
    pub fn emac_rwuff_wakeupfil(&self) -> EMAC_RWUFF_WAKEUPFIL_R {
        EMAC_RWUFF_WAKEUPFIL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Remote Wake-Up Frame Filter"]
    #[inline(always)]
    pub fn emac_rwuff_wakeupfil(&mut self) -> EMAC_RWUFF_WAKEUPFIL_W {
        EMAC_RWUFF_WAKEUPFIL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Remote Wake-Up Frame Filter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rwuff](index.html) module"]
pub struct RWUFF_SPEC;
impl crate::RegisterSpec for RWUFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rwuff::R](R) reader structure"]
impl crate::Readable for RWUFF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rwuff::W](W) writer structure"]
impl crate::Writable for RWUFF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RWUFF to value 0"]
impl crate::Resettable for RWUFF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
