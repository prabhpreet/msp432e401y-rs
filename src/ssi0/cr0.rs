#[doc = "Register `CR0` reader"]
pub struct R(crate::R<CR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR0` writer"]
pub struct W(crate::W<CR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR0_SPEC>;
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
impl From<crate::W<CR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SSI Data Size Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSI_CR0_DSS_A {
    #[doc = "3: 4-bit data"]
    SSI_CR0_DSS_4 = 3,
    #[doc = "4: 5-bit data"]
    SSI_CR0_DSS_5 = 4,
    #[doc = "5: 6-bit data"]
    SSI_CR0_DSS_6 = 5,
    #[doc = "6: 7-bit data"]
    SSI_CR0_DSS_7 = 6,
    #[doc = "7: 8-bit data"]
    SSI_CR0_DSS_8 = 7,
    #[doc = "8: 9-bit data"]
    SSI_CR0_DSS_9 = 8,
    #[doc = "9: 10-bit data"]
    SSI_CR0_DSS_10 = 9,
    #[doc = "10: 11-bit data"]
    SSI_CR0_DSS_11 = 10,
    #[doc = "11: 12-bit data"]
    SSI_CR0_DSS_12 = 11,
    #[doc = "12: 13-bit data"]
    SSI_CR0_DSS_13 = 12,
    #[doc = "13: 14-bit data"]
    SSI_CR0_DSS_14 = 13,
    #[doc = "14: 15-bit data"]
    SSI_CR0_DSS_15 = 14,
    #[doc = "15: 16-bit data"]
    SSI_CR0_DSS_16 = 15,
}
impl From<SSI_CR0_DSS_A> for u8 {
    #[inline(always)]
    fn from(variant: SSI_CR0_DSS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SSI_CR0_DSS` reader - SSI Data Size Select"]
pub type SSI_CR0_DSS_R = crate::FieldReader<u8, SSI_CR0_DSS_A>;
impl SSI_CR0_DSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SSI_CR0_DSS_A> {
        match self.bits {
            3 => Some(SSI_CR0_DSS_A::SSI_CR0_DSS_4),
            4 => Some(SSI_CR0_DSS_A::SSI_CR0_DSS_5),
            5 => Some(SSI_CR0_DSS_A::SSI_CR0_DSS_6),
            6 => Some(SSI_CR0_DSS_A::SSI_CR0_DSS_7),
            7 => Some(SSI_CR0_DSS_A::SSI_CR0_DSS_8),
            8 => Some(SSI_CR0_DSS_A::SSI_CR0_DSS_9),
            9 => Some(SSI_CR0_DSS_A::SSI_CR0_DSS_10),
            10 => Some(SSI_CR0_DSS_A::SSI_CR0_DSS_11),
            11 => Some(SSI_CR0_DSS_A::SSI_CR0_DSS_12),
            12 => Some(SSI_CR0_DSS_A::SSI_CR0_DSS_13),
            13 => Some(SSI_CR0_DSS_A::SSI_CR0_DSS_14),
            14 => Some(SSI_CR0_DSS_A::SSI_CR0_DSS_15),
            15 => Some(SSI_CR0_DSS_A::SSI_CR0_DSS_16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_4`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_4(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_4
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_5`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_5(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_5
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_6`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_6(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_6
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_7`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_7(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_7
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_8`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_8(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_8
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_9`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_9(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_9
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_10`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_10(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_10
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_11`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_11(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_11
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_12`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_12(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_12
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_13`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_13(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_13
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_14`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_14(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_14
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_15`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_15(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_15
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_16`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_16(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_16
    }
}
#[doc = "Field `SSI_CR0_DSS` writer - SSI Data Size Select"]
pub type SSI_CR0_DSS_W<'a> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, SSI_CR0_DSS_A, 4, 0>;
impl<'a> SSI_CR0_DSS_W<'a> {
    #[doc = "4-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_4(self) -> &'a mut W {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_4)
    }
    #[doc = "5-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_5(self) -> &'a mut W {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_5)
    }
    #[doc = "6-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_6(self) -> &'a mut W {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_6)
    }
    #[doc = "7-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_7(self) -> &'a mut W {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_7)
    }
    #[doc = "8-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_8(self) -> &'a mut W {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_8)
    }
    #[doc = "9-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_9(self) -> &'a mut W {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_9)
    }
    #[doc = "10-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_10(self) -> &'a mut W {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_10)
    }
    #[doc = "11-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_11(self) -> &'a mut W {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_11)
    }
    #[doc = "12-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_12(self) -> &'a mut W {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_12)
    }
    #[doc = "13-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_13(self) -> &'a mut W {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_13)
    }
    #[doc = "14-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_14(self) -> &'a mut W {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_14)
    }
    #[doc = "15-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_15(self) -> &'a mut W {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_15)
    }
    #[doc = "16-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_16(self) -> &'a mut W {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_16)
    }
}
#[doc = "SSI Frame Format Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSI_CR0_FRF_A {
    #[doc = "0: Freescale SPI Frame Format"]
    SSI_CR0_FRF_MOTO = 0,
    #[doc = "1: Synchronous Serial Frame Format"]
    SSI_CR0_FRF_TI = 1,
}
impl From<SSI_CR0_FRF_A> for u8 {
    #[inline(always)]
    fn from(variant: SSI_CR0_FRF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SSI_CR0_FRF` reader - SSI Frame Format Select"]
pub type SSI_CR0_FRF_R = crate::FieldReader<u8, SSI_CR0_FRF_A>;
impl SSI_CR0_FRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SSI_CR0_FRF_A> {
        match self.bits {
            0 => Some(SSI_CR0_FRF_A::SSI_CR0_FRF_MOTO),
            1 => Some(SSI_CR0_FRF_A::SSI_CR0_FRF_TI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_FRF_MOTO`"]
    #[inline(always)]
    pub fn is_ssi_cr0_frf_moto(&self) -> bool {
        *self == SSI_CR0_FRF_A::SSI_CR0_FRF_MOTO
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_FRF_TI`"]
    #[inline(always)]
    pub fn is_ssi_cr0_frf_ti(&self) -> bool {
        *self == SSI_CR0_FRF_A::SSI_CR0_FRF_TI
    }
}
#[doc = "Field `SSI_CR0_FRF` writer - SSI Frame Format Select"]
pub type SSI_CR0_FRF_W<'a> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, SSI_CR0_FRF_A, 2, 4>;
impl<'a> SSI_CR0_FRF_W<'a> {
    #[doc = "Freescale SPI Frame Format"]
    #[inline(always)]
    pub fn ssi_cr0_frf_moto(self) -> &'a mut W {
        self.variant(SSI_CR0_FRF_A::SSI_CR0_FRF_MOTO)
    }
    #[doc = "Synchronous Serial Frame Format"]
    #[inline(always)]
    pub fn ssi_cr0_frf_ti(self) -> &'a mut W {
        self.variant(SSI_CR0_FRF_A::SSI_CR0_FRF_TI)
    }
}
#[doc = "Field `SSI_CR0_SPO` reader - SSI Serial Clock Polarity"]
pub type SSI_CR0_SPO_R = crate::BitReader<bool>;
#[doc = "Field `SSI_CR0_SPO` writer - SSI Serial Clock Polarity"]
pub type SSI_CR0_SPO_W<'a> = crate::BitWriter<'a, u32, CR0_SPEC, bool, 6>;
#[doc = "Field `SSI_CR0_SPH` reader - SSI Serial Clock Phase"]
pub type SSI_CR0_SPH_R = crate::BitReader<bool>;
#[doc = "Field `SSI_CR0_SPH` writer - SSI Serial Clock Phase"]
pub type SSI_CR0_SPH_W<'a> = crate::BitWriter<'a, u32, CR0_SPEC, bool, 7>;
#[doc = "Field `SSI_CR0_SCR` reader - SSI Serial Clock Rate"]
pub type SSI_CR0_SCR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SSI_CR0_SCR` writer - SSI Serial Clock Rate"]
pub type SSI_CR0_SCR_W<'a> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 8, 8>;
impl R {
    #[doc = "Bits 0:3 - SSI Data Size Select"]
    #[inline(always)]
    pub fn ssi_cr0_dss(&self) -> SSI_CR0_DSS_R {
        SSI_CR0_DSS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - SSI Frame Format Select"]
    #[inline(always)]
    pub fn ssi_cr0_frf(&self) -> SSI_CR0_FRF_R {
        SSI_CR0_FRF_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - SSI Serial Clock Polarity"]
    #[inline(always)]
    pub fn ssi_cr0_spo(&self) -> SSI_CR0_SPO_R {
        SSI_CR0_SPO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SSI Serial Clock Phase"]
    #[inline(always)]
    pub fn ssi_cr0_sph(&self) -> SSI_CR0_SPH_R {
        SSI_CR0_SPH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - SSI Serial Clock Rate"]
    #[inline(always)]
    pub fn ssi_cr0_scr(&self) -> SSI_CR0_SCR_R {
        SSI_CR0_SCR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SSI Data Size Select"]
    #[inline(always)]
    pub fn ssi_cr0_dss(&mut self) -> SSI_CR0_DSS_W {
        SSI_CR0_DSS_W::new(self)
    }
    #[doc = "Bits 4:5 - SSI Frame Format Select"]
    #[inline(always)]
    pub fn ssi_cr0_frf(&mut self) -> SSI_CR0_FRF_W {
        SSI_CR0_FRF_W::new(self)
    }
    #[doc = "Bit 6 - SSI Serial Clock Polarity"]
    #[inline(always)]
    pub fn ssi_cr0_spo(&mut self) -> SSI_CR0_SPO_W {
        SSI_CR0_SPO_W::new(self)
    }
    #[doc = "Bit 7 - SSI Serial Clock Phase"]
    #[inline(always)]
    pub fn ssi_cr0_sph(&mut self) -> SSI_CR0_SPH_W {
        SSI_CR0_SPH_W::new(self)
    }
    #[doc = "Bits 8:15 - SSI Serial Clock Rate"]
    #[inline(always)]
    pub fn ssi_cr0_scr(&mut self) -> SSI_CR0_SCR_W {
        SSI_CR0_SCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SSI Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr0](index.html) module"]
pub struct CR0_SPEC;
impl crate::RegisterSpec for CR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr0::R](R) reader structure"]
impl crate::Readable for CR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr0::W](W) writer structure"]
impl crate::Writable for CR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for CR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
