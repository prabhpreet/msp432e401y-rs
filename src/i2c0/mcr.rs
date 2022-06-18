#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_MCR_LPBK` reader - I2C Loopback"]
pub type I2C_MCR_LPBK_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MCR_LPBK` writer - I2C Loopback"]
pub type I2C_MCR_LPBK_W<'a> = crate::BitWriter<'a, u32, MCR_SPEC, bool, 0>;
#[doc = "Field `I2C_MCR_MFE` reader - I2C Master Function Enable"]
pub type I2C_MCR_MFE_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MCR_MFE` writer - I2C Master Function Enable"]
pub type I2C_MCR_MFE_W<'a> = crate::BitWriter<'a, u32, MCR_SPEC, bool, 4>;
#[doc = "Field `I2C_MCR_SFE` reader - I2C Slave Function Enable"]
pub type I2C_MCR_SFE_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MCR_SFE` writer - I2C Slave Function Enable"]
pub type I2C_MCR_SFE_W<'a> = crate::BitWriter<'a, u32, MCR_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - I2C Loopback"]
    #[inline(always)]
    pub fn i2c_mcr_lpbk(&self) -> I2C_MCR_LPBK_R {
        I2C_MCR_LPBK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - I2C Master Function Enable"]
    #[inline(always)]
    pub fn i2c_mcr_mfe(&self) -> I2C_MCR_MFE_R {
        I2C_MCR_MFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Slave Function Enable"]
    #[inline(always)]
    pub fn i2c_mcr_sfe(&self) -> I2C_MCR_SFE_R {
        I2C_MCR_SFE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Loopback"]
    #[inline(always)]
    pub fn i2c_mcr_lpbk(&mut self) -> I2C_MCR_LPBK_W {
        I2C_MCR_LPBK_W::new(self)
    }
    #[doc = "Bit 4 - I2C Master Function Enable"]
    #[inline(always)]
    pub fn i2c_mcr_mfe(&mut self) -> I2C_MCR_MFE_W {
        I2C_MCR_MFE_W::new(self)
    }
    #[doc = "Bit 5 - I2C Slave Function Enable"]
    #[inline(always)]
    pub fn i2c_mcr_sfe(&mut self) -> I2C_MCR_SFE_W {
        I2C_MCR_SFE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Master Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
