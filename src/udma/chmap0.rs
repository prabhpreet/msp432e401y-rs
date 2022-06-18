#[doc = "Register `CHMAP0` reader"]
pub struct R(crate::R<CHMAP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHMAP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHMAP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHMAP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHMAP0` writer"]
pub struct W(crate::W<CHMAP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHMAP0_SPEC>;
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
impl From<crate::W<CHMAP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHMAP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UDMA_CHMAP0_CH0SEL` reader - uDMA Channel 0 Source Select"]
pub type UDMA_CHMAP0_CH0SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP0_CH0SEL` writer - uDMA Channel 0 Source Select"]
pub type UDMA_CHMAP0_CH0SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP0_SPEC, u8, u8, 4, 0>;
#[doc = "Field `UDMA_CHMAP0_CH1SEL` reader - uDMA Channel 1 Source Select"]
pub type UDMA_CHMAP0_CH1SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP0_CH1SEL` writer - uDMA Channel 1 Source Select"]
pub type UDMA_CHMAP0_CH1SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP0_SPEC, u8, u8, 4, 4>;
#[doc = "Field `UDMA_CHMAP0_CH2SEL` reader - uDMA Channel 2 Source Select"]
pub type UDMA_CHMAP0_CH2SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP0_CH2SEL` writer - uDMA Channel 2 Source Select"]
pub type UDMA_CHMAP0_CH2SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP0_SPEC, u8, u8, 4, 8>;
#[doc = "Field `UDMA_CHMAP0_CH3SEL` reader - uDMA Channel 3 Source Select"]
pub type UDMA_CHMAP0_CH3SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP0_CH3SEL` writer - uDMA Channel 3 Source Select"]
pub type UDMA_CHMAP0_CH3SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP0_SPEC, u8, u8, 4, 12>;
#[doc = "Field `UDMA_CHMAP0_CH4SEL` reader - uDMA Channel 4 Source Select"]
pub type UDMA_CHMAP0_CH4SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP0_CH4SEL` writer - uDMA Channel 4 Source Select"]
pub type UDMA_CHMAP0_CH4SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP0_SPEC, u8, u8, 4, 16>;
#[doc = "Field `UDMA_CHMAP0_CH5SEL` reader - uDMA Channel 5 Source Select"]
pub type UDMA_CHMAP0_CH5SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP0_CH5SEL` writer - uDMA Channel 5 Source Select"]
pub type UDMA_CHMAP0_CH5SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP0_SPEC, u8, u8, 4, 20>;
#[doc = "Field `UDMA_CHMAP0_CH6SEL` reader - uDMA Channel 6 Source Select"]
pub type UDMA_CHMAP0_CH6SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP0_CH6SEL` writer - uDMA Channel 6 Source Select"]
pub type UDMA_CHMAP0_CH6SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP0_SPEC, u8, u8, 4, 24>;
#[doc = "Field `UDMA_CHMAP0_CH7SEL` reader - uDMA Channel 7 Source Select"]
pub type UDMA_CHMAP0_CH7SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP0_CH7SEL` writer - uDMA Channel 7 Source Select"]
pub type UDMA_CHMAP0_CH7SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP0_SPEC, u8, u8, 4, 28>;
impl R {
    #[doc = "Bits 0:3 - uDMA Channel 0 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch0sel(&self) -> UDMA_CHMAP0_CH0SEL_R {
        UDMA_CHMAP0_CH0SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - uDMA Channel 1 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch1sel(&self) -> UDMA_CHMAP0_CH1SEL_R {
        UDMA_CHMAP0_CH1SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - uDMA Channel 2 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch2sel(&self) -> UDMA_CHMAP0_CH2SEL_R {
        UDMA_CHMAP0_CH2SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - uDMA Channel 3 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch3sel(&self) -> UDMA_CHMAP0_CH3SEL_R {
        UDMA_CHMAP0_CH3SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - uDMA Channel 4 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch4sel(&self) -> UDMA_CHMAP0_CH4SEL_R {
        UDMA_CHMAP0_CH4SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - uDMA Channel 5 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch5sel(&self) -> UDMA_CHMAP0_CH5SEL_R {
        UDMA_CHMAP0_CH5SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - uDMA Channel 6 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch6sel(&self) -> UDMA_CHMAP0_CH6SEL_R {
        UDMA_CHMAP0_CH6SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - uDMA Channel 7 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch7sel(&self) -> UDMA_CHMAP0_CH7SEL_R {
        UDMA_CHMAP0_CH7SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - uDMA Channel 0 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch0sel(&mut self) -> UDMA_CHMAP0_CH0SEL_W {
        UDMA_CHMAP0_CH0SEL_W::new(self)
    }
    #[doc = "Bits 4:7 - uDMA Channel 1 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch1sel(&mut self) -> UDMA_CHMAP0_CH1SEL_W {
        UDMA_CHMAP0_CH1SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - uDMA Channel 2 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch2sel(&mut self) -> UDMA_CHMAP0_CH2SEL_W {
        UDMA_CHMAP0_CH2SEL_W::new(self)
    }
    #[doc = "Bits 12:15 - uDMA Channel 3 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch3sel(&mut self) -> UDMA_CHMAP0_CH3SEL_W {
        UDMA_CHMAP0_CH3SEL_W::new(self)
    }
    #[doc = "Bits 16:19 - uDMA Channel 4 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch4sel(&mut self) -> UDMA_CHMAP0_CH4SEL_W {
        UDMA_CHMAP0_CH4SEL_W::new(self)
    }
    #[doc = "Bits 20:23 - uDMA Channel 5 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch5sel(&mut self) -> UDMA_CHMAP0_CH5SEL_W {
        UDMA_CHMAP0_CH5SEL_W::new(self)
    }
    #[doc = "Bits 24:27 - uDMA Channel 6 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch6sel(&mut self) -> UDMA_CHMAP0_CH6SEL_W {
        UDMA_CHMAP0_CH6SEL_W::new(self)
    }
    #[doc = "Bits 28:31 - uDMA Channel 7 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch7sel(&mut self) -> UDMA_CHMAP0_CH7SEL_W {
        UDMA_CHMAP0_CH7SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel Map Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chmap0](index.html) module"]
pub struct CHMAP0_SPEC;
impl crate::RegisterSpec for CHMAP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chmap0::R](R) reader structure"]
impl crate::Readable for CHMAP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chmap0::W](W) writer structure"]
impl crate::Writable for CHMAP0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHMAP0 to value 0"]
impl crate::Resettable for CHMAP0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
