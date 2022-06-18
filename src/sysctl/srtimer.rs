#[doc = "Register `SRTIMER` reader"]
pub struct R(crate::R<SRTIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRTIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRTIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRTIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRTIMER` writer"]
pub struct W(crate::W<SRTIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRTIMER_SPEC>;
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
impl From<crate::W<SRTIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRTIMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SRTIMER_R0` reader - 16/32-Bit General-Purpose Timer 0 Software Reset"]
pub type SYSCTL_SRTIMER_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SRTIMER_R0` writer - 16/32-Bit General-Purpose Timer 0 Software Reset"]
pub type SYSCTL_SRTIMER_R0_W<'a> = crate::BitWriter<'a, u32, SRTIMER_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_SRTIMER_R1` reader - 16/32-Bit General-Purpose Timer 1 Software Reset"]
pub type SYSCTL_SRTIMER_R1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SRTIMER_R1` writer - 16/32-Bit General-Purpose Timer 1 Software Reset"]
pub type SYSCTL_SRTIMER_R1_W<'a> = crate::BitWriter<'a, u32, SRTIMER_SPEC, bool, 1>;
#[doc = "Field `SYSCTL_SRTIMER_R2` reader - 16/32-Bit General-Purpose Timer 2 Software Reset"]
pub type SYSCTL_SRTIMER_R2_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SRTIMER_R2` writer - 16/32-Bit General-Purpose Timer 2 Software Reset"]
pub type SYSCTL_SRTIMER_R2_W<'a> = crate::BitWriter<'a, u32, SRTIMER_SPEC, bool, 2>;
#[doc = "Field `SYSCTL_SRTIMER_R3` reader - 16/32-Bit General-Purpose Timer 3 Software Reset"]
pub type SYSCTL_SRTIMER_R3_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SRTIMER_R3` writer - 16/32-Bit General-Purpose Timer 3 Software Reset"]
pub type SYSCTL_SRTIMER_R3_W<'a> = crate::BitWriter<'a, u32, SRTIMER_SPEC, bool, 3>;
#[doc = "Field `SYSCTL_SRTIMER_R4` reader - 16/32-Bit General-Purpose Timer 4 Software Reset"]
pub type SYSCTL_SRTIMER_R4_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SRTIMER_R4` writer - 16/32-Bit General-Purpose Timer 4 Software Reset"]
pub type SYSCTL_SRTIMER_R4_W<'a> = crate::BitWriter<'a, u32, SRTIMER_SPEC, bool, 4>;
#[doc = "Field `SYSCTL_SRTIMER_R5` reader - 16/32-Bit General-Purpose Timer 5 Software Reset"]
pub type SYSCTL_SRTIMER_R5_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SRTIMER_R5` writer - 16/32-Bit General-Purpose Timer 5 Software Reset"]
pub type SYSCTL_SRTIMER_R5_W<'a> = crate::BitWriter<'a, u32, SRTIMER_SPEC, bool, 5>;
#[doc = "Field `SYSCTL_SRTIMER_R6` reader - 16/32-Bit General-Purpose Timer 6 Software Reset"]
pub type SYSCTL_SRTIMER_R6_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SRTIMER_R6` writer - 16/32-Bit General-Purpose Timer 6 Software Reset"]
pub type SYSCTL_SRTIMER_R6_W<'a> = crate::BitWriter<'a, u32, SRTIMER_SPEC, bool, 6>;
#[doc = "Field `SYSCTL_SRTIMER_R7` reader - 16/32-Bit General-Purpose Timer 7 Software Reset"]
pub type SYSCTL_SRTIMER_R7_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SRTIMER_R7` writer - 16/32-Bit General-Purpose Timer 7 Software Reset"]
pub type SYSCTL_SRTIMER_R7_W<'a> = crate::BitWriter<'a, u32, SRTIMER_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - 16/32-Bit General-Purpose Timer 0 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srtimer_r0(&self) -> SYSCTL_SRTIMER_R0_R {
        SYSCTL_SRTIMER_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 16/32-Bit General-Purpose Timer 1 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srtimer_r1(&self) -> SYSCTL_SRTIMER_R1_R {
        SYSCTL_SRTIMER_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 16/32-Bit General-Purpose Timer 2 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srtimer_r2(&self) -> SYSCTL_SRTIMER_R2_R {
        SYSCTL_SRTIMER_R2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 16/32-Bit General-Purpose Timer 3 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srtimer_r3(&self) -> SYSCTL_SRTIMER_R3_R {
        SYSCTL_SRTIMER_R3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 16/32-Bit General-Purpose Timer 4 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srtimer_r4(&self) -> SYSCTL_SRTIMER_R4_R {
        SYSCTL_SRTIMER_R4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 16/32-Bit General-Purpose Timer 5 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srtimer_r5(&self) -> SYSCTL_SRTIMER_R5_R {
        SYSCTL_SRTIMER_R5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 16/32-Bit General-Purpose Timer 6 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srtimer_r6(&self) -> SYSCTL_SRTIMER_R6_R {
        SYSCTL_SRTIMER_R6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 16/32-Bit General-Purpose Timer 7 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srtimer_r7(&self) -> SYSCTL_SRTIMER_R7_R {
        SYSCTL_SRTIMER_R7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 16/32-Bit General-Purpose Timer 0 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srtimer_r0(&mut self) -> SYSCTL_SRTIMER_R0_W {
        SYSCTL_SRTIMER_R0_W::new(self)
    }
    #[doc = "Bit 1 - 16/32-Bit General-Purpose Timer 1 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srtimer_r1(&mut self) -> SYSCTL_SRTIMER_R1_W {
        SYSCTL_SRTIMER_R1_W::new(self)
    }
    #[doc = "Bit 2 - 16/32-Bit General-Purpose Timer 2 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srtimer_r2(&mut self) -> SYSCTL_SRTIMER_R2_W {
        SYSCTL_SRTIMER_R2_W::new(self)
    }
    #[doc = "Bit 3 - 16/32-Bit General-Purpose Timer 3 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srtimer_r3(&mut self) -> SYSCTL_SRTIMER_R3_W {
        SYSCTL_SRTIMER_R3_W::new(self)
    }
    #[doc = "Bit 4 - 16/32-Bit General-Purpose Timer 4 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srtimer_r4(&mut self) -> SYSCTL_SRTIMER_R4_W {
        SYSCTL_SRTIMER_R4_W::new(self)
    }
    #[doc = "Bit 5 - 16/32-Bit General-Purpose Timer 5 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srtimer_r5(&mut self) -> SYSCTL_SRTIMER_R5_W {
        SYSCTL_SRTIMER_R5_W::new(self)
    }
    #[doc = "Bit 6 - 16/32-Bit General-Purpose Timer 6 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srtimer_r6(&mut self) -> SYSCTL_SRTIMER_R6_W {
        SYSCTL_SRTIMER_R6_W::new(self)
    }
    #[doc = "Bit 7 - 16/32-Bit General-Purpose Timer 7 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srtimer_r7(&mut self) -> SYSCTL_SRTIMER_R7_W {
        SYSCTL_SRTIMER_R7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "16/32-Bit General-Purpose Timer Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srtimer](index.html) module"]
pub struct SRTIMER_SPEC;
impl crate::RegisterSpec for SRTIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srtimer::R](R) reader structure"]
impl crate::Readable for SRTIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srtimer::W](W) writer structure"]
impl crate::Writable for SRTIMER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRTIMER to value 0"]
impl crate::Resettable for SRTIMER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
