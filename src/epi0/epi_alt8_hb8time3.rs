#[doc = "Register `HB8TIME3` reader"]
pub struct R(crate::R<EPI_ALT8_HB8TIME3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPI_ALT8_HB8TIME3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPI_ALT8_HB8TIME3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPI_ALT8_HB8TIME3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HB8TIME3` writer"]
pub struct W(crate::W<EPI_ALT8_HB8TIME3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPI_ALT8_HB8TIME3_SPEC>;
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
impl From<crate::W<EPI_ALT8_HB8TIME3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPI_ALT8_HB8TIME3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPI_HB8TIME3_RDWSM` reader - CS2n Read Wait State Minus One"]
pub type EPI_HB8TIME3_RDWSM_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB8TIME3_RDWSM` writer - CS2n Read Wait State Minus One"]
pub type EPI_HB8TIME3_RDWSM_W<'a> = crate::BitWriter<'a, u32, EPI_ALT8_HB8TIME3_SPEC, bool, 0>;
#[doc = "Field `EPI_HB8TIME3_WRWSM` reader - CS2n Write Wait State Minus One"]
pub type EPI_HB8TIME3_WRWSM_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB8TIME3_WRWSM` writer - CS2n Write Wait State Minus One"]
pub type EPI_HB8TIME3_WRWSM_W<'a> = crate::BitWriter<'a, u32, EPI_ALT8_HB8TIME3_SPEC, bool, 4>;
#[doc = "Field `EPI_HB8TIME3_CAPWIDTH` reader - CS2n Inter-transfer Capture Width"]
pub type EPI_HB8TIME3_CAPWIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPI_HB8TIME3_CAPWIDTH` writer - CS2n Inter-transfer Capture Width"]
pub type EPI_HB8TIME3_CAPWIDTH_W<'a> =
    crate::FieldWriter<'a, u32, EPI_ALT8_HB8TIME3_SPEC, u8, u8, 2, 12>;
#[doc = "Field `EPI_HB8TIME3_IRDYDLY` reader - CS2n Input Ready Delay"]
pub type EPI_HB8TIME3_IRDYDLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPI_HB8TIME3_IRDYDLY` writer - CS2n Input Ready Delay"]
pub type EPI_HB8TIME3_IRDYDLY_W<'a> =
    crate::FieldWriter<'a, u32, EPI_ALT8_HB8TIME3_SPEC, u8, u8, 2, 24>;
impl R {
    #[doc = "Bit 0 - CS2n Read Wait State Minus One"]
    #[inline(always)]
    pub fn epi_hb8time3_rdwsm(&self) -> EPI_HB8TIME3_RDWSM_R {
        EPI_HB8TIME3_RDWSM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CS2n Write Wait State Minus One"]
    #[inline(always)]
    pub fn epi_hb8time3_wrwsm(&self) -> EPI_HB8TIME3_WRWSM_R {
        EPI_HB8TIME3_WRWSM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 12:13 - CS2n Inter-transfer Capture Width"]
    #[inline(always)]
    pub fn epi_hb8time3_capwidth(&self) -> EPI_HB8TIME3_CAPWIDTH_R {
        EPI_HB8TIME3_CAPWIDTH_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 24:25 - CS2n Input Ready Delay"]
    #[inline(always)]
    pub fn epi_hb8time3_irdydly(&self) -> EPI_HB8TIME3_IRDYDLY_R {
        EPI_HB8TIME3_IRDYDLY_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CS2n Read Wait State Minus One"]
    #[inline(always)]
    pub fn epi_hb8time3_rdwsm(&mut self) -> EPI_HB8TIME3_RDWSM_W {
        EPI_HB8TIME3_RDWSM_W::new(self)
    }
    #[doc = "Bit 4 - CS2n Write Wait State Minus One"]
    #[inline(always)]
    pub fn epi_hb8time3_wrwsm(&mut self) -> EPI_HB8TIME3_WRWSM_W {
        EPI_HB8TIME3_WRWSM_W::new(self)
    }
    #[doc = "Bits 12:13 - CS2n Inter-transfer Capture Width"]
    #[inline(always)]
    pub fn epi_hb8time3_capwidth(&mut self) -> EPI_HB8TIME3_CAPWIDTH_W {
        EPI_HB8TIME3_CAPWIDTH_W::new(self)
    }
    #[doc = "Bits 24:25 - CS2n Input Ready Delay"]
    #[inline(always)]
    pub fn epi_hb8time3_irdydly(&mut self) -> EPI_HB8TIME3_IRDYDLY_W {
        EPI_HB8TIME3_IRDYDLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Host-Bus 8 Timing Extension\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epi_alt8_hb8time3](index.html) module"]
pub struct EPI_ALT8_HB8TIME3_SPEC;
impl crate::RegisterSpec for EPI_ALT8_HB8TIME3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epi_alt8_hb8time3::R](R) reader structure"]
impl crate::Readable for EPI_ALT8_HB8TIME3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epi_alt8_hb8time3::W](W) writer structure"]
impl crate::Writable for EPI_ALT8_HB8TIME3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HB8TIME3 to value 0"]
impl crate::Resettable for EPI_ALT8_HB8TIME3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
