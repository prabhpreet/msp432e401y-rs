#[doc = "Register `USERREG2` reader"]
pub struct R(crate::R<USERREG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USERREG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USERREG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USERREG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USERREG2` writer"]
pub struct W(crate::W<USERREG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USERREG2_SPEC>;
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
impl From<crate::W<USERREG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USERREG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_USERREG2_DATA` reader - User Data"]
pub type FLASH_USERREG2_DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FLASH_USERREG2_DATA` writer - User Data"]
pub type FLASH_USERREG2_DATA_W<'a> = crate::FieldWriter<'a, u32, USERREG2_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - User Data"]
    #[inline(always)]
    pub fn flash_userreg2_data(&self) -> FLASH_USERREG2_DATA_R {
        FLASH_USERREG2_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User Data"]
    #[inline(always)]
    pub fn flash_userreg2_data(&mut self) -> FLASH_USERREG2_DATA_W {
        FLASH_USERREG2_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "User Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [userreg2](index.html) module"]
pub struct USERREG2_SPEC;
impl crate::RegisterSpec for USERREG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [userreg2::R](R) reader structure"]
impl crate::Readable for USERREG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [userreg2::W](W) writer structure"]
impl crate::Writable for USERREG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USERREG2 to value 0"]
impl crate::Resettable for USERREG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
