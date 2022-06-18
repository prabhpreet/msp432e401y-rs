#[doc = "Register `RCGCCAN` reader"]
pub struct R(crate::R<RCGCCAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCGCCAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCGCCAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCGCCAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCGCCAN` writer"]
pub struct W(crate::W<RCGCCAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCGCCAN_SPEC>;
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
impl From<crate::W<RCGCCAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCGCCAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_RCGCCAN_R0` reader - CAN Module 0 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCCAN_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RCGCCAN_R0` writer - CAN Module 0 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCCAN_R0_W<'a> = crate::BitWriter<'a, u32, RCGCCAN_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_RCGCCAN_R1` reader - CAN Module 1 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCCAN_R1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RCGCCAN_R1` writer - CAN Module 1 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCCAN_R1_W<'a> = crate::BitWriter<'a, u32, RCGCCAN_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - CAN Module 0 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgccan_r0(&self) -> SYSCTL_RCGCCAN_R0_R {
        SYSCTL_RCGCCAN_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CAN Module 1 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgccan_r1(&self) -> SYSCTL_RCGCCAN_R1_R {
        SYSCTL_RCGCCAN_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAN Module 0 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgccan_r0(&mut self) -> SYSCTL_RCGCCAN_R0_W {
        SYSCTL_RCGCCAN_R0_W::new(self)
    }
    #[doc = "Bit 1 - CAN Module 1 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgccan_r1(&mut self) -> SYSCTL_RCGCCAN_R1_W {
        SYSCTL_RCGCCAN_R1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controller Area Network Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgccan](index.html) module"]
pub struct RCGCCAN_SPEC;
impl crate::RegisterSpec for RCGCCAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcgccan::R](R) reader structure"]
impl crate::Readable for RCGCCAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcgccan::W](W) writer structure"]
impl crate::Writable for RCGCCAN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCGCCAN to value 0"]
impl crate::Resettable for RCGCCAN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
