#[doc = "Register `TPLOG7` reader"]
pub struct R(crate::R<TPLOG7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPLOG7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPLOG7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPLOG7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPLOG7` writer"]
pub struct W(crate::W<TPLOG7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPLOG7_SPEC>;
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
impl From<crate::W<TPLOG7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPLOG7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIB_TPLOG7_TRIG0` reader - Status of TMPR\\[0\\]
Trigger"]
pub type HIB_TPLOG7_TRIG0_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TPLOG7_TRIG0` writer - Status of TMPR\\[0\\]
Trigger"]
pub type HIB_TPLOG7_TRIG0_W<'a> = crate::BitWriter<'a, u32, TPLOG7_SPEC, bool, 0>;
#[doc = "Field `HIB_TPLOG7_TRIG1` reader - Status of TMPR\\[1\\]
Trigger"]
pub type HIB_TPLOG7_TRIG1_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TPLOG7_TRIG1` writer - Status of TMPR\\[1\\]
Trigger"]
pub type HIB_TPLOG7_TRIG1_W<'a> = crate::BitWriter<'a, u32, TPLOG7_SPEC, bool, 1>;
#[doc = "Field `HIB_TPLOG7_TRIG2` reader - Status of TMPR\\[2\\]
Trigger"]
pub type HIB_TPLOG7_TRIG2_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TPLOG7_TRIG2` writer - Status of TMPR\\[2\\]
Trigger"]
pub type HIB_TPLOG7_TRIG2_W<'a> = crate::BitWriter<'a, u32, TPLOG7_SPEC, bool, 2>;
#[doc = "Field `HIB_TPLOG7_TRIG3` reader - Status of TMPR\\[3\\]
Trigger"]
pub type HIB_TPLOG7_TRIG3_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TPLOG7_TRIG3` writer - Status of TMPR\\[3\\]
Trigger"]
pub type HIB_TPLOG7_TRIG3_W<'a> = crate::BitWriter<'a, u32, TPLOG7_SPEC, bool, 3>;
#[doc = "Field `HIB_TPLOG7_XOSC` reader - Status of external 32"]
pub type HIB_TPLOG7_XOSC_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TPLOG7_XOSC` writer - Status of external 32"]
pub type HIB_TPLOG7_XOSC_W<'a> = crate::BitWriter<'a, u32, TPLOG7_SPEC, bool, 16>;
impl R {
    #[doc = "Bit 0 - Status of TMPR\\[0\\]
Trigger"]
    #[inline(always)]
    pub fn hib_tplog7_trig0(&self) -> HIB_TPLOG7_TRIG0_R {
        HIB_TPLOG7_TRIG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status of TMPR\\[1\\]
Trigger"]
    #[inline(always)]
    pub fn hib_tplog7_trig1(&self) -> HIB_TPLOG7_TRIG1_R {
        HIB_TPLOG7_TRIG1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status of TMPR\\[2\\]
Trigger"]
    #[inline(always)]
    pub fn hib_tplog7_trig2(&self) -> HIB_TPLOG7_TRIG2_R {
        HIB_TPLOG7_TRIG2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status of TMPR\\[3\\]
Trigger"]
    #[inline(always)]
    pub fn hib_tplog7_trig3(&self) -> HIB_TPLOG7_TRIG3_R {
        HIB_TPLOG7_TRIG3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Status of external 32"]
    #[inline(always)]
    pub fn hib_tplog7_xosc(&self) -> HIB_TPLOG7_XOSC_R {
        HIB_TPLOG7_XOSC_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Status of TMPR\\[0\\]
Trigger"]
    #[inline(always)]
    pub fn hib_tplog7_trig0(&mut self) -> HIB_TPLOG7_TRIG0_W {
        HIB_TPLOG7_TRIG0_W::new(self)
    }
    #[doc = "Bit 1 - Status of TMPR\\[1\\]
Trigger"]
    #[inline(always)]
    pub fn hib_tplog7_trig1(&mut self) -> HIB_TPLOG7_TRIG1_W {
        HIB_TPLOG7_TRIG1_W::new(self)
    }
    #[doc = "Bit 2 - Status of TMPR\\[2\\]
Trigger"]
    #[inline(always)]
    pub fn hib_tplog7_trig2(&mut self) -> HIB_TPLOG7_TRIG2_W {
        HIB_TPLOG7_TRIG2_W::new(self)
    }
    #[doc = "Bit 3 - Status of TMPR\\[3\\]
Trigger"]
    #[inline(always)]
    pub fn hib_tplog7_trig3(&mut self) -> HIB_TPLOG7_TRIG3_W {
        HIB_TPLOG7_TRIG3_W::new(self)
    }
    #[doc = "Bit 16 - Status of external 32"]
    #[inline(always)]
    pub fn hib_tplog7_xosc(&mut self) -> HIB_TPLOG7_XOSC_W {
        HIB_TPLOG7_XOSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HIB Tamper Log 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tplog7](index.html) module"]
pub struct TPLOG7_SPEC;
impl crate::RegisterSpec for TPLOG7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tplog7::R](R) reader structure"]
impl crate::Readable for TPLOG7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tplog7::W](W) writer structure"]
impl crate::Writable for TPLOG7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPLOG7 to value 0"]
impl crate::Resettable for TPLOG7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
