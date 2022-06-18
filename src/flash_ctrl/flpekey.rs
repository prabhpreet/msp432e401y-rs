#[doc = "Register `FLPEKEY` reader"]
pub struct R(crate::R<FLPEKEY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLPEKEY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLPEKEY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLPEKEY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLPEKEY` writer"]
pub struct W(crate::W<FLPEKEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLPEKEY_SPEC>;
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
impl From<crate::W<FLPEKEY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLPEKEY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_FLPEKEY_PEKEY` reader - Key Value"]
pub type FLASH_FLPEKEY_PEKEY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FLASH_FLPEKEY_PEKEY` writer - Key Value"]
pub type FLASH_FLPEKEY_PEKEY_W<'a> = crate::FieldWriter<'a, u32, FLPEKEY_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Key Value"]
    #[inline(always)]
    pub fn flash_flpekey_pekey(&self) -> FLASH_FLPEKEY_PEKEY_R {
        FLASH_FLPEKEY_PEKEY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Key Value"]
    #[inline(always)]
    pub fn flash_flpekey_pekey(&mut self) -> FLASH_FLPEKEY_PEKEY_W {
        FLASH_FLPEKEY_PEKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Program/Erase Key\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flpekey](index.html) module"]
pub struct FLPEKEY_SPEC;
impl crate::RegisterSpec for FLPEKEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flpekey::R](R) reader structure"]
impl crate::Readable for FLPEKEY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flpekey::W](W) writer structure"]
impl crate::Writable for FLPEKEY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLPEKEY to value 0"]
impl crate::Resettable for FLPEKEY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
