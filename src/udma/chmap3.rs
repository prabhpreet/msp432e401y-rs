#[doc = "Register `CHMAP3` reader"]
pub struct R(crate::R<CHMAP3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHMAP3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHMAP3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHMAP3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHMAP3` writer"]
pub struct W(crate::W<CHMAP3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHMAP3_SPEC>;
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
impl From<crate::W<CHMAP3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHMAP3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UDMA_CHMAP3_CH24SEL` reader - uDMA Channel 24 Source Select"]
pub type UDMA_CHMAP3_CH24SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP3_CH24SEL` writer - uDMA Channel 24 Source Select"]
pub type UDMA_CHMAP3_CH24SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP3_SPEC, u8, u8, 4, 0>;
#[doc = "Field `UDMA_CHMAP3_CH25SEL` reader - uDMA Channel 25 Source Select"]
pub type UDMA_CHMAP3_CH25SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP3_CH25SEL` writer - uDMA Channel 25 Source Select"]
pub type UDMA_CHMAP3_CH25SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP3_SPEC, u8, u8, 4, 4>;
#[doc = "Field `UDMA_CHMAP3_CH26SEL` reader - uDMA Channel 26 Source Select"]
pub type UDMA_CHMAP3_CH26SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP3_CH26SEL` writer - uDMA Channel 26 Source Select"]
pub type UDMA_CHMAP3_CH26SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP3_SPEC, u8, u8, 4, 8>;
#[doc = "Field `UDMA_CHMAP3_CH27SEL` reader - uDMA Channel 27 Source Select"]
pub type UDMA_CHMAP3_CH27SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP3_CH27SEL` writer - uDMA Channel 27 Source Select"]
pub type UDMA_CHMAP3_CH27SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP3_SPEC, u8, u8, 4, 12>;
#[doc = "Field `UDMA_CHMAP3_CH28SEL` reader - uDMA Channel 28 Source Select"]
pub type UDMA_CHMAP3_CH28SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP3_CH28SEL` writer - uDMA Channel 28 Source Select"]
pub type UDMA_CHMAP3_CH28SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP3_SPEC, u8, u8, 4, 16>;
#[doc = "Field `UDMA_CHMAP3_CH29SEL` reader - uDMA Channel 29 Source Select"]
pub type UDMA_CHMAP3_CH29SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP3_CH29SEL` writer - uDMA Channel 29 Source Select"]
pub type UDMA_CHMAP3_CH29SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP3_SPEC, u8, u8, 4, 20>;
#[doc = "Field `UDMA_CHMAP3_CH30SEL` reader - uDMA Channel 30 Source Select"]
pub type UDMA_CHMAP3_CH30SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP3_CH30SEL` writer - uDMA Channel 30 Source Select"]
pub type UDMA_CHMAP3_CH30SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP3_SPEC, u8, u8, 4, 24>;
#[doc = "Field `UDMA_CHMAP3_CH31SEL` reader - uDMA Channel 31 Source Select"]
pub type UDMA_CHMAP3_CH31SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP3_CH31SEL` writer - uDMA Channel 31 Source Select"]
pub type UDMA_CHMAP3_CH31SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP3_SPEC, u8, u8, 4, 28>;
impl R {
    #[doc = "Bits 0:3 - uDMA Channel 24 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch24sel(&self) -> UDMA_CHMAP3_CH24SEL_R {
        UDMA_CHMAP3_CH24SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - uDMA Channel 25 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch25sel(&self) -> UDMA_CHMAP3_CH25SEL_R {
        UDMA_CHMAP3_CH25SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - uDMA Channel 26 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch26sel(&self) -> UDMA_CHMAP3_CH26SEL_R {
        UDMA_CHMAP3_CH26SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - uDMA Channel 27 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch27sel(&self) -> UDMA_CHMAP3_CH27SEL_R {
        UDMA_CHMAP3_CH27SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - uDMA Channel 28 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch28sel(&self) -> UDMA_CHMAP3_CH28SEL_R {
        UDMA_CHMAP3_CH28SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - uDMA Channel 29 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch29sel(&self) -> UDMA_CHMAP3_CH29SEL_R {
        UDMA_CHMAP3_CH29SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - uDMA Channel 30 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch30sel(&self) -> UDMA_CHMAP3_CH30SEL_R {
        UDMA_CHMAP3_CH30SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - uDMA Channel 31 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch31sel(&self) -> UDMA_CHMAP3_CH31SEL_R {
        UDMA_CHMAP3_CH31SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - uDMA Channel 24 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch24sel(&mut self) -> UDMA_CHMAP3_CH24SEL_W {
        UDMA_CHMAP3_CH24SEL_W::new(self)
    }
    #[doc = "Bits 4:7 - uDMA Channel 25 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch25sel(&mut self) -> UDMA_CHMAP3_CH25SEL_W {
        UDMA_CHMAP3_CH25SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - uDMA Channel 26 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch26sel(&mut self) -> UDMA_CHMAP3_CH26SEL_W {
        UDMA_CHMAP3_CH26SEL_W::new(self)
    }
    #[doc = "Bits 12:15 - uDMA Channel 27 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch27sel(&mut self) -> UDMA_CHMAP3_CH27SEL_W {
        UDMA_CHMAP3_CH27SEL_W::new(self)
    }
    #[doc = "Bits 16:19 - uDMA Channel 28 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch28sel(&mut self) -> UDMA_CHMAP3_CH28SEL_W {
        UDMA_CHMAP3_CH28SEL_W::new(self)
    }
    #[doc = "Bits 20:23 - uDMA Channel 29 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch29sel(&mut self) -> UDMA_CHMAP3_CH29SEL_W {
        UDMA_CHMAP3_CH29SEL_W::new(self)
    }
    #[doc = "Bits 24:27 - uDMA Channel 30 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch30sel(&mut self) -> UDMA_CHMAP3_CH30SEL_W {
        UDMA_CHMAP3_CH30SEL_W::new(self)
    }
    #[doc = "Bits 28:31 - uDMA Channel 31 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch31sel(&mut self) -> UDMA_CHMAP3_CH31SEL_W {
        UDMA_CHMAP3_CH31SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel Map Select 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chmap3](index.html) module"]
pub struct CHMAP3_SPEC;
impl crate::RegisterSpec for CHMAP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chmap3::R](R) reader structure"]
impl crate::Readable for CHMAP3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chmap3::W](W) writer structure"]
impl crate::Writable for CHMAP3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHMAP3 to value 0"]
impl crate::Resettable for CHMAP3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
