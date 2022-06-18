#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSI_CR1_LBM` reader - SSI Loopback Mode"]
pub type SSI_CR1_LBM_R = crate::BitReader<bool>;
#[doc = "Field `SSI_CR1_LBM` writer - SSI Loopback Mode"]
pub type SSI_CR1_LBM_W<'a> = crate::BitWriter<'a, u32, CR1_SPEC, bool, 0>;
#[doc = "Field `SSI_CR1_SSE` reader - SSI Synchronous Serial Port Enable"]
pub type SSI_CR1_SSE_R = crate::BitReader<bool>;
#[doc = "Field `SSI_CR1_SSE` writer - SSI Synchronous Serial Port Enable"]
pub type SSI_CR1_SSE_W<'a> = crate::BitWriter<'a, u32, CR1_SPEC, bool, 1>;
#[doc = "Field `SSI_CR1_MS` reader - SSI Master/Slave Select"]
pub type SSI_CR1_MS_R = crate::BitReader<bool>;
#[doc = "Field `SSI_CR1_MS` writer - SSI Master/Slave Select"]
pub type SSI_CR1_MS_W<'a> = crate::BitWriter<'a, u32, CR1_SPEC, bool, 2>;
#[doc = "Field `SSI_CR1_EOT` reader - End of Transmission"]
pub type SSI_CR1_EOT_R = crate::BitReader<bool>;
#[doc = "Field `SSI_CR1_EOT` writer - End of Transmission"]
pub type SSI_CR1_EOT_W<'a> = crate::BitWriter<'a, u32, CR1_SPEC, bool, 4>;
#[doc = "SSI Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSI_CR1_MODE_A {
    #[doc = "0: Legacy SSI mode"]
    SSI_CR1_MODE_LEGACY = 0,
    #[doc = "1: Bi-SSI mode"]
    SSI_CR1_MODE_BI = 1,
    #[doc = "2: Quad-SSI Mode"]
    SSI_CR1_MODE_QUAD = 2,
    #[doc = "3: Advanced SSI Mode with 8-bit packet size"]
    SSI_CR1_MODE_ADVANCED = 3,
}
impl From<SSI_CR1_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: SSI_CR1_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SSI_CR1_MODE` reader - SSI Mode"]
pub type SSI_CR1_MODE_R = crate::FieldReader<u8, SSI_CR1_MODE_A>;
impl SSI_CR1_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSI_CR1_MODE_A {
        match self.bits {
            0 => SSI_CR1_MODE_A::SSI_CR1_MODE_LEGACY,
            1 => SSI_CR1_MODE_A::SSI_CR1_MODE_BI,
            2 => SSI_CR1_MODE_A::SSI_CR1_MODE_QUAD,
            3 => SSI_CR1_MODE_A::SSI_CR1_MODE_ADVANCED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SSI_CR1_MODE_LEGACY`"]
    #[inline(always)]
    pub fn is_ssi_cr1_mode_legacy(&self) -> bool {
        *self == SSI_CR1_MODE_A::SSI_CR1_MODE_LEGACY
    }
    #[doc = "Checks if the value of the field is `SSI_CR1_MODE_BI`"]
    #[inline(always)]
    pub fn is_ssi_cr1_mode_bi(&self) -> bool {
        *self == SSI_CR1_MODE_A::SSI_CR1_MODE_BI
    }
    #[doc = "Checks if the value of the field is `SSI_CR1_MODE_QUAD`"]
    #[inline(always)]
    pub fn is_ssi_cr1_mode_quad(&self) -> bool {
        *self == SSI_CR1_MODE_A::SSI_CR1_MODE_QUAD
    }
    #[doc = "Checks if the value of the field is `SSI_CR1_MODE_ADVANCED`"]
    #[inline(always)]
    pub fn is_ssi_cr1_mode_advanced(&self) -> bool {
        *self == SSI_CR1_MODE_A::SSI_CR1_MODE_ADVANCED
    }
}
#[doc = "Field `SSI_CR1_MODE` writer - SSI Mode"]
pub type SSI_CR1_MODE_W<'a> = crate::FieldWriterSafe<'a, u32, CR1_SPEC, u8, SSI_CR1_MODE_A, 2, 6>;
impl<'a> SSI_CR1_MODE_W<'a> {
    #[doc = "Legacy SSI mode"]
    #[inline(always)]
    pub fn ssi_cr1_mode_legacy(self) -> &'a mut W {
        self.variant(SSI_CR1_MODE_A::SSI_CR1_MODE_LEGACY)
    }
    #[doc = "Bi-SSI mode"]
    #[inline(always)]
    pub fn ssi_cr1_mode_bi(self) -> &'a mut W {
        self.variant(SSI_CR1_MODE_A::SSI_CR1_MODE_BI)
    }
    #[doc = "Quad-SSI Mode"]
    #[inline(always)]
    pub fn ssi_cr1_mode_quad(self) -> &'a mut W {
        self.variant(SSI_CR1_MODE_A::SSI_CR1_MODE_QUAD)
    }
    #[doc = "Advanced SSI Mode with 8-bit packet size"]
    #[inline(always)]
    pub fn ssi_cr1_mode_advanced(self) -> &'a mut W {
        self.variant(SSI_CR1_MODE_A::SSI_CR1_MODE_ADVANCED)
    }
}
#[doc = "Field `SSI_CR1_DIR` reader - SSI Direction of Operation"]
pub type SSI_CR1_DIR_R = crate::BitReader<bool>;
#[doc = "Field `SSI_CR1_DIR` writer - SSI Direction of Operation"]
pub type SSI_CR1_DIR_W<'a> = crate::BitWriter<'a, u32, CR1_SPEC, bool, 8>;
#[doc = "Field `SSI_CR1_HSCLKEN` reader - High Speed Clock Enable"]
pub type SSI_CR1_HSCLKEN_R = crate::BitReader<bool>;
#[doc = "Field `SSI_CR1_HSCLKEN` writer - High Speed Clock Enable"]
pub type SSI_CR1_HSCLKEN_W<'a> = crate::BitWriter<'a, u32, CR1_SPEC, bool, 9>;
#[doc = "Field `SSI_CR1_FSSHLDFRM` reader - FSS Hold Frame"]
pub type SSI_CR1_FSSHLDFRM_R = crate::BitReader<bool>;
#[doc = "Field `SSI_CR1_FSSHLDFRM` writer - FSS Hold Frame"]
pub type SSI_CR1_FSSHLDFRM_W<'a> = crate::BitWriter<'a, u32, CR1_SPEC, bool, 10>;
#[doc = "Field `SSI_CR1_EOM` reader - Stop Frame (End of Message)"]
pub type SSI_CR1_EOM_R = crate::BitReader<bool>;
#[doc = "Field `SSI_CR1_EOM` writer - Stop Frame (End of Message)"]
pub type SSI_CR1_EOM_W<'a> = crate::BitWriter<'a, u32, CR1_SPEC, bool, 11>;
impl R {
    #[doc = "Bit 0 - SSI Loopback Mode"]
    #[inline(always)]
    pub fn ssi_cr1_lbm(&self) -> SSI_CR1_LBM_R {
        SSI_CR1_LBM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI Synchronous Serial Port Enable"]
    #[inline(always)]
    pub fn ssi_cr1_sse(&self) -> SSI_CR1_SSE_R {
        SSI_CR1_SSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI Master/Slave Select"]
    #[inline(always)]
    pub fn ssi_cr1_ms(&self) -> SSI_CR1_MS_R {
        SSI_CR1_MS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Transmission"]
    #[inline(always)]
    pub fn ssi_cr1_eot(&self) -> SSI_CR1_EOT_R {
        SSI_CR1_EOT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - SSI Mode"]
    #[inline(always)]
    pub fn ssi_cr1_mode(&self) -> SSI_CR1_MODE_R {
        SSI_CR1_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - SSI Direction of Operation"]
    #[inline(always)]
    pub fn ssi_cr1_dir(&self) -> SSI_CR1_DIR_R {
        SSI_CR1_DIR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - High Speed Clock Enable"]
    #[inline(always)]
    pub fn ssi_cr1_hsclken(&self) -> SSI_CR1_HSCLKEN_R {
        SSI_CR1_HSCLKEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FSS Hold Frame"]
    #[inline(always)]
    pub fn ssi_cr1_fsshldfrm(&self) -> SSI_CR1_FSSHLDFRM_R {
        SSI_CR1_FSSHLDFRM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Stop Frame (End of Message)"]
    #[inline(always)]
    pub fn ssi_cr1_eom(&self) -> SSI_CR1_EOM_R {
        SSI_CR1_EOM_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Loopback Mode"]
    #[inline(always)]
    pub fn ssi_cr1_lbm(&mut self) -> SSI_CR1_LBM_W {
        SSI_CR1_LBM_W::new(self)
    }
    #[doc = "Bit 1 - SSI Synchronous Serial Port Enable"]
    #[inline(always)]
    pub fn ssi_cr1_sse(&mut self) -> SSI_CR1_SSE_W {
        SSI_CR1_SSE_W::new(self)
    }
    #[doc = "Bit 2 - SSI Master/Slave Select"]
    #[inline(always)]
    pub fn ssi_cr1_ms(&mut self) -> SSI_CR1_MS_W {
        SSI_CR1_MS_W::new(self)
    }
    #[doc = "Bit 4 - End of Transmission"]
    #[inline(always)]
    pub fn ssi_cr1_eot(&mut self) -> SSI_CR1_EOT_W {
        SSI_CR1_EOT_W::new(self)
    }
    #[doc = "Bits 6:7 - SSI Mode"]
    #[inline(always)]
    pub fn ssi_cr1_mode(&mut self) -> SSI_CR1_MODE_W {
        SSI_CR1_MODE_W::new(self)
    }
    #[doc = "Bit 8 - SSI Direction of Operation"]
    #[inline(always)]
    pub fn ssi_cr1_dir(&mut self) -> SSI_CR1_DIR_W {
        SSI_CR1_DIR_W::new(self)
    }
    #[doc = "Bit 9 - High Speed Clock Enable"]
    #[inline(always)]
    pub fn ssi_cr1_hsclken(&mut self) -> SSI_CR1_HSCLKEN_W {
        SSI_CR1_HSCLKEN_W::new(self)
    }
    #[doc = "Bit 10 - FSS Hold Frame"]
    #[inline(always)]
    pub fn ssi_cr1_fsshldfrm(&mut self) -> SSI_CR1_FSSHLDFRM_W {
        SSI_CR1_FSSHLDFRM_W::new(self)
    }
    #[doc = "Bit 11 - Stop Frame (End of Message)"]
    #[inline(always)]
    pub fn ssi_cr1_eom(&mut self) -> SSI_CR1_EOM_W {
        SSI_CR1_EOM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SSI Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
