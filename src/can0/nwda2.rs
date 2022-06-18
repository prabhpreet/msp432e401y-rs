#[doc = "Register `NWDA2` reader"]
pub struct R(crate::R<NWDA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NWDA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NWDA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NWDA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NWDA2` writer"]
pub struct W(crate::W<NWDA2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NWDA2_SPEC>;
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
impl From<crate::W<NWDA2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NWDA2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN_NWDA2_NEWDAT` reader - New Data Bits"]
pub type CAN_NWDA2_NEWDAT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CAN_NWDA2_NEWDAT` writer - New Data Bits"]
pub type CAN_NWDA2_NEWDAT_W<'a> = crate::FieldWriter<'a, u32, NWDA2_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - New Data Bits"]
    #[inline(always)]
    pub fn can_nwda2_newdat(&self) -> CAN_NWDA2_NEWDAT_R {
        CAN_NWDA2_NEWDAT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - New Data Bits"]
    #[inline(always)]
    pub fn can_nwda2_newdat(&mut self) -> CAN_NWDA2_NEWDAT_W {
        CAN_NWDA2_NEWDAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN New Data 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nwda2](index.html) module"]
pub struct NWDA2_SPEC;
impl crate::RegisterSpec for NWDA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nwda2::R](R) reader structure"]
impl crate::Readable for NWDA2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nwda2::W](W) writer structure"]
impl crate::Writable for NWDA2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NWDA2 to value 0"]
impl crate::Resettable for NWDA2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
