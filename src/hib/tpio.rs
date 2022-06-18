#[doc = "Register `TPIO` reader"]
pub struct R(crate::R<TPIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPIO` writer"]
pub struct W(crate::W<TPIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPIO_SPEC>;
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
impl From<crate::W<TPIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIB_TPIO_EN0` reader - TMPR0 Enable"]
pub type HIB_TPIO_EN0_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TPIO_EN0` writer - TMPR0 Enable"]
pub type HIB_TPIO_EN0_W<'a> = crate::BitWriter<'a, u32, TPIO_SPEC, bool, 0>;
#[doc = "Field `HIB_TPIO_LEV0` reader - TMPR0 Trigger Level"]
pub type HIB_TPIO_LEV0_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TPIO_LEV0` writer - TMPR0 Trigger Level"]
pub type HIB_TPIO_LEV0_W<'a> = crate::BitWriter<'a, u32, TPIO_SPEC, bool, 1>;
#[doc = "Field `HIB_TPIO_PUEN0` reader - TMPR0 Internal Weak Pull-up Enable"]
pub type HIB_TPIO_PUEN0_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TPIO_PUEN0` writer - TMPR0 Internal Weak Pull-up Enable"]
pub type HIB_TPIO_PUEN0_W<'a> = crate::BitWriter<'a, u32, TPIO_SPEC, bool, 2>;
#[doc = "Field `HIB_TPIO_GFLTR0` reader - TMPR0 Glitch Filtering"]
pub type HIB_TPIO_GFLTR0_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TPIO_GFLTR0` writer - TMPR0 Glitch Filtering"]
pub type HIB_TPIO_GFLTR0_W<'a> = crate::BitWriter<'a, u32, TPIO_SPEC, bool, 3>;
#[doc = "Field `HIB_TPIO_EN1` reader - TMPR1Enable"]
pub type HIB_TPIO_EN1_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TPIO_EN1` writer - TMPR1Enable"]
pub type HIB_TPIO_EN1_W<'a> = crate::BitWriter<'a, u32, TPIO_SPEC, bool, 8>;
#[doc = "Field `HIB_TPIO_LEV1` reader - TMPR1 Trigger Level"]
pub type HIB_TPIO_LEV1_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TPIO_LEV1` writer - TMPR1 Trigger Level"]
pub type HIB_TPIO_LEV1_W<'a> = crate::BitWriter<'a, u32, TPIO_SPEC, bool, 9>;
#[doc = "Field `HIB_TPIO_PUEN1` reader - TMPR1 Internal Weak Pull-up Enable"]
pub type HIB_TPIO_PUEN1_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TPIO_PUEN1` writer - TMPR1 Internal Weak Pull-up Enable"]
pub type HIB_TPIO_PUEN1_W<'a> = crate::BitWriter<'a, u32, TPIO_SPEC, bool, 10>;
#[doc = "Field `HIB_TPIO_GFLTR1` reader - TMPR1 Glitch Filtering"]
pub type HIB_TPIO_GFLTR1_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TPIO_GFLTR1` writer - TMPR1 Glitch Filtering"]
pub type HIB_TPIO_GFLTR1_W<'a> = crate::BitWriter<'a, u32, TPIO_SPEC, bool, 11>;
#[doc = "Field `HIB_TPIO_EN2` reader - TMPR2 Enable"]
pub type HIB_TPIO_EN2_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TPIO_EN2` writer - TMPR2 Enable"]
pub type HIB_TPIO_EN2_W<'a> = crate::BitWriter<'a, u32, TPIO_SPEC, bool, 16>;
#[doc = "Field `HIB_TPIO_LEV2` reader - TMPR2 Trigger Level"]
pub type HIB_TPIO_LEV2_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TPIO_LEV2` writer - TMPR2 Trigger Level"]
pub type HIB_TPIO_LEV2_W<'a> = crate::BitWriter<'a, u32, TPIO_SPEC, bool, 17>;
#[doc = "Field `HIB_TPIO_PUEN2` reader - TMPR2 Internal Weak Pull-up Enable"]
pub type HIB_TPIO_PUEN2_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TPIO_PUEN2` writer - TMPR2 Internal Weak Pull-up Enable"]
pub type HIB_TPIO_PUEN2_W<'a> = crate::BitWriter<'a, u32, TPIO_SPEC, bool, 18>;
#[doc = "Field `HIB_TPIO_GFLTR2` reader - TMPR2 Glitch Filtering"]
pub type HIB_TPIO_GFLTR2_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TPIO_GFLTR2` writer - TMPR2 Glitch Filtering"]
pub type HIB_TPIO_GFLTR2_W<'a> = crate::BitWriter<'a, u32, TPIO_SPEC, bool, 19>;
#[doc = "Field `HIB_TPIO_EN3` reader - TMPR3 Enable"]
pub type HIB_TPIO_EN3_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TPIO_EN3` writer - TMPR3 Enable"]
pub type HIB_TPIO_EN3_W<'a> = crate::BitWriter<'a, u32, TPIO_SPEC, bool, 24>;
#[doc = "Field `HIB_TPIO_LEV3` reader - TMPR3 Trigger Level"]
pub type HIB_TPIO_LEV3_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TPIO_LEV3` writer - TMPR3 Trigger Level"]
pub type HIB_TPIO_LEV3_W<'a> = crate::BitWriter<'a, u32, TPIO_SPEC, bool, 25>;
#[doc = "Field `HIB_TPIO_PUEN3` reader - TMPR3 Internal Weak Pull-up Enable"]
pub type HIB_TPIO_PUEN3_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TPIO_PUEN3` writer - TMPR3 Internal Weak Pull-up Enable"]
pub type HIB_TPIO_PUEN3_W<'a> = crate::BitWriter<'a, u32, TPIO_SPEC, bool, 26>;
#[doc = "Field `HIB_TPIO_GFLTR3` reader - TMPR3 Glitch Filtering"]
pub type HIB_TPIO_GFLTR3_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TPIO_GFLTR3` writer - TMPR3 Glitch Filtering"]
pub type HIB_TPIO_GFLTR3_W<'a> = crate::BitWriter<'a, u32, TPIO_SPEC, bool, 27>;
impl R {
    #[doc = "Bit 0 - TMPR0 Enable"]
    #[inline(always)]
    pub fn hib_tpio_en0(&self) -> HIB_TPIO_EN0_R {
        HIB_TPIO_EN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TMPR0 Trigger Level"]
    #[inline(always)]
    pub fn hib_tpio_lev0(&self) -> HIB_TPIO_LEV0_R {
        HIB_TPIO_LEV0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TMPR0 Internal Weak Pull-up Enable"]
    #[inline(always)]
    pub fn hib_tpio_puen0(&self) -> HIB_TPIO_PUEN0_R {
        HIB_TPIO_PUEN0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TMPR0 Glitch Filtering"]
    #[inline(always)]
    pub fn hib_tpio_gfltr0(&self) -> HIB_TPIO_GFLTR0_R {
        HIB_TPIO_GFLTR0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - TMPR1Enable"]
    #[inline(always)]
    pub fn hib_tpio_en1(&self) -> HIB_TPIO_EN1_R {
        HIB_TPIO_EN1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TMPR1 Trigger Level"]
    #[inline(always)]
    pub fn hib_tpio_lev1(&self) -> HIB_TPIO_LEV1_R {
        HIB_TPIO_LEV1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TMPR1 Internal Weak Pull-up Enable"]
    #[inline(always)]
    pub fn hib_tpio_puen1(&self) -> HIB_TPIO_PUEN1_R {
        HIB_TPIO_PUEN1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TMPR1 Glitch Filtering"]
    #[inline(always)]
    pub fn hib_tpio_gfltr1(&self) -> HIB_TPIO_GFLTR1_R {
        HIB_TPIO_GFLTR1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - TMPR2 Enable"]
    #[inline(always)]
    pub fn hib_tpio_en2(&self) -> HIB_TPIO_EN2_R {
        HIB_TPIO_EN2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TMPR2 Trigger Level"]
    #[inline(always)]
    pub fn hib_tpio_lev2(&self) -> HIB_TPIO_LEV2_R {
        HIB_TPIO_LEV2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TMPR2 Internal Weak Pull-up Enable"]
    #[inline(always)]
    pub fn hib_tpio_puen2(&self) -> HIB_TPIO_PUEN2_R {
        HIB_TPIO_PUEN2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TMPR2 Glitch Filtering"]
    #[inline(always)]
    pub fn hib_tpio_gfltr2(&self) -> HIB_TPIO_GFLTR2_R {
        HIB_TPIO_GFLTR2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - TMPR3 Enable"]
    #[inline(always)]
    pub fn hib_tpio_en3(&self) -> HIB_TPIO_EN3_R {
        HIB_TPIO_EN3_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TMPR3 Trigger Level"]
    #[inline(always)]
    pub fn hib_tpio_lev3(&self) -> HIB_TPIO_LEV3_R {
        HIB_TPIO_LEV3_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TMPR3 Internal Weak Pull-up Enable"]
    #[inline(always)]
    pub fn hib_tpio_puen3(&self) -> HIB_TPIO_PUEN3_R {
        HIB_TPIO_PUEN3_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TMPR3 Glitch Filtering"]
    #[inline(always)]
    pub fn hib_tpio_gfltr3(&self) -> HIB_TPIO_GFLTR3_R {
        HIB_TPIO_GFLTR3_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TMPR0 Enable"]
    #[inline(always)]
    pub fn hib_tpio_en0(&mut self) -> HIB_TPIO_EN0_W {
        HIB_TPIO_EN0_W::new(self)
    }
    #[doc = "Bit 1 - TMPR0 Trigger Level"]
    #[inline(always)]
    pub fn hib_tpio_lev0(&mut self) -> HIB_TPIO_LEV0_W {
        HIB_TPIO_LEV0_W::new(self)
    }
    #[doc = "Bit 2 - TMPR0 Internal Weak Pull-up Enable"]
    #[inline(always)]
    pub fn hib_tpio_puen0(&mut self) -> HIB_TPIO_PUEN0_W {
        HIB_TPIO_PUEN0_W::new(self)
    }
    #[doc = "Bit 3 - TMPR0 Glitch Filtering"]
    #[inline(always)]
    pub fn hib_tpio_gfltr0(&mut self) -> HIB_TPIO_GFLTR0_W {
        HIB_TPIO_GFLTR0_W::new(self)
    }
    #[doc = "Bit 8 - TMPR1Enable"]
    #[inline(always)]
    pub fn hib_tpio_en1(&mut self) -> HIB_TPIO_EN1_W {
        HIB_TPIO_EN1_W::new(self)
    }
    #[doc = "Bit 9 - TMPR1 Trigger Level"]
    #[inline(always)]
    pub fn hib_tpio_lev1(&mut self) -> HIB_TPIO_LEV1_W {
        HIB_TPIO_LEV1_W::new(self)
    }
    #[doc = "Bit 10 - TMPR1 Internal Weak Pull-up Enable"]
    #[inline(always)]
    pub fn hib_tpio_puen1(&mut self) -> HIB_TPIO_PUEN1_W {
        HIB_TPIO_PUEN1_W::new(self)
    }
    #[doc = "Bit 11 - TMPR1 Glitch Filtering"]
    #[inline(always)]
    pub fn hib_tpio_gfltr1(&mut self) -> HIB_TPIO_GFLTR1_W {
        HIB_TPIO_GFLTR1_W::new(self)
    }
    #[doc = "Bit 16 - TMPR2 Enable"]
    #[inline(always)]
    pub fn hib_tpio_en2(&mut self) -> HIB_TPIO_EN2_W {
        HIB_TPIO_EN2_W::new(self)
    }
    #[doc = "Bit 17 - TMPR2 Trigger Level"]
    #[inline(always)]
    pub fn hib_tpio_lev2(&mut self) -> HIB_TPIO_LEV2_W {
        HIB_TPIO_LEV2_W::new(self)
    }
    #[doc = "Bit 18 - TMPR2 Internal Weak Pull-up Enable"]
    #[inline(always)]
    pub fn hib_tpio_puen2(&mut self) -> HIB_TPIO_PUEN2_W {
        HIB_TPIO_PUEN2_W::new(self)
    }
    #[doc = "Bit 19 - TMPR2 Glitch Filtering"]
    #[inline(always)]
    pub fn hib_tpio_gfltr2(&mut self) -> HIB_TPIO_GFLTR2_W {
        HIB_TPIO_GFLTR2_W::new(self)
    }
    #[doc = "Bit 24 - TMPR3 Enable"]
    #[inline(always)]
    pub fn hib_tpio_en3(&mut self) -> HIB_TPIO_EN3_W {
        HIB_TPIO_EN3_W::new(self)
    }
    #[doc = "Bit 25 - TMPR3 Trigger Level"]
    #[inline(always)]
    pub fn hib_tpio_lev3(&mut self) -> HIB_TPIO_LEV3_W {
        HIB_TPIO_LEV3_W::new(self)
    }
    #[doc = "Bit 26 - TMPR3 Internal Weak Pull-up Enable"]
    #[inline(always)]
    pub fn hib_tpio_puen3(&mut self) -> HIB_TPIO_PUEN3_W {
        HIB_TPIO_PUEN3_W::new(self)
    }
    #[doc = "Bit 27 - TMPR3 Glitch Filtering"]
    #[inline(always)]
    pub fn hib_tpio_gfltr3(&mut self) -> HIB_TPIO_GFLTR3_W {
        HIB_TPIO_GFLTR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HIB Tamper I/O Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpio](index.html) module"]
pub struct TPIO_SPEC;
impl crate::RegisterSpec for TPIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpio::R](R) reader structure"]
impl crate::Readable for TPIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpio::W](W) writer structure"]
impl crate::Writable for TPIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPIO to value 0"]
impl crate::Resettable for TPIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
