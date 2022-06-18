#[doc = "Register `PRSSI` reader"]
pub struct R(crate::R<PRSSI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRSSI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRSSI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRSSI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRSSI` writer"]
pub struct W(crate::W<PRSSI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRSSI_SPEC>;
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
impl From<crate::W<PRSSI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRSSI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PRSSI_R0` reader - SSI Module 0 Peripheral Ready"]
pub type SYSCTL_PRSSI_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRSSI_R0` writer - SSI Module 0 Peripheral Ready"]
pub type SYSCTL_PRSSI_R0_W<'a> = crate::BitWriter<'a, u32, PRSSI_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_PRSSI_R1` reader - SSI Module 1 Peripheral Ready"]
pub type SYSCTL_PRSSI_R1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRSSI_R1` writer - SSI Module 1 Peripheral Ready"]
pub type SYSCTL_PRSSI_R1_W<'a> = crate::BitWriter<'a, u32, PRSSI_SPEC, bool, 1>;
#[doc = "Field `SYSCTL_PRSSI_R2` reader - SSI Module 2 Peripheral Ready"]
pub type SYSCTL_PRSSI_R2_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRSSI_R2` writer - SSI Module 2 Peripheral Ready"]
pub type SYSCTL_PRSSI_R2_W<'a> = crate::BitWriter<'a, u32, PRSSI_SPEC, bool, 2>;
#[doc = "Field `SYSCTL_PRSSI_R3` reader - SSI Module 3 Peripheral Ready"]
pub type SYSCTL_PRSSI_R3_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRSSI_R3` writer - SSI Module 3 Peripheral Ready"]
pub type SYSCTL_PRSSI_R3_W<'a> = crate::BitWriter<'a, u32, PRSSI_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - SSI Module 0 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prssi_r0(&self) -> SYSCTL_PRSSI_R0_R {
        SYSCTL_PRSSI_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI Module 1 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prssi_r1(&self) -> SYSCTL_PRSSI_R1_R {
        SYSCTL_PRSSI_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI Module 2 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prssi_r2(&self) -> SYSCTL_PRSSI_R2_R {
        SYSCTL_PRSSI_R2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSI Module 3 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prssi_r3(&self) -> SYSCTL_PRSSI_R3_R {
        SYSCTL_PRSSI_R3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Module 0 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prssi_r0(&mut self) -> SYSCTL_PRSSI_R0_W {
        SYSCTL_PRSSI_R0_W::new(self)
    }
    #[doc = "Bit 1 - SSI Module 1 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prssi_r1(&mut self) -> SYSCTL_PRSSI_R1_W {
        SYSCTL_PRSSI_R1_W::new(self)
    }
    #[doc = "Bit 2 - SSI Module 2 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prssi_r2(&mut self) -> SYSCTL_PRSSI_R2_W {
        SYSCTL_PRSSI_R2_W::new(self)
    }
    #[doc = "Bit 3 - SSI Module 3 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prssi_r3(&mut self) -> SYSCTL_PRSSI_R3_W {
        SYSCTL_PRSSI_R3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Synchronous Serial Interface Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prssi](index.html) module"]
pub struct PRSSI_SPEC;
impl crate::RegisterSpec for PRSSI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prssi::R](R) reader structure"]
impl crate::Readable for PRSSI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prssi::W](W) writer structure"]
impl crate::Writable for PRSSI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRSSI to value 0"]
impl crate::Resettable for PRSSI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
