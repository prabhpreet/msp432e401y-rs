#[doc = "Register `CRCRSLTPP` reader"]
pub struct R(crate::R<CRCRSLTPP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCRSLTPP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCRSLTPP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCRSLTPP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCRSLTPP` writer"]
pub struct W(crate::W<CRCRSLTPP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCRSLTPP_SPEC>;
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
impl From<crate::W<CRCRSLTPP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCRSLTPP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC_RSLTPP_RSLTPP` reader - Post Processing Result"]
pub type CRC_RSLTPP_RSLTPP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CRC_RSLTPP_RSLTPP` writer - Post Processing Result"]
pub type CRC_RSLTPP_RSLTPP_W<'a> = crate::FieldWriter<'a, u32, CRCRSLTPP_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Post Processing Result"]
    #[inline(always)]
    pub fn crc_rsltpp_rsltpp(&self) -> CRC_RSLTPP_RSLTPP_R {
        CRC_RSLTPP_RSLTPP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Post Processing Result"]
    #[inline(always)]
    pub fn crc_rsltpp_rsltpp(&mut self) -> CRC_RSLTPP_RSLTPP_W {
        CRC_RSLTPP_RSLTPP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Post Processing Result\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcrsltpp](index.html) module"]
pub struct CRCRSLTPP_SPEC;
impl crate::RegisterSpec for CRCRSLTPP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crcrsltpp::R](R) reader structure"]
impl crate::Readable for CRCRSLTPP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crcrsltpp::W](W) writer structure"]
impl crate::Writable for CRCRSLTPP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRCRSLTPP to value 0"]
impl crate::Resettable for CRCRSLTPP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
