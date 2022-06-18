#[doc = "Register `DMASZ` reader"]
pub struct R(crate::R<DMASZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMASZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMASZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMASZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMASZ` writer"]
pub struct W(crate::W<DMASZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMASZ_SPEC>;
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
impl From<crate::W<DMASZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMASZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_DMASZ_SIZE` reader - uDMA-accessible Memory Size"]
pub type FLASH_DMASZ_SIZE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FLASH_DMASZ_SIZE` writer - uDMA-accessible Memory Size"]
pub type FLASH_DMASZ_SIZE_W<'a> = crate::FieldWriter<'a, u32, DMASZ_SPEC, u32, u32, 18, 0>;
impl R {
    #[doc = "Bits 0:17 - uDMA-accessible Memory Size"]
    #[inline(always)]
    pub fn flash_dmasz_size(&self) -> FLASH_DMASZ_SIZE_R {
        FLASH_DMASZ_SIZE_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - uDMA-accessible Memory Size"]
    #[inline(always)]
    pub fn flash_dmasz_size(&mut self) -> FLASH_DMASZ_SIZE_W {
        FLASH_DMASZ_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash DMA Address Size\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmasz](index.html) module"]
pub struct DMASZ_SPEC;
impl crate::RegisterSpec for DMASZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmasz::R](R) reader structure"]
impl crate::Readable for DMASZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmasz::W](W) writer structure"]
impl crate::Writable for DMASZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMASZ to value 0"]
impl crate::Resettable for DMASZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
