#[doc = "Register `NWDA1` reader"]
pub struct R(crate::R<NWDA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NWDA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NWDA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NWDA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NWDA1` writer"]
pub struct W(crate::W<NWDA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NWDA1_SPEC>;
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
impl From<crate::W<NWDA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NWDA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN_NWDA1_NEWDAT` reader - New Data Bits"]
pub type CAN_NWDA1_NEWDAT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CAN_NWDA1_NEWDAT` writer - New Data Bits"]
pub type CAN_NWDA1_NEWDAT_W<'a> = crate::FieldWriter<'a, u32, NWDA1_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - New Data Bits"]
    #[inline(always)]
    pub fn can_nwda1_newdat(&self) -> CAN_NWDA1_NEWDAT_R {
        CAN_NWDA1_NEWDAT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - New Data Bits"]
    #[inline(always)]
    pub fn can_nwda1_newdat(&mut self) -> CAN_NWDA1_NEWDAT_W {
        CAN_NWDA1_NEWDAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN New Data 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nwda1](index.html) module"]
pub struct NWDA1_SPEC;
impl crate::RegisterSpec for NWDA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nwda1::R](R) reader structure"]
impl crate::Readable for NWDA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nwda1::W](W) writer structure"]
impl crate::Writable for NWDA1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NWDA1 to value 0"]
impl crate::Resettable for NWDA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
