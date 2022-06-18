#[doc = "Register `USERREG0` reader"]
pub struct R(crate::R<USERREG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USERREG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USERREG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USERREG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USERREG0` writer"]
pub struct W(crate::W<USERREG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USERREG0_SPEC>;
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
impl From<crate::W<USERREG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USERREG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_USERREG0_DATA` reader - User Data"]
pub type FLASH_USERREG0_DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FLASH_USERREG0_DATA` writer - User Data"]
pub type FLASH_USERREG0_DATA_W<'a> = crate::FieldWriter<'a, u32, USERREG0_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - User Data"]
    #[inline(always)]
    pub fn flash_userreg0_data(&self) -> FLASH_USERREG0_DATA_R {
        FLASH_USERREG0_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User Data"]
    #[inline(always)]
    pub fn flash_userreg0_data(&mut self) -> FLASH_USERREG0_DATA_W {
        FLASH_USERREG0_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "User Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [userreg0](index.html) module"]
pub struct USERREG0_SPEC;
impl crate::RegisterSpec for USERREG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [userreg0::R](R) reader structure"]
impl crate::Readable for USERREG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [userreg0::W](W) writer structure"]
impl crate::Writable for USERREG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USERREG0 to value 0"]
impl crate::Resettable for USERREG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
