#[doc = "Register `FMC` reader"]
pub struct R(crate::R<FMC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC` writer"]
pub struct W(crate::W<FMC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_SPEC>;
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
impl From<crate::W<FMC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_FMC_WRITE` reader - Write a Word into Flash Memory"]
pub type FLASH_FMC_WRITE_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_FMC_WRITE` writer - Write a Word into Flash Memory"]
pub type FLASH_FMC_WRITE_W<'a> = crate::BitWriter<'a, u32, FMC_SPEC, bool, 0>;
#[doc = "Field `FLASH_FMC_ERASE` reader - Erase a Page of Flash Memory"]
pub type FLASH_FMC_ERASE_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_FMC_ERASE` writer - Erase a Page of Flash Memory"]
pub type FLASH_FMC_ERASE_W<'a> = crate::BitWriter<'a, u32, FMC_SPEC, bool, 1>;
#[doc = "Field `FLASH_FMC_MERASE` reader - Mass Erase Flash Memory"]
pub type FLASH_FMC_MERASE_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_FMC_MERASE` writer - Mass Erase Flash Memory"]
pub type FLASH_FMC_MERASE_W<'a> = crate::BitWriter<'a, u32, FMC_SPEC, bool, 2>;
#[doc = "Field `FLASH_FMC_COMT` reader - Commit Register Value"]
pub type FLASH_FMC_COMT_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_FMC_COMT` writer - Commit Register Value"]
pub type FLASH_FMC_COMT_W<'a> = crate::BitWriter<'a, u32, FMC_SPEC, bool, 3>;
#[doc = "Field `FLASH_FMC_WRKEY` reader - FLASH write key"]
pub type FLASH_FMC_WRKEY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FLASH_FMC_WRKEY` writer - FLASH write key"]
pub type FLASH_FMC_WRKEY_W<'a> = crate::FieldWriter<'a, u32, FMC_SPEC, u16, u16, 15, 17>;
impl R {
    #[doc = "Bit 0 - Write a Word into Flash Memory"]
    #[inline(always)]
    pub fn flash_fmc_write(&self) -> FLASH_FMC_WRITE_R {
        FLASH_FMC_WRITE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Erase a Page of Flash Memory"]
    #[inline(always)]
    pub fn flash_fmc_erase(&self) -> FLASH_FMC_ERASE_R {
        FLASH_FMC_ERASE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mass Erase Flash Memory"]
    #[inline(always)]
    pub fn flash_fmc_merase(&self) -> FLASH_FMC_MERASE_R {
        FLASH_FMC_MERASE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Commit Register Value"]
    #[inline(always)]
    pub fn flash_fmc_comt(&self) -> FLASH_FMC_COMT_R {
        FLASH_FMC_COMT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 17:31 - FLASH write key"]
    #[inline(always)]
    pub fn flash_fmc_wrkey(&self) -> FLASH_FMC_WRKEY_R {
        FLASH_FMC_WRKEY_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Write a Word into Flash Memory"]
    #[inline(always)]
    pub fn flash_fmc_write(&mut self) -> FLASH_FMC_WRITE_W {
        FLASH_FMC_WRITE_W::new(self)
    }
    #[doc = "Bit 1 - Erase a Page of Flash Memory"]
    #[inline(always)]
    pub fn flash_fmc_erase(&mut self) -> FLASH_FMC_ERASE_W {
        FLASH_FMC_ERASE_W::new(self)
    }
    #[doc = "Bit 2 - Mass Erase Flash Memory"]
    #[inline(always)]
    pub fn flash_fmc_merase(&mut self) -> FLASH_FMC_MERASE_W {
        FLASH_FMC_MERASE_W::new(self)
    }
    #[doc = "Bit 3 - Commit Register Value"]
    #[inline(always)]
    pub fn flash_fmc_comt(&mut self) -> FLASH_FMC_COMT_W {
        FLASH_FMC_COMT_W::new(self)
    }
    #[doc = "Bits 17:31 - FLASH write key"]
    #[inline(always)]
    pub fn flash_fmc_wrkey(&mut self) -> FLASH_FMC_WRKEY_W {
        FLASH_FMC_WRKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Memory Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc](index.html) module"]
pub struct FMC_SPEC;
impl crate::RegisterSpec for FMC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc::R](R) reader structure"]
impl crate::Readable for FMC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc::W](W) writer structure"]
impl crate::Writable for FMC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC to value 0"]
impl crate::Resettable for FMC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
