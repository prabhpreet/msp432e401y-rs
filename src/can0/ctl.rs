#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN_CTL_INIT` reader - Initialization"]
pub type CAN_CTL_INIT_R = crate::BitReader<bool>;
#[doc = "Field `CAN_CTL_INIT` writer - Initialization"]
pub type CAN_CTL_INIT_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 0>;
#[doc = "Field `CAN_CTL_IE` reader - CAN Interrupt Enable"]
pub type CAN_CTL_IE_R = crate::BitReader<bool>;
#[doc = "Field `CAN_CTL_IE` writer - CAN Interrupt Enable"]
pub type CAN_CTL_IE_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 1>;
#[doc = "Field `CAN_CTL_SIE` reader - Status Interrupt Enable"]
pub type CAN_CTL_SIE_R = crate::BitReader<bool>;
#[doc = "Field `CAN_CTL_SIE` writer - Status Interrupt Enable"]
pub type CAN_CTL_SIE_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 2>;
#[doc = "Field `CAN_CTL_EIE` reader - Error Interrupt Enable"]
pub type CAN_CTL_EIE_R = crate::BitReader<bool>;
#[doc = "Field `CAN_CTL_EIE` writer - Error Interrupt Enable"]
pub type CAN_CTL_EIE_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 3>;
#[doc = "Field `CAN_CTL_DAR` reader - Disable Automatic-Retransmission"]
pub type CAN_CTL_DAR_R = crate::BitReader<bool>;
#[doc = "Field `CAN_CTL_DAR` writer - Disable Automatic-Retransmission"]
pub type CAN_CTL_DAR_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 5>;
#[doc = "Field `CAN_CTL_CCE` reader - Configuration Change Enable"]
pub type CAN_CTL_CCE_R = crate::BitReader<bool>;
#[doc = "Field `CAN_CTL_CCE` writer - Configuration Change Enable"]
pub type CAN_CTL_CCE_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 6>;
#[doc = "Field `CAN_CTL_TEST` reader - Test Mode Enable"]
pub type CAN_CTL_TEST_R = crate::BitReader<bool>;
#[doc = "Field `CAN_CTL_TEST` writer - Test Mode Enable"]
pub type CAN_CTL_TEST_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Initialization"]
    #[inline(always)]
    pub fn can_ctl_init(&self) -> CAN_CTL_INIT_R {
        CAN_CTL_INIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CAN Interrupt Enable"]
    #[inline(always)]
    pub fn can_ctl_ie(&self) -> CAN_CTL_IE_R {
        CAN_CTL_IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status Interrupt Enable"]
    #[inline(always)]
    pub fn can_ctl_sie(&self) -> CAN_CTL_SIE_R {
        CAN_CTL_SIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn can_ctl_eie(&self) -> CAN_CTL_EIE_R {
        CAN_CTL_EIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable Automatic-Retransmission"]
    #[inline(always)]
    pub fn can_ctl_dar(&self) -> CAN_CTL_DAR_R {
        CAN_CTL_DAR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configuration Change Enable"]
    #[inline(always)]
    pub fn can_ctl_cce(&self) -> CAN_CTL_CCE_R {
        CAN_CTL_CCE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Test Mode Enable"]
    #[inline(always)]
    pub fn can_ctl_test(&self) -> CAN_CTL_TEST_R {
        CAN_CTL_TEST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initialization"]
    #[inline(always)]
    pub fn can_ctl_init(&mut self) -> CAN_CTL_INIT_W {
        CAN_CTL_INIT_W::new(self)
    }
    #[doc = "Bit 1 - CAN Interrupt Enable"]
    #[inline(always)]
    pub fn can_ctl_ie(&mut self) -> CAN_CTL_IE_W {
        CAN_CTL_IE_W::new(self)
    }
    #[doc = "Bit 2 - Status Interrupt Enable"]
    #[inline(always)]
    pub fn can_ctl_sie(&mut self) -> CAN_CTL_SIE_W {
        CAN_CTL_SIE_W::new(self)
    }
    #[doc = "Bit 3 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn can_ctl_eie(&mut self) -> CAN_CTL_EIE_W {
        CAN_CTL_EIE_W::new(self)
    }
    #[doc = "Bit 5 - Disable Automatic-Retransmission"]
    #[inline(always)]
    pub fn can_ctl_dar(&mut self) -> CAN_CTL_DAR_W {
        CAN_CTL_DAR_W::new(self)
    }
    #[doc = "Bit 6 - Configuration Change Enable"]
    #[inline(always)]
    pub fn can_ctl_cce(&mut self) -> CAN_CTL_CCE_W {
        CAN_CTL_CCE_W::new(self)
    }
    #[doc = "Bit 7 - Test Mode Enable"]
    #[inline(always)]
    pub fn can_ctl_test(&mut self) -> CAN_CTL_TEST_W {
        CAN_CTL_TEST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
