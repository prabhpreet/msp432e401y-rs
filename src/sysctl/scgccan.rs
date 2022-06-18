#[doc = "Register `SCGCCAN` reader"]
pub struct R(crate::R<SCGCCAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGCCAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGCCAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGCCAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGCCAN` writer"]
pub struct W(crate::W<SCGCCAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGCCAN_SPEC>;
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
impl From<crate::W<SCGCCAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGCCAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SCGCCAN_S0` reader - CAN Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCCAN_S0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCCAN_S0` writer - CAN Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCCAN_S0_W<'a> = crate::BitWriter<'a, u32, SCGCCAN_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_SCGCCAN_S1` reader - CAN Module 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCCAN_S1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCCAN_S1` writer - CAN Module 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCCAN_S1_W<'a> = crate::BitWriter<'a, u32, SCGCCAN_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - CAN Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgccan_s0(&self) -> SYSCTL_SCGCCAN_S0_R {
        SYSCTL_SCGCCAN_S0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CAN Module 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgccan_s1(&self) -> SYSCTL_SCGCCAN_S1_R {
        SYSCTL_SCGCCAN_S1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAN Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgccan_s0(&mut self) -> SYSCTL_SCGCCAN_S0_W {
        SYSCTL_SCGCCAN_S0_W::new(self)
    }
    #[doc = "Bit 1 - CAN Module 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgccan_s1(&mut self) -> SYSCTL_SCGCCAN_S1_W {
        SYSCTL_SCGCCAN_S1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controller Area Network Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgccan](index.html) module"]
pub struct SCGCCAN_SPEC;
impl crate::RegisterSpec for SCGCCAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgccan::R](R) reader structure"]
impl crate::Readable for SCGCCAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgccan::W](W) writer structure"]
impl crate::Writable for SCGCCAN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCGCCAN to value 0"]
impl crate::Resettable for SCGCCAN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
