#[doc = "Register `PPS0WIDTH` reader"]
pub struct R(crate::R<PPS0WIDTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPS0WIDTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPS0WIDTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPS0WIDTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPS0WIDTH` writer"]
pub struct W(crate::W<PPS0WIDTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPS0WIDTH_SPEC>;
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
impl From<crate::W<PPS0WIDTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPS0WIDTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_PPS0WIDTH` reader - EN0PPS Output Signal Width"]
pub type EMAC_PPS0WIDTH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMAC_PPS0WIDTH` writer - EN0PPS Output Signal Width"]
pub type EMAC_PPS0WIDTH_W<'a> = crate::FieldWriter<'a, u32, PPS0WIDTH_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - EN0PPS Output Signal Width"]
    #[inline(always)]
    pub fn emac_pps0width(&self) -> EMAC_PPS0WIDTH_R {
        EMAC_PPS0WIDTH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - EN0PPS Output Signal Width"]
    #[inline(always)]
    pub fn emac_pps0width(&mut self) -> EMAC_PPS0WIDTH_W {
        EMAC_PPS0WIDTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC PPS0 Width\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pps0width](index.html) module"]
pub struct PPS0WIDTH_SPEC;
impl crate::RegisterSpec for PPS0WIDTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pps0width::R](R) reader structure"]
impl crate::Readable for PPS0WIDTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pps0width::W](W) writer structure"]
impl crate::Writable for PPS0WIDTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PPS0WIDTH to value 0"]
impl crate::Resettable for PPS0WIDTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
