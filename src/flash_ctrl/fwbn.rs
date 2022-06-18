#[doc = "Register `FWBN[%s]` reader"]
pub struct R(crate::R<FWBN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FWBN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FWBN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FWBN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FWBN[%s]` writer"]
pub struct W(crate::W<FWBN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FWBN_SPEC>;
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
impl From<crate::W<FWBN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FWBN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_FWBN_DATA` reader - Data"]
pub type FLASH_FWBN_DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FLASH_FWBN_DATA` writer - Data"]
pub type FLASH_FWBN_DATA_W<'a> = crate::FieldWriter<'a, u32, FWBN_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn flash_fwbn_data(&self) -> FLASH_FWBN_DATA_R {
        FLASH_FWBN_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn flash_fwbn_data(&mut self) -> FLASH_FWBN_DATA_W {
        FLASH_FWBN_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Write Buffer n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwbn](index.html) module"]
pub struct FWBN_SPEC;
impl crate::RegisterSpec for FWBN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fwbn::R](R) reader structure"]
impl crate::Readable for FWBN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fwbn::W](W) writer structure"]
impl crate::Writable for FWBN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FWBN[%s]
to value 0"]
impl crate::Resettable for FWBN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
