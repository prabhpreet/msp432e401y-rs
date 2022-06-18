#[doc = "Register `DIRTYBITS` reader"]
pub struct R(crate::R<DIRTYBITS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIRTYBITS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIRTYBITS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIRTYBITS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIRTYBITS` writer"]
pub struct W(crate::W<DIRTYBITS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIRTYBITS_SPEC>;
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
impl From<crate::W<DIRTYBITS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIRTYBITS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AES_DIRTYBITS_S_ACCESS` reader - AES Access Bit"]
pub type AES_DIRTYBITS_S_ACCESS_R = crate::BitReader<bool>;
#[doc = "Field `AES_DIRTYBITS_S_ACCESS` writer - AES Access Bit"]
pub type AES_DIRTYBITS_S_ACCESS_W<'a> = crate::BitWriter<'a, u32, DIRTYBITS_SPEC, bool, 0>;
#[doc = "Field `AES_DIRTYBITS_S_DIRTY` reader - AES Dirty Bit"]
pub type AES_DIRTYBITS_S_DIRTY_R = crate::BitReader<bool>;
#[doc = "Field `AES_DIRTYBITS_S_DIRTY` writer - AES Dirty Bit"]
pub type AES_DIRTYBITS_S_DIRTY_W<'a> = crate::BitWriter<'a, u32, DIRTYBITS_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - AES Access Bit"]
    #[inline(always)]
    pub fn aes_dirtybits_s_access(&self) -> AES_DIRTYBITS_S_ACCESS_R {
        AES_DIRTYBITS_S_ACCESS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AES Dirty Bit"]
    #[inline(always)]
    pub fn aes_dirtybits_s_dirty(&self) -> AES_DIRTYBITS_S_DIRTY_R {
        AES_DIRTYBITS_S_DIRTY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AES Access Bit"]
    #[inline(always)]
    pub fn aes_dirtybits_s_access(&mut self) -> AES_DIRTYBITS_S_ACCESS_W {
        AES_DIRTYBITS_S_ACCESS_W::new(self)
    }
    #[doc = "Bit 1 - AES Dirty Bit"]
    #[inline(always)]
    pub fn aes_dirtybits_s_dirty(&mut self) -> AES_DIRTYBITS_S_DIRTY_W {
        AES_DIRTYBITS_S_DIRTY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Dirty Bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dirtybits](index.html) module"]
pub struct DIRTYBITS_SPEC;
impl crate::RegisterSpec for DIRTYBITS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dirtybits::R](R) reader structure"]
impl crate::Readable for DIRTYBITS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dirtybits::W](W) writer structure"]
impl crate::Writable for DIRTYBITS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIRTYBITS to value 0"]
impl crate::Resettable for DIRTYBITS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
