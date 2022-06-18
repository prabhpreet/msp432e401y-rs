#[doc = "Register `PRCAN` reader"]
pub struct R(crate::R<PRCAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRCAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRCAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRCAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRCAN` writer"]
pub struct W(crate::W<PRCAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRCAN_SPEC>;
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
impl From<crate::W<PRCAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRCAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PRCAN_R0` reader - CAN Module 0 Peripheral Ready"]
pub type SYSCTL_PRCAN_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRCAN_R0` writer - CAN Module 0 Peripheral Ready"]
pub type SYSCTL_PRCAN_R0_W<'a> = crate::BitWriter<'a, u32, PRCAN_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_PRCAN_R1` reader - CAN Module 1 Peripheral Ready"]
pub type SYSCTL_PRCAN_R1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRCAN_R1` writer - CAN Module 1 Peripheral Ready"]
pub type SYSCTL_PRCAN_R1_W<'a> = crate::BitWriter<'a, u32, PRCAN_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - CAN Module 0 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prcan_r0(&self) -> SYSCTL_PRCAN_R0_R {
        SYSCTL_PRCAN_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CAN Module 1 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prcan_r1(&self) -> SYSCTL_PRCAN_R1_R {
        SYSCTL_PRCAN_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAN Module 0 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prcan_r0(&mut self) -> SYSCTL_PRCAN_R0_W {
        SYSCTL_PRCAN_R0_W::new(self)
    }
    #[doc = "Bit 1 - CAN Module 1 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prcan_r1(&mut self) -> SYSCTL_PRCAN_R1_W {
        SYSCTL_PRCAN_R1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controller Area Network Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prcan](index.html) module"]
pub struct PRCAN_SPEC;
impl crate::RegisterSpec for PRCAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prcan::R](R) reader structure"]
impl crate::Readable for PRCAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prcan::W](W) writer structure"]
impl crate::Writable for PRCAN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRCAN to value 0"]
impl crate::Resettable for PRCAN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
