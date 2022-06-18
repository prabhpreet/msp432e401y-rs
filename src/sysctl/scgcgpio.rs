#[doc = "Register `SCGCGPIO` reader"]
pub struct R(crate::R<SCGCGPIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGCGPIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGCGPIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGCGPIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGCGPIO` writer"]
pub struct W(crate::W<SCGCGPIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGCGPIO_SPEC>;
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
impl From<crate::W<SCGCGPIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGCGPIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SCGCGPIO_S0` reader - GPIO Port A Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCGPIO_S0` writer - GPIO Port A Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S0_W<'a> = crate::BitWriter<'a, u32, SCGCGPIO_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_SCGCGPIO_S1` reader - GPIO Port B Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCGPIO_S1` writer - GPIO Port B Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S1_W<'a> = crate::BitWriter<'a, u32, SCGCGPIO_SPEC, bool, 1>;
#[doc = "Field `SYSCTL_SCGCGPIO_S2` reader - GPIO Port C Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S2_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCGPIO_S2` writer - GPIO Port C Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S2_W<'a> = crate::BitWriter<'a, u32, SCGCGPIO_SPEC, bool, 2>;
#[doc = "Field `SYSCTL_SCGCGPIO_S3` reader - GPIO Port D Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S3_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCGPIO_S3` writer - GPIO Port D Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S3_W<'a> = crate::BitWriter<'a, u32, SCGCGPIO_SPEC, bool, 3>;
#[doc = "Field `SYSCTL_SCGCGPIO_S4` reader - GPIO Port E Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S4_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCGPIO_S4` writer - GPIO Port E Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S4_W<'a> = crate::BitWriter<'a, u32, SCGCGPIO_SPEC, bool, 4>;
#[doc = "Field `SYSCTL_SCGCGPIO_S5` reader - GPIO Port F Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S5_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCGPIO_S5` writer - GPIO Port F Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S5_W<'a> = crate::BitWriter<'a, u32, SCGCGPIO_SPEC, bool, 5>;
#[doc = "Field `SYSCTL_SCGCGPIO_S6` reader - GPIO Port G Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S6_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCGPIO_S6` writer - GPIO Port G Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S6_W<'a> = crate::BitWriter<'a, u32, SCGCGPIO_SPEC, bool, 6>;
#[doc = "Field `SYSCTL_SCGCGPIO_S7` reader - GPIO Port H Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S7_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCGPIO_S7` writer - GPIO Port H Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S7_W<'a> = crate::BitWriter<'a, u32, SCGCGPIO_SPEC, bool, 7>;
#[doc = "Field `SYSCTL_SCGCGPIO_S8` reader - GPIO Port J Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S8_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCGPIO_S8` writer - GPIO Port J Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S8_W<'a> = crate::BitWriter<'a, u32, SCGCGPIO_SPEC, bool, 8>;
#[doc = "Field `SYSCTL_SCGCGPIO_S9` reader - GPIO Port K Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S9_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCGPIO_S9` writer - GPIO Port K Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S9_W<'a> = crate::BitWriter<'a, u32, SCGCGPIO_SPEC, bool, 9>;
#[doc = "Field `SYSCTL_SCGCGPIO_S10` reader - GPIO Port L Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S10_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCGPIO_S10` writer - GPIO Port L Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S10_W<'a> = crate::BitWriter<'a, u32, SCGCGPIO_SPEC, bool, 10>;
#[doc = "Field `SYSCTL_SCGCGPIO_S11` reader - GPIO Port M Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S11_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCGPIO_S11` writer - GPIO Port M Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S11_W<'a> = crate::BitWriter<'a, u32, SCGCGPIO_SPEC, bool, 11>;
#[doc = "Field `SYSCTL_SCGCGPIO_S12` reader - GPIO Port N Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S12_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCGPIO_S12` writer - GPIO Port N Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S12_W<'a> = crate::BitWriter<'a, u32, SCGCGPIO_SPEC, bool, 12>;
#[doc = "Field `SYSCTL_SCGCGPIO_S13` reader - GPIO Port P Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S13_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCGPIO_S13` writer - GPIO Port P Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S13_W<'a> = crate::BitWriter<'a, u32, SCGCGPIO_SPEC, bool, 13>;
#[doc = "Field `SYSCTL_SCGCGPIO_S14` reader - GPIO Port Q Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S14_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCGPIO_S14` writer - GPIO Port Q Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S14_W<'a> = crate::BitWriter<'a, u32, SCGCGPIO_SPEC, bool, 14>;
impl R {
    #[doc = "Bit 0 - GPIO Port A Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s0(&self) -> SYSCTL_SCGCGPIO_S0_R {
        SYSCTL_SCGCGPIO_S0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO Port B Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s1(&self) -> SYSCTL_SCGCGPIO_S1_R {
        SYSCTL_SCGCGPIO_S1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO Port C Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s2(&self) -> SYSCTL_SCGCGPIO_S2_R {
        SYSCTL_SCGCGPIO_S2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO Port D Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s3(&self) -> SYSCTL_SCGCGPIO_S3_R {
        SYSCTL_SCGCGPIO_S3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO Port E Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s4(&self) -> SYSCTL_SCGCGPIO_S4_R {
        SYSCTL_SCGCGPIO_S4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO Port F Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s5(&self) -> SYSCTL_SCGCGPIO_S5_R {
        SYSCTL_SCGCGPIO_S5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO Port G Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s6(&self) -> SYSCTL_SCGCGPIO_S6_R {
        SYSCTL_SCGCGPIO_S6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO Port H Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s7(&self) -> SYSCTL_SCGCGPIO_S7_R {
        SYSCTL_SCGCGPIO_S7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO Port J Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s8(&self) -> SYSCTL_SCGCGPIO_S8_R {
        SYSCTL_SCGCGPIO_S8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO Port K Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s9(&self) -> SYSCTL_SCGCGPIO_S9_R {
        SYSCTL_SCGCGPIO_S9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO Port L Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s10(&self) -> SYSCTL_SCGCGPIO_S10_R {
        SYSCTL_SCGCGPIO_S10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO Port M Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s11(&self) -> SYSCTL_SCGCGPIO_S11_R {
        SYSCTL_SCGCGPIO_S11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIO Port N Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s12(&self) -> SYSCTL_SCGCGPIO_S12_R {
        SYSCTL_SCGCGPIO_S12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIO Port P Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s13(&self) -> SYSCTL_SCGCGPIO_S13_R {
        SYSCTL_SCGCGPIO_S13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO Port Q Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s14(&self) -> SYSCTL_SCGCGPIO_S14_R {
        SYSCTL_SCGCGPIO_S14_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO Port A Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s0(&mut self) -> SYSCTL_SCGCGPIO_S0_W {
        SYSCTL_SCGCGPIO_S0_W::new(self)
    }
    #[doc = "Bit 1 - GPIO Port B Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s1(&mut self) -> SYSCTL_SCGCGPIO_S1_W {
        SYSCTL_SCGCGPIO_S1_W::new(self)
    }
    #[doc = "Bit 2 - GPIO Port C Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s2(&mut self) -> SYSCTL_SCGCGPIO_S2_W {
        SYSCTL_SCGCGPIO_S2_W::new(self)
    }
    #[doc = "Bit 3 - GPIO Port D Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s3(&mut self) -> SYSCTL_SCGCGPIO_S3_W {
        SYSCTL_SCGCGPIO_S3_W::new(self)
    }
    #[doc = "Bit 4 - GPIO Port E Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s4(&mut self) -> SYSCTL_SCGCGPIO_S4_W {
        SYSCTL_SCGCGPIO_S4_W::new(self)
    }
    #[doc = "Bit 5 - GPIO Port F Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s5(&mut self) -> SYSCTL_SCGCGPIO_S5_W {
        SYSCTL_SCGCGPIO_S5_W::new(self)
    }
    #[doc = "Bit 6 - GPIO Port G Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s6(&mut self) -> SYSCTL_SCGCGPIO_S6_W {
        SYSCTL_SCGCGPIO_S6_W::new(self)
    }
    #[doc = "Bit 7 - GPIO Port H Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s7(&mut self) -> SYSCTL_SCGCGPIO_S7_W {
        SYSCTL_SCGCGPIO_S7_W::new(self)
    }
    #[doc = "Bit 8 - GPIO Port J Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s8(&mut self) -> SYSCTL_SCGCGPIO_S8_W {
        SYSCTL_SCGCGPIO_S8_W::new(self)
    }
    #[doc = "Bit 9 - GPIO Port K Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s9(&mut self) -> SYSCTL_SCGCGPIO_S9_W {
        SYSCTL_SCGCGPIO_S9_W::new(self)
    }
    #[doc = "Bit 10 - GPIO Port L Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s10(&mut self) -> SYSCTL_SCGCGPIO_S10_W {
        SYSCTL_SCGCGPIO_S10_W::new(self)
    }
    #[doc = "Bit 11 - GPIO Port M Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s11(&mut self) -> SYSCTL_SCGCGPIO_S11_W {
        SYSCTL_SCGCGPIO_S11_W::new(self)
    }
    #[doc = "Bit 12 - GPIO Port N Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s12(&mut self) -> SYSCTL_SCGCGPIO_S12_W {
        SYSCTL_SCGCGPIO_S12_W::new(self)
    }
    #[doc = "Bit 13 - GPIO Port P Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s13(&mut self) -> SYSCTL_SCGCGPIO_S13_W {
        SYSCTL_SCGCGPIO_S13_W::new(self)
    }
    #[doc = "Bit 14 - GPIO Port Q Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s14(&mut self) -> SYSCTL_SCGCGPIO_S14_W {
        SYSCTL_SCGCGPIO_S14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General-Purpose Input/Output Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcgpio](index.html) module"]
pub struct SCGCGPIO_SPEC;
impl crate::RegisterSpec for SCGCGPIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgcgpio::R](R) reader structure"]
impl crate::Readable for SCGCGPIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgcgpio::W](W) writer structure"]
impl crate::Writable for SCGCGPIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCGCGPIO to value 0"]
impl crate::Resettable for SCGCGPIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
