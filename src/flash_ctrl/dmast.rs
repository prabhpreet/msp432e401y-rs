#[doc = "Register `DMAST` reader"]
pub struct R(crate::R<DMAST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAST` writer"]
pub struct W(crate::W<DMAST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAST_SPEC>;
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
impl From<crate::W<DMAST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_DMAST_ADDR` reader - Contains the starting address of the flash region accessible by uDMA if the FLASHPP register DFA bit is set"]
pub type FLASH_DMAST_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FLASH_DMAST_ADDR` writer - Contains the starting address of the flash region accessible by uDMA if the FLASHPP register DFA bit is set"]
pub type FLASH_DMAST_ADDR_W<'a> = crate::FieldWriter<'a, u32, DMAST_SPEC, u32, u32, 18, 11>;
impl R {
    #[doc = "Bits 11:28 - Contains the starting address of the flash region accessible by uDMA if the FLASHPP register DFA bit is set"]
    #[inline(always)]
    pub fn flash_dmast_addr(&self) -> FLASH_DMAST_ADDR_R {
        FLASH_DMAST_ADDR_R::new(((self.bits >> 11) & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 11:28 - Contains the starting address of the flash region accessible by uDMA if the FLASHPP register DFA bit is set"]
    #[inline(always)]
    pub fn flash_dmast_addr(&mut self) -> FLASH_DMAST_ADDR_W {
        FLASH_DMAST_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash DMA Starting Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmast](index.html) module"]
pub struct DMAST_SPEC;
impl crate::RegisterSpec for DMAST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmast::R](R) reader structure"]
impl crate::Readable for DMAST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmast::W](W) writer structure"]
impl crate::Writable for DMAST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAST to value 0"]
impl crate::Resettable for DMAST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
