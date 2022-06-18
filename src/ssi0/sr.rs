#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSI_SR_TFE` reader - SSI Transmit FIFO Empty"]
pub type SSI_SR_TFE_R = crate::BitReader<bool>;
#[doc = "Field `SSI_SR_TFE` writer - SSI Transmit FIFO Empty"]
pub type SSI_SR_TFE_W<'a> = crate::BitWriter<'a, u32, SR_SPEC, bool, 0>;
#[doc = "Field `SSI_SR_TNF` reader - SSI Transmit FIFO Not Full"]
pub type SSI_SR_TNF_R = crate::BitReader<bool>;
#[doc = "Field `SSI_SR_TNF` writer - SSI Transmit FIFO Not Full"]
pub type SSI_SR_TNF_W<'a> = crate::BitWriter<'a, u32, SR_SPEC, bool, 1>;
#[doc = "Field `SSI_SR_RNE` reader - SSI Receive FIFO Not Empty"]
pub type SSI_SR_RNE_R = crate::BitReader<bool>;
#[doc = "Field `SSI_SR_RNE` writer - SSI Receive FIFO Not Empty"]
pub type SSI_SR_RNE_W<'a> = crate::BitWriter<'a, u32, SR_SPEC, bool, 2>;
#[doc = "Field `SSI_SR_RFF` reader - SSI Receive FIFO Full"]
pub type SSI_SR_RFF_R = crate::BitReader<bool>;
#[doc = "Field `SSI_SR_RFF` writer - SSI Receive FIFO Full"]
pub type SSI_SR_RFF_W<'a> = crate::BitWriter<'a, u32, SR_SPEC, bool, 3>;
#[doc = "Field `SSI_SR_BSY` reader - SSI Busy Bit"]
pub type SSI_SR_BSY_R = crate::BitReader<bool>;
#[doc = "Field `SSI_SR_BSY` writer - SSI Busy Bit"]
pub type SSI_SR_BSY_W<'a> = crate::BitWriter<'a, u32, SR_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 0 - SSI Transmit FIFO Empty"]
    #[inline(always)]
    pub fn ssi_sr_tfe(&self) -> SSI_SR_TFE_R {
        SSI_SR_TFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI Transmit FIFO Not Full"]
    #[inline(always)]
    pub fn ssi_sr_tnf(&self) -> SSI_SR_TNF_R {
        SSI_SR_TNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI Receive FIFO Not Empty"]
    #[inline(always)]
    pub fn ssi_sr_rne(&self) -> SSI_SR_RNE_R {
        SSI_SR_RNE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSI Receive FIFO Full"]
    #[inline(always)]
    pub fn ssi_sr_rff(&self) -> SSI_SR_RFF_R {
        SSI_SR_RFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SSI Busy Bit"]
    #[inline(always)]
    pub fn ssi_sr_bsy(&self) -> SSI_SR_BSY_R {
        SSI_SR_BSY_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Transmit FIFO Empty"]
    #[inline(always)]
    pub fn ssi_sr_tfe(&mut self) -> SSI_SR_TFE_W {
        SSI_SR_TFE_W::new(self)
    }
    #[doc = "Bit 1 - SSI Transmit FIFO Not Full"]
    #[inline(always)]
    pub fn ssi_sr_tnf(&mut self) -> SSI_SR_TNF_W {
        SSI_SR_TNF_W::new(self)
    }
    #[doc = "Bit 2 - SSI Receive FIFO Not Empty"]
    #[inline(always)]
    pub fn ssi_sr_rne(&mut self) -> SSI_SR_RNE_W {
        SSI_SR_RNE_W::new(self)
    }
    #[doc = "Bit 3 - SSI Receive FIFO Full"]
    #[inline(always)]
    pub fn ssi_sr_rff(&mut self) -> SSI_SR_RFF_W {
        SSI_SR_RFF_W::new(self)
    }
    #[doc = "Bit 4 - SSI Busy Bit"]
    #[inline(always)]
    pub fn ssi_sr_bsy(&mut self) -> SSI_SR_BSY_W {
        SSI_SR_BSY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SSI Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
