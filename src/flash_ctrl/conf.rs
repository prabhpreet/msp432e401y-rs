#[doc = "Register `CONF` reader"]
pub struct R(crate::R<CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF` writer"]
pub struct W(crate::W<CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_SPEC>;
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
impl From<crate::W<CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_CONF_FPFOFF` reader - Force Prefetch Off"]
pub type FLASH_CONF_FPFOFF_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_CONF_FPFOFF` writer - Force Prefetch Off"]
pub type FLASH_CONF_FPFOFF_W<'a> = crate::BitWriter<'a, u32, CONF_SPEC, bool, 16>;
#[doc = "Field `FLASH_CONF_FPFON` reader - Force Prefetch On"]
pub type FLASH_CONF_FPFON_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_CONF_FPFON` writer - Force Prefetch On"]
pub type FLASH_CONF_FPFON_W<'a> = crate::BitWriter<'a, u32, CONF_SPEC, bool, 17>;
#[doc = "Field `FLASH_CONF_CLRTV` reader - Clear Valid Tags"]
pub type FLASH_CONF_CLRTV_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_CONF_CLRTV` writer - Clear Valid Tags"]
pub type FLASH_CONF_CLRTV_W<'a> = crate::BitWriter<'a, u32, CONF_SPEC, bool, 20>;
#[doc = "Field `FLASH_CONF_SPFE` reader - Single Prefetch Mode Enable"]
pub type FLASH_CONF_SPFE_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_CONF_SPFE` writer - Single Prefetch Mode Enable"]
pub type FLASH_CONF_SPFE_W<'a> = crate::BitWriter<'a, u32, CONF_SPEC, bool, 29>;
#[doc = "Field `FLASH_CONF_FMME` reader - Flash Mirror Mode Enable"]
pub type FLASH_CONF_FMME_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_CONF_FMME` writer - Flash Mirror Mode Enable"]
pub type FLASH_CONF_FMME_W<'a> = crate::BitWriter<'a, u32, CONF_SPEC, bool, 30>;
impl R {
    #[doc = "Bit 16 - Force Prefetch Off"]
    #[inline(always)]
    pub fn flash_conf_fpfoff(&self) -> FLASH_CONF_FPFOFF_R {
        FLASH_CONF_FPFOFF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Force Prefetch On"]
    #[inline(always)]
    pub fn flash_conf_fpfon(&self) -> FLASH_CONF_FPFON_R {
        FLASH_CONF_FPFON_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Clear Valid Tags"]
    #[inline(always)]
    pub fn flash_conf_clrtv(&self) -> FLASH_CONF_CLRTV_R {
        FLASH_CONF_CLRTV_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 29 - Single Prefetch Mode Enable"]
    #[inline(always)]
    pub fn flash_conf_spfe(&self) -> FLASH_CONF_SPFE_R {
        FLASH_CONF_SPFE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Flash Mirror Mode Enable"]
    #[inline(always)]
    pub fn flash_conf_fmme(&self) -> FLASH_CONF_FMME_R {
        FLASH_CONF_FMME_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Force Prefetch Off"]
    #[inline(always)]
    pub fn flash_conf_fpfoff(&mut self) -> FLASH_CONF_FPFOFF_W {
        FLASH_CONF_FPFOFF_W::new(self)
    }
    #[doc = "Bit 17 - Force Prefetch On"]
    #[inline(always)]
    pub fn flash_conf_fpfon(&mut self) -> FLASH_CONF_FPFON_W {
        FLASH_CONF_FPFON_W::new(self)
    }
    #[doc = "Bit 20 - Clear Valid Tags"]
    #[inline(always)]
    pub fn flash_conf_clrtv(&mut self) -> FLASH_CONF_CLRTV_W {
        FLASH_CONF_CLRTV_W::new(self)
    }
    #[doc = "Bit 29 - Single Prefetch Mode Enable"]
    #[inline(always)]
    pub fn flash_conf_spfe(&mut self) -> FLASH_CONF_SPFE_W {
        FLASH_CONF_SPFE_W::new(self)
    }
    #[doc = "Bit 30 - Flash Mirror Mode Enable"]
    #[inline(always)]
    pub fn flash_conf_fmme(&mut self) -> FLASH_CONF_FMME_W {
        FLASH_CONF_FMME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf](index.html) module"]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf::R](R) reader structure"]
impl crate::Readable for CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf::W](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF to value 0"]
impl crate::Resettable for CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
