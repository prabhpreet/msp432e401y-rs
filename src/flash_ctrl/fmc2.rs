#[doc = "Register `FMC2` reader"]
pub struct R(crate::R<FMC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC2` writer"]
pub struct W(crate::W<FMC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC2_SPEC>;
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
impl From<crate::W<FMC2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_FMC2_WRBUF` reader - Buffered Flash Memory Write"]
pub type FLASH_FMC2_WRBUF_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_FMC2_WRBUF` writer - Buffered Flash Memory Write"]
pub type FLASH_FMC2_WRBUF_W<'a> = crate::BitWriter<'a, u32, FMC2_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Buffered Flash Memory Write"]
    #[inline(always)]
    pub fn flash_fmc2_wrbuf(&self) -> FLASH_FMC2_WRBUF_R {
        FLASH_FMC2_WRBUF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Buffered Flash Memory Write"]
    #[inline(always)]
    pub fn flash_fmc2_wrbuf(&mut self) -> FLASH_FMC2_WRBUF_W {
        FLASH_FMC2_WRBUF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Memory Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc2](index.html) module"]
pub struct FMC2_SPEC;
impl crate::RegisterSpec for FMC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc2::R](R) reader structure"]
impl crate::Readable for FMC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc2::W](W) writer structure"]
impl crate::Writable for FMC2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC2 to value 0"]
impl crate::Resettable for FMC2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
