#[doc = "Register `PPOWIRE` reader"]
pub struct R(crate::R<PPOWIRE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPOWIRE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPOWIRE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPOWIRE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPOWIRE` writer"]
pub struct W(crate::W<PPOWIRE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPOWIRE_SPEC>;
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
impl From<crate::W<PPOWIRE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPOWIRE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PPOWIRE_P0` reader - 1-Wire Module Present"]
pub type SYSCTL_PPOWIRE_P0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPOWIRE_P0` writer - 1-Wire Module Present"]
pub type SYSCTL_PPOWIRE_P0_W<'a> = crate::BitWriter<'a, u32, PPOWIRE_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - 1-Wire Module Present"]
    #[inline(always)]
    pub fn sysctl_ppowire_p0(&self) -> SYSCTL_PPOWIRE_P0_R {
        SYSCTL_PPOWIRE_P0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1-Wire Module Present"]
    #[inline(always)]
    pub fn sysctl_ppowire_p0(&mut self) -> SYSCTL_PPOWIRE_P0_W {
        SYSCTL_PPOWIRE_P0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1-Wire Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppowire](index.html) module"]
pub struct PPOWIRE_SPEC;
impl crate::RegisterSpec for PPOWIRE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppowire::R](R) reader structure"]
impl crate::Readable for PPOWIRE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppowire::W](W) writer structure"]
impl crate::Writable for PPOWIRE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PPOWIRE to value 0"]
impl crate::Resettable for PPOWIRE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
