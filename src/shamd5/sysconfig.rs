#[doc = "Register `SYSCONFIG` reader"]
pub struct R(crate::R<SYSCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCONFIG` writer"]
pub struct W(crate::W<SYSCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCONFIG_SPEC>;
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
impl From<crate::W<SYSCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHAMD5_SYSCONFIG_SOFTRESET` reader - Soft reset"]
pub type SHAMD5_SYSCONFIG_SOFTRESET_R = crate::BitReader<bool>;
#[doc = "Field `SHAMD5_SYSCONFIG_SOFTRESET` writer - Soft reset"]
pub type SHAMD5_SYSCONFIG_SOFTRESET_W<'a> = crate::BitWriter<'a, u32, SYSCONFIG_SPEC, bool, 1>;
#[doc = "Field `SHAMD5_SYSCONFIG_IT_EN` reader - Interrupt Enable"]
pub type SHAMD5_SYSCONFIG_IT_EN_R = crate::BitReader<bool>;
#[doc = "Field `SHAMD5_SYSCONFIG_IT_EN` writer - Interrupt Enable"]
pub type SHAMD5_SYSCONFIG_IT_EN_W<'a> = crate::BitWriter<'a, u32, SYSCONFIG_SPEC, bool, 2>;
#[doc = "Field `SHAMD5_SYSCONFIG_DMA_EN` reader - uDMA Request Enable"]
pub type SHAMD5_SYSCONFIG_DMA_EN_R = crate::BitReader<bool>;
#[doc = "Field `SHAMD5_SYSCONFIG_DMA_EN` writer - uDMA Request Enable"]
pub type SHAMD5_SYSCONFIG_DMA_EN_W<'a> = crate::BitWriter<'a, u32, SYSCONFIG_SPEC, bool, 3>;
#[doc = "Sidle mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SHAMD5_SYSCONFIG_SIDLE_A {
    #[doc = "0: Force-idle mode"]
    SHAMD5_SYSCONFIG_SIDLE_FORCE = 0,
}
impl From<SHAMD5_SYSCONFIG_SIDLE_A> for u8 {
    #[inline(always)]
    fn from(variant: SHAMD5_SYSCONFIG_SIDLE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SHAMD5_SYSCONFIG_SIDLE` reader - Sidle mode"]
pub type SHAMD5_SYSCONFIG_SIDLE_R = crate::FieldReader<u8, SHAMD5_SYSCONFIG_SIDLE_A>;
impl SHAMD5_SYSCONFIG_SIDLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SHAMD5_SYSCONFIG_SIDLE_A> {
        match self.bits {
            0 => Some(SHAMD5_SYSCONFIG_SIDLE_A::SHAMD5_SYSCONFIG_SIDLE_FORCE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SHAMD5_SYSCONFIG_SIDLE_FORCE`"]
    #[inline(always)]
    pub fn is_shamd5_sysconfig_sidle_force(&self) -> bool {
        *self == SHAMD5_SYSCONFIG_SIDLE_A::SHAMD5_SYSCONFIG_SIDLE_FORCE
    }
}
#[doc = "Field `SHAMD5_SYSCONFIG_SIDLE` writer - Sidle mode"]
pub type SHAMD5_SYSCONFIG_SIDLE_W<'a> =
    crate::FieldWriter<'a, u32, SYSCONFIG_SPEC, u8, SHAMD5_SYSCONFIG_SIDLE_A, 2, 4>;
impl<'a> SHAMD5_SYSCONFIG_SIDLE_W<'a> {
    #[doc = "Force-idle mode"]
    #[inline(always)]
    pub fn shamd5_sysconfig_sidle_force(self) -> &'a mut W {
        self.variant(SHAMD5_SYSCONFIG_SIDLE_A::SHAMD5_SYSCONFIG_SIDLE_FORCE)
    }
}
#[doc = "Field `SHAMD5_SYSCONFIG_SADVANCED` reader - Advanced Mode Enable"]
pub type SHAMD5_SYSCONFIG_SADVANCED_R = crate::BitReader<bool>;
#[doc = "Field `SHAMD5_SYSCONFIG_SADVANCED` writer - Advanced Mode Enable"]
pub type SHAMD5_SYSCONFIG_SADVANCED_W<'a> = crate::BitWriter<'a, u32, SYSCONFIG_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 1 - Soft reset"]
    #[inline(always)]
    pub fn shamd5_sysconfig_softreset(&self) -> SHAMD5_SYSCONFIG_SOFTRESET_R {
        SHAMD5_SYSCONFIG_SOFTRESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Enable"]
    #[inline(always)]
    pub fn shamd5_sysconfig_it_en(&self) -> SHAMD5_SYSCONFIG_IT_EN_R {
        SHAMD5_SYSCONFIG_IT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - uDMA Request Enable"]
    #[inline(always)]
    pub fn shamd5_sysconfig_dma_en(&self) -> SHAMD5_SYSCONFIG_DMA_EN_R {
        SHAMD5_SYSCONFIG_DMA_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Sidle mode"]
    #[inline(always)]
    pub fn shamd5_sysconfig_sidle(&self) -> SHAMD5_SYSCONFIG_SIDLE_R {
        SHAMD5_SYSCONFIG_SIDLE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Advanced Mode Enable"]
    #[inline(always)]
    pub fn shamd5_sysconfig_sadvanced(&self) -> SHAMD5_SYSCONFIG_SADVANCED_R {
        SHAMD5_SYSCONFIG_SADVANCED_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Soft reset"]
    #[inline(always)]
    pub fn shamd5_sysconfig_softreset(&mut self) -> SHAMD5_SYSCONFIG_SOFTRESET_W {
        SHAMD5_SYSCONFIG_SOFTRESET_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt Enable"]
    #[inline(always)]
    pub fn shamd5_sysconfig_it_en(&mut self) -> SHAMD5_SYSCONFIG_IT_EN_W {
        SHAMD5_SYSCONFIG_IT_EN_W::new(self)
    }
    #[doc = "Bit 3 - uDMA Request Enable"]
    #[inline(always)]
    pub fn shamd5_sysconfig_dma_en(&mut self) -> SHAMD5_SYSCONFIG_DMA_EN_W {
        SHAMD5_SYSCONFIG_DMA_EN_W::new(self)
    }
    #[doc = "Bits 4:5 - Sidle mode"]
    #[inline(always)]
    pub fn shamd5_sysconfig_sidle(&mut self) -> SHAMD5_SYSCONFIG_SIDLE_W {
        SHAMD5_SYSCONFIG_SIDLE_W::new(self)
    }
    #[doc = "Bit 7 - Advanced Mode Enable"]
    #[inline(always)]
    pub fn shamd5_sysconfig_sadvanced(&mut self) -> SHAMD5_SYSCONFIG_SADVANCED_W {
        SHAMD5_SYSCONFIG_SADVANCED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SHA System Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysconfig](index.html) module"]
pub struct SYSCONFIG_SPEC;
impl crate::RegisterSpec for SYSCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysconfig::R](R) reader structure"]
impl crate::Readable for SYSCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysconfig::W](W) writer structure"]
impl crate::Writable for SYSCONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSCONFIG to value 0"]
impl crate::Resettable for SYSCONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
