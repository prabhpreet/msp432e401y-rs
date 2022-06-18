#[doc = "Register `CHMAP1` reader"]
pub struct R(crate::R<CHMAP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHMAP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHMAP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHMAP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHMAP1` writer"]
pub struct W(crate::W<CHMAP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHMAP1_SPEC>;
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
impl From<crate::W<CHMAP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHMAP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UDMA_CHMAP1_CH8SEL` reader - uDMA Channel 8 Source Select"]
pub type UDMA_CHMAP1_CH8SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP1_CH8SEL` writer - uDMA Channel 8 Source Select"]
pub type UDMA_CHMAP1_CH8SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP1_SPEC, u8, u8, 4, 0>;
#[doc = "Field `UDMA_CHMAP1_CH9SEL` reader - uDMA Channel 9 Source Select"]
pub type UDMA_CHMAP1_CH9SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP1_CH9SEL` writer - uDMA Channel 9 Source Select"]
pub type UDMA_CHMAP1_CH9SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP1_SPEC, u8, u8, 4, 4>;
#[doc = "Field `UDMA_CHMAP1_CH10SEL` reader - uDMA Channel 10 Source Select"]
pub type UDMA_CHMAP1_CH10SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP1_CH10SEL` writer - uDMA Channel 10 Source Select"]
pub type UDMA_CHMAP1_CH10SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP1_SPEC, u8, u8, 4, 8>;
#[doc = "Field `UDMA_CHMAP1_CH11SEL` reader - uDMA Channel 11 Source Select"]
pub type UDMA_CHMAP1_CH11SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP1_CH11SEL` writer - uDMA Channel 11 Source Select"]
pub type UDMA_CHMAP1_CH11SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP1_SPEC, u8, u8, 4, 12>;
#[doc = "Field `UDMA_CHMAP1_CH12SEL` reader - uDMA Channel 12 Source Select"]
pub type UDMA_CHMAP1_CH12SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP1_CH12SEL` writer - uDMA Channel 12 Source Select"]
pub type UDMA_CHMAP1_CH12SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP1_SPEC, u8, u8, 4, 16>;
#[doc = "Field `UDMA_CHMAP1_CH13SEL` reader - uDMA Channel 13 Source Select"]
pub type UDMA_CHMAP1_CH13SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP1_CH13SEL` writer - uDMA Channel 13 Source Select"]
pub type UDMA_CHMAP1_CH13SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP1_SPEC, u8, u8, 4, 20>;
#[doc = "Field `UDMA_CHMAP1_CH14SEL` reader - uDMA Channel 14 Source Select"]
pub type UDMA_CHMAP1_CH14SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP1_CH14SEL` writer - uDMA Channel 14 Source Select"]
pub type UDMA_CHMAP1_CH14SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP1_SPEC, u8, u8, 4, 24>;
#[doc = "Field `UDMA_CHMAP1_CH15SEL` reader - uDMA Channel 15 Source Select"]
pub type UDMA_CHMAP1_CH15SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP1_CH15SEL` writer - uDMA Channel 15 Source Select"]
pub type UDMA_CHMAP1_CH15SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP1_SPEC, u8, u8, 4, 28>;
impl R {
    #[doc = "Bits 0:3 - uDMA Channel 8 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch8sel(&self) -> UDMA_CHMAP1_CH8SEL_R {
        UDMA_CHMAP1_CH8SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - uDMA Channel 9 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch9sel(&self) -> UDMA_CHMAP1_CH9SEL_R {
        UDMA_CHMAP1_CH9SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - uDMA Channel 10 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch10sel(&self) -> UDMA_CHMAP1_CH10SEL_R {
        UDMA_CHMAP1_CH10SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - uDMA Channel 11 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch11sel(&self) -> UDMA_CHMAP1_CH11SEL_R {
        UDMA_CHMAP1_CH11SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - uDMA Channel 12 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch12sel(&self) -> UDMA_CHMAP1_CH12SEL_R {
        UDMA_CHMAP1_CH12SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - uDMA Channel 13 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch13sel(&self) -> UDMA_CHMAP1_CH13SEL_R {
        UDMA_CHMAP1_CH13SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - uDMA Channel 14 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch14sel(&self) -> UDMA_CHMAP1_CH14SEL_R {
        UDMA_CHMAP1_CH14SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - uDMA Channel 15 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch15sel(&self) -> UDMA_CHMAP1_CH15SEL_R {
        UDMA_CHMAP1_CH15SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - uDMA Channel 8 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch8sel(&mut self) -> UDMA_CHMAP1_CH8SEL_W {
        UDMA_CHMAP1_CH8SEL_W::new(self)
    }
    #[doc = "Bits 4:7 - uDMA Channel 9 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch9sel(&mut self) -> UDMA_CHMAP1_CH9SEL_W {
        UDMA_CHMAP1_CH9SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - uDMA Channel 10 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch10sel(&mut self) -> UDMA_CHMAP1_CH10SEL_W {
        UDMA_CHMAP1_CH10SEL_W::new(self)
    }
    #[doc = "Bits 12:15 - uDMA Channel 11 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch11sel(&mut self) -> UDMA_CHMAP1_CH11SEL_W {
        UDMA_CHMAP1_CH11SEL_W::new(self)
    }
    #[doc = "Bits 16:19 - uDMA Channel 12 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch12sel(&mut self) -> UDMA_CHMAP1_CH12SEL_W {
        UDMA_CHMAP1_CH12SEL_W::new(self)
    }
    #[doc = "Bits 20:23 - uDMA Channel 13 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch13sel(&mut self) -> UDMA_CHMAP1_CH13SEL_W {
        UDMA_CHMAP1_CH13SEL_W::new(self)
    }
    #[doc = "Bits 24:27 - uDMA Channel 14 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch14sel(&mut self) -> UDMA_CHMAP1_CH14SEL_W {
        UDMA_CHMAP1_CH14SEL_W::new(self)
    }
    #[doc = "Bits 28:31 - uDMA Channel 15 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch15sel(&mut self) -> UDMA_CHMAP1_CH15SEL_W {
        UDMA_CHMAP1_CH15SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel Map Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chmap1](index.html) module"]
pub struct CHMAP1_SPEC;
impl crate::RegisterSpec for CHMAP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chmap1::R](R) reader structure"]
impl crate::Readable for CHMAP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chmap1::W](W) writer structure"]
impl crate::Writable for CHMAP1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHMAP1 to value 0"]
impl crate::Resettable for CHMAP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
