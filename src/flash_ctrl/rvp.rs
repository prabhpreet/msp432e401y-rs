#[doc = "Register `RVP` reader"]
pub struct R(crate::R<RVP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RVP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RVP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RVP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RVP` writer"]
pub struct W(crate::W<RVP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RVP_SPEC>;
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
impl From<crate::W<RVP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RVP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_RVP_RV` reader - Reset Vector Pointer Address"]
pub type FLASH_RVP_RV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FLASH_RVP_RV` writer - Reset Vector Pointer Address"]
pub type FLASH_RVP_RV_W<'a> = crate::FieldWriter<'a, u32, RVP_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Reset Vector Pointer Address"]
    #[inline(always)]
    pub fn flash_rvp_rv(&self) -> FLASH_RVP_RV_R {
        FLASH_RVP_RV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reset Vector Pointer Address"]
    #[inline(always)]
    pub fn flash_rvp_rv(&mut self) -> FLASH_RVP_RV_W {
        FLASH_RVP_RV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Vector Pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rvp](index.html) module"]
pub struct RVP_SPEC;
impl crate::RegisterSpec for RVP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rvp::R](R) reader structure"]
impl crate::Readable for RVP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rvp::W](W) writer structure"]
impl crate::Writable for RVP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RVP to value 0"]
impl crate::Resettable for RVP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
