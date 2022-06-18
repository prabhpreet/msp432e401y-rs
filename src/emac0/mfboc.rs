#[doc = "Register `MFBOC` reader"]
pub struct R(crate::R<MFBOC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MFBOC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MFBOC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MFBOC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MFBOC` writer"]
pub struct W(crate::W<MFBOC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MFBOC_SPEC>;
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
impl From<crate::W<MFBOC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MFBOC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_MFBOC_MISFRMCNT` reader - Missed Frame Counter"]
pub type EMAC_MFBOC_MISFRMCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EMAC_MFBOC_MISFRMCNT` writer - Missed Frame Counter"]
pub type EMAC_MFBOC_MISFRMCNT_W<'a> = crate::FieldWriter<'a, u32, MFBOC_SPEC, u16, u16, 16, 0>;
#[doc = "Field `EMAC_MFBOC_MISCNTOVF` reader - Overflow bit for Missed Frame Counter"]
pub type EMAC_MFBOC_MISCNTOVF_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_MFBOC_MISCNTOVF` writer - Overflow bit for Missed Frame Counter"]
pub type EMAC_MFBOC_MISCNTOVF_W<'a> = crate::BitWriter<'a, u32, MFBOC_SPEC, bool, 16>;
#[doc = "Field `EMAC_MFBOC_OVFFRMCNT` reader - Overflow Frame Counter"]
pub type EMAC_MFBOC_OVFFRMCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EMAC_MFBOC_OVFFRMCNT` writer - Overflow Frame Counter"]
pub type EMAC_MFBOC_OVFFRMCNT_W<'a> = crate::FieldWriter<'a, u32, MFBOC_SPEC, u16, u16, 11, 17>;
#[doc = "Field `EMAC_MFBOC_OVFCNTOVF` reader - Overflow Bit for FIFO Overflow Counter"]
pub type EMAC_MFBOC_OVFCNTOVF_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_MFBOC_OVFCNTOVF` writer - Overflow Bit for FIFO Overflow Counter"]
pub type EMAC_MFBOC_OVFCNTOVF_W<'a> = crate::BitWriter<'a, u32, MFBOC_SPEC, bool, 28>;
impl R {
    #[doc = "Bits 0:15 - Missed Frame Counter"]
    #[inline(always)]
    pub fn emac_mfboc_misfrmcnt(&self) -> EMAC_MFBOC_MISFRMCNT_R {
        EMAC_MFBOC_MISFRMCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Overflow bit for Missed Frame Counter"]
    #[inline(always)]
    pub fn emac_mfboc_miscntovf(&self) -> EMAC_MFBOC_MISCNTOVF_R {
        EMAC_MFBOC_MISCNTOVF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:27 - Overflow Frame Counter"]
    #[inline(always)]
    pub fn emac_mfboc_ovffrmcnt(&self) -> EMAC_MFBOC_OVFFRMCNT_R {
        EMAC_MFBOC_OVFFRMCNT_R::new(((self.bits >> 17) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - Overflow Bit for FIFO Overflow Counter"]
    #[inline(always)]
    pub fn emac_mfboc_ovfcntovf(&self) -> EMAC_MFBOC_OVFCNTOVF_R {
        EMAC_MFBOC_OVFCNTOVF_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Missed Frame Counter"]
    #[inline(always)]
    pub fn emac_mfboc_misfrmcnt(&mut self) -> EMAC_MFBOC_MISFRMCNT_W {
        EMAC_MFBOC_MISFRMCNT_W::new(self)
    }
    #[doc = "Bit 16 - Overflow bit for Missed Frame Counter"]
    #[inline(always)]
    pub fn emac_mfboc_miscntovf(&mut self) -> EMAC_MFBOC_MISCNTOVF_W {
        EMAC_MFBOC_MISCNTOVF_W::new(self)
    }
    #[doc = "Bits 17:27 - Overflow Frame Counter"]
    #[inline(always)]
    pub fn emac_mfboc_ovffrmcnt(&mut self) -> EMAC_MFBOC_OVFFRMCNT_W {
        EMAC_MFBOC_OVFFRMCNT_W::new(self)
    }
    #[doc = "Bit 28 - Overflow Bit for FIFO Overflow Counter"]
    #[inline(always)]
    pub fn emac_mfboc_ovfcntovf(&mut self) -> EMAC_MFBOC_OVFCNTOVF_W {
        EMAC_MFBOC_OVFCNTOVF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Missed Frame and Buffer Overflow Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mfboc](index.html) module"]
pub struct MFBOC_SPEC;
impl crate::RegisterSpec for MFBOC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mfboc::R](R) reader structure"]
impl crate::Readable for MFBOC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mfboc::W](W) writer structure"]
impl crate::Writable for MFBOC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MFBOC to value 0"]
impl crate::Resettable for MFBOC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
