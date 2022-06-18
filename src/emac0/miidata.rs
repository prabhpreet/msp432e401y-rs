#[doc = "Register `MIIDATA` reader"]
pub struct R(crate::R<MIIDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIIDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIIDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIIDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIIDATA` writer"]
pub struct W(crate::W<MIIDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIIDATA_SPEC>;
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
impl From<crate::W<MIIDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIIDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_MIIDATA_DATA` reader - MII Data"]
pub type EMAC_MIIDATA_DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EMAC_MIIDATA_DATA` writer - MII Data"]
pub type EMAC_MIIDATA_DATA_W<'a> = crate::FieldWriter<'a, u32, MIIDATA_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - MII Data"]
    #[inline(always)]
    pub fn emac_miidata_data(&self) -> EMAC_MIIDATA_DATA_R {
        EMAC_MIIDATA_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MII Data"]
    #[inline(always)]
    pub fn emac_miidata_data(&mut self) -> EMAC_MIIDATA_DATA_W {
        EMAC_MIIDATA_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC MII Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miidata](index.html) module"]
pub struct MIIDATA_SPEC;
impl crate::RegisterSpec for MIIDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [miidata::R](R) reader structure"]
impl crate::Readable for MIIDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [miidata::W](W) writer structure"]
impl crate::Writable for MIIDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIIDATA to value 0"]
impl crate::Resettable for MIIDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
