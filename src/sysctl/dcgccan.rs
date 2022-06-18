#[doc = "Register `DCGCCAN` reader"]
pub struct R(crate::R<DCGCCAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCGCCAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCGCCAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCGCCAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCGCCAN` writer"]
pub struct W(crate::W<DCGCCAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCGCCAN_SPEC>;
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
impl From<crate::W<DCGCCAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCGCCAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_DCGCCAN_D0` reader - CAN Module 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCCAN_D0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCCAN_D0` writer - CAN Module 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCCAN_D0_W<'a> = crate::BitWriter<'a, u32, DCGCCAN_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_DCGCCAN_D1` reader - CAN Module 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCCAN_D1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCCAN_D1` writer - CAN Module 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCCAN_D1_W<'a> = crate::BitWriter<'a, u32, DCGCCAN_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - CAN Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgccan_d0(&self) -> SYSCTL_DCGCCAN_D0_R {
        SYSCTL_DCGCCAN_D0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CAN Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgccan_d1(&self) -> SYSCTL_DCGCCAN_D1_R {
        SYSCTL_DCGCCAN_D1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAN Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgccan_d0(&mut self) -> SYSCTL_DCGCCAN_D0_W {
        SYSCTL_DCGCCAN_D0_W::new(self)
    }
    #[doc = "Bit 1 - CAN Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgccan_d1(&mut self) -> SYSCTL_DCGCCAN_D1_W {
        SYSCTL_DCGCCAN_D1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controller Area Network Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgccan](index.html) module"]
pub struct DCGCCAN_SPEC;
impl crate::RegisterSpec for DCGCCAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcgccan::R](R) reader structure"]
impl crate::Readable for DCGCCAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcgccan::W](W) writer structure"]
impl crate::Writable for DCGCCAN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCGCCAN to value 0"]
impl crate::Resettable for DCGCCAN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
