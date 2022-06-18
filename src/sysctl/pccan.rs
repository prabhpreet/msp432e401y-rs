#[doc = "Register `PCCAN` reader"]
pub struct R(crate::R<PCCAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCCAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCCAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCCAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCCAN` writer"]
pub struct W(crate::W<PCCAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCCAN_SPEC>;
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
impl From<crate::W<PCCAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCCAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PCCAN_P0` reader - CAN Module 0 Power Control"]
pub type SYSCTL_PCCAN_P0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PCCAN_P0` writer - CAN Module 0 Power Control"]
pub type SYSCTL_PCCAN_P0_W<'a> = crate::BitWriter<'a, u32, PCCAN_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_PCCAN_P1` reader - CAN Module 1 Power Control"]
pub type SYSCTL_PCCAN_P1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PCCAN_P1` writer - CAN Module 1 Power Control"]
pub type SYSCTL_PCCAN_P1_W<'a> = crate::BitWriter<'a, u32, PCCAN_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - CAN Module 0 Power Control"]
    #[inline(always)]
    pub fn sysctl_pccan_p0(&self) -> SYSCTL_PCCAN_P0_R {
        SYSCTL_PCCAN_P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CAN Module 1 Power Control"]
    #[inline(always)]
    pub fn sysctl_pccan_p1(&self) -> SYSCTL_PCCAN_P1_R {
        SYSCTL_PCCAN_P1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAN Module 0 Power Control"]
    #[inline(always)]
    pub fn sysctl_pccan_p0(&mut self) -> SYSCTL_PCCAN_P0_W {
        SYSCTL_PCCAN_P0_W::new(self)
    }
    #[doc = "Bit 1 - CAN Module 1 Power Control"]
    #[inline(always)]
    pub fn sysctl_pccan_p1(&mut self) -> SYSCTL_PCCAN_P1_W {
        SYSCTL_PCCAN_P1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controller Area Network Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pccan](index.html) module"]
pub struct PCCAN_SPEC;
impl crate::RegisterSpec for PCCAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pccan::R](R) reader structure"]
impl crate::Readable for PCCAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pccan::W](W) writer structure"]
impl crate::Writable for PCCAN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCCAN to value 0"]
impl crate::Resettable for PCCAN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
