#[doc = "Register `PPCAN` reader"]
pub struct R(crate::R<PPCAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPCAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPCAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPCAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPCAN` writer"]
pub struct W(crate::W<PPCAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPCAN_SPEC>;
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
impl From<crate::W<PPCAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPCAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PPCAN_P0` reader - CAN Module 0 Present"]
pub type SYSCTL_PPCAN_P0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPCAN_P0` writer - CAN Module 0 Present"]
pub type SYSCTL_PPCAN_P0_W<'a> = crate::BitWriter<'a, u32, PPCAN_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_PPCAN_P1` reader - CAN Module 1 Present"]
pub type SYSCTL_PPCAN_P1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPCAN_P1` writer - CAN Module 1 Present"]
pub type SYSCTL_PPCAN_P1_W<'a> = crate::BitWriter<'a, u32, PPCAN_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - CAN Module 0 Present"]
    #[inline(always)]
    pub fn sysctl_ppcan_p0(&self) -> SYSCTL_PPCAN_P0_R {
        SYSCTL_PPCAN_P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CAN Module 1 Present"]
    #[inline(always)]
    pub fn sysctl_ppcan_p1(&self) -> SYSCTL_PPCAN_P1_R {
        SYSCTL_PPCAN_P1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAN Module 0 Present"]
    #[inline(always)]
    pub fn sysctl_ppcan_p0(&mut self) -> SYSCTL_PPCAN_P0_W {
        SYSCTL_PPCAN_P0_W::new(self)
    }
    #[doc = "Bit 1 - CAN Module 1 Present"]
    #[inline(always)]
    pub fn sysctl_ppcan_p1(&mut self) -> SYSCTL_PPCAN_P1_W {
        SYSCTL_PPCAN_P1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controller Area Network Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppcan](index.html) module"]
pub struct PPCAN_SPEC;
impl crate::RegisterSpec for PPCAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppcan::R](R) reader structure"]
impl crate::Readable for PPCAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppcan::W](W) writer structure"]
impl crate::Writable for PPCAN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PPCAN to value 0"]
impl crate::Resettable for PPCAN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
