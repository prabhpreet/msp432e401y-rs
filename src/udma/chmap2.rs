#[doc = "Register `CHMAP2` reader"]
pub struct R(crate::R<CHMAP2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHMAP2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHMAP2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHMAP2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHMAP2` writer"]
pub struct W(crate::W<CHMAP2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHMAP2_SPEC>;
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
impl From<crate::W<CHMAP2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHMAP2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UDMA_CHMAP2_CH16SEL` reader - uDMA Channel 16 Source Select"]
pub type UDMA_CHMAP2_CH16SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP2_CH16SEL` writer - uDMA Channel 16 Source Select"]
pub type UDMA_CHMAP2_CH16SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP2_SPEC, u8, u8, 4, 0>;
#[doc = "Field `UDMA_CHMAP2_CH17SEL` reader - uDMA Channel 17 Source Select"]
pub type UDMA_CHMAP2_CH17SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP2_CH17SEL` writer - uDMA Channel 17 Source Select"]
pub type UDMA_CHMAP2_CH17SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP2_SPEC, u8, u8, 4, 4>;
#[doc = "Field `UDMA_CHMAP2_CH18SEL` reader - uDMA Channel 18 Source Select"]
pub type UDMA_CHMAP2_CH18SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP2_CH18SEL` writer - uDMA Channel 18 Source Select"]
pub type UDMA_CHMAP2_CH18SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP2_SPEC, u8, u8, 4, 8>;
#[doc = "Field `UDMA_CHMAP2_CH19SEL` reader - uDMA Channel 19 Source Select"]
pub type UDMA_CHMAP2_CH19SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP2_CH19SEL` writer - uDMA Channel 19 Source Select"]
pub type UDMA_CHMAP2_CH19SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP2_SPEC, u8, u8, 4, 12>;
#[doc = "Field `UDMA_CHMAP2_CH20SEL` reader - uDMA Channel 20 Source Select"]
pub type UDMA_CHMAP2_CH20SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP2_CH20SEL` writer - uDMA Channel 20 Source Select"]
pub type UDMA_CHMAP2_CH20SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP2_SPEC, u8, u8, 4, 16>;
#[doc = "Field `UDMA_CHMAP2_CH21SEL` reader - uDMA Channel 21 Source Select"]
pub type UDMA_CHMAP2_CH21SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP2_CH21SEL` writer - uDMA Channel 21 Source Select"]
pub type UDMA_CHMAP2_CH21SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP2_SPEC, u8, u8, 4, 20>;
#[doc = "Field `UDMA_CHMAP2_CH22SEL` reader - uDMA Channel 22 Source Select"]
pub type UDMA_CHMAP2_CH22SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP2_CH22SEL` writer - uDMA Channel 22 Source Select"]
pub type UDMA_CHMAP2_CH22SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP2_SPEC, u8, u8, 4, 24>;
#[doc = "Field `UDMA_CHMAP2_CH23SEL` reader - uDMA Channel 23 Source Select"]
pub type UDMA_CHMAP2_CH23SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_CHMAP2_CH23SEL` writer - uDMA Channel 23 Source Select"]
pub type UDMA_CHMAP2_CH23SEL_W<'a> = crate::FieldWriter<'a, u32, CHMAP2_SPEC, u8, u8, 4, 28>;
impl R {
    #[doc = "Bits 0:3 - uDMA Channel 16 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch16sel(&self) -> UDMA_CHMAP2_CH16SEL_R {
        UDMA_CHMAP2_CH16SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - uDMA Channel 17 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch17sel(&self) -> UDMA_CHMAP2_CH17SEL_R {
        UDMA_CHMAP2_CH17SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - uDMA Channel 18 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch18sel(&self) -> UDMA_CHMAP2_CH18SEL_R {
        UDMA_CHMAP2_CH18SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - uDMA Channel 19 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch19sel(&self) -> UDMA_CHMAP2_CH19SEL_R {
        UDMA_CHMAP2_CH19SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - uDMA Channel 20 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch20sel(&self) -> UDMA_CHMAP2_CH20SEL_R {
        UDMA_CHMAP2_CH20SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - uDMA Channel 21 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch21sel(&self) -> UDMA_CHMAP2_CH21SEL_R {
        UDMA_CHMAP2_CH21SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - uDMA Channel 22 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch22sel(&self) -> UDMA_CHMAP2_CH22SEL_R {
        UDMA_CHMAP2_CH22SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - uDMA Channel 23 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch23sel(&self) -> UDMA_CHMAP2_CH23SEL_R {
        UDMA_CHMAP2_CH23SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - uDMA Channel 16 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch16sel(&mut self) -> UDMA_CHMAP2_CH16SEL_W {
        UDMA_CHMAP2_CH16SEL_W::new(self)
    }
    #[doc = "Bits 4:7 - uDMA Channel 17 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch17sel(&mut self) -> UDMA_CHMAP2_CH17SEL_W {
        UDMA_CHMAP2_CH17SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - uDMA Channel 18 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch18sel(&mut self) -> UDMA_CHMAP2_CH18SEL_W {
        UDMA_CHMAP2_CH18SEL_W::new(self)
    }
    #[doc = "Bits 12:15 - uDMA Channel 19 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch19sel(&mut self) -> UDMA_CHMAP2_CH19SEL_W {
        UDMA_CHMAP2_CH19SEL_W::new(self)
    }
    #[doc = "Bits 16:19 - uDMA Channel 20 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch20sel(&mut self) -> UDMA_CHMAP2_CH20SEL_W {
        UDMA_CHMAP2_CH20SEL_W::new(self)
    }
    #[doc = "Bits 20:23 - uDMA Channel 21 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch21sel(&mut self) -> UDMA_CHMAP2_CH21SEL_W {
        UDMA_CHMAP2_CH21SEL_W::new(self)
    }
    #[doc = "Bits 24:27 - uDMA Channel 22 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch22sel(&mut self) -> UDMA_CHMAP2_CH22SEL_W {
        UDMA_CHMAP2_CH22SEL_W::new(self)
    }
    #[doc = "Bits 28:31 - uDMA Channel 23 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch23sel(&mut self) -> UDMA_CHMAP2_CH23SEL_W {
        UDMA_CHMAP2_CH23SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel Map Select 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chmap2](index.html) module"]
pub struct CHMAP2_SPEC;
impl crate::RegisterSpec for CHMAP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chmap2::R](R) reader structure"]
impl crate::Readable for CHMAP2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chmap2::W](W) writer structure"]
impl crate::Writable for CHMAP2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHMAP2 to value 0"]
impl crate::Resettable for CHMAP2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
