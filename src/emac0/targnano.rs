#[doc = "Register `TARGNANO` reader"]
pub struct R(crate::R<TARGNANO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TARGNANO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TARGNANO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TARGNANO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TARGNANO` writer"]
pub struct W(crate::W<TARGNANO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TARGNANO_SPEC>;
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
impl From<crate::W<TARGNANO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TARGNANO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_TARGNANO_TTSLO` reader - Target Timestamp Low Register"]
pub type EMAC_TARGNANO_TTSLO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMAC_TARGNANO_TTSLO` writer - Target Timestamp Low Register"]
pub type EMAC_TARGNANO_TTSLO_W<'a> = crate::FieldWriter<'a, u32, TARGNANO_SPEC, u32, u32, 31, 0>;
#[doc = "Field `EMAC_TARGNANO_TRGTBUSY` reader - Target Time Register Busy"]
pub type EMAC_TARGNANO_TRGTBUSY_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_TARGNANO_TRGTBUSY` writer - Target Time Register Busy"]
pub type EMAC_TARGNANO_TRGTBUSY_W<'a> = crate::BitWriter<'a, u32, TARGNANO_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:30 - Target Timestamp Low Register"]
    #[inline(always)]
    pub fn emac_targnano_ttslo(&self) -> EMAC_TARGNANO_TTSLO_R {
        EMAC_TARGNANO_TTSLO_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - Target Time Register Busy"]
    #[inline(always)]
    pub fn emac_targnano_trgtbusy(&self) -> EMAC_TARGNANO_TRGTBUSY_R {
        EMAC_TARGNANO_TRGTBUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Target Timestamp Low Register"]
    #[inline(always)]
    pub fn emac_targnano_ttslo(&mut self) -> EMAC_TARGNANO_TTSLO_W {
        EMAC_TARGNANO_TTSLO_W::new(self)
    }
    #[doc = "Bit 31 - Target Time Register Busy"]
    #[inline(always)]
    pub fn emac_targnano_trgtbusy(&mut self) -> EMAC_TARGNANO_TRGTBUSY_W {
        EMAC_TARGNANO_TRGTBUSY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Target Time Nanoseconds\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [targnano](index.html) module"]
pub struct TARGNANO_SPEC;
impl crate::RegisterSpec for TARGNANO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [targnano::R](R) reader structure"]
impl crate::Readable for TARGNANO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [targnano::W](W) writer structure"]
impl crate::Writable for TARGNANO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TARGNANO to value 0"]
impl crate::Resettable for TARGNANO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
