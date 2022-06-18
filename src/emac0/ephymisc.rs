#[doc = "Register `EPHYMISC` reader"]
pub struct R(crate::R<EPHYMISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPHYMISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPHYMISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPHYMISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPHYMISC` writer"]
pub struct W(crate::W<EPHYMISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPHYMISC_SPEC>;
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
impl From<crate::W<EPHYMISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPHYMISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_EPHYMISC_INT` reader - Ethernet PHY Status and Clear register"]
pub type EMAC_EPHYMISC_INT_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_EPHYMISC_INT` writer - Ethernet PHY Status and Clear register"]
pub type EMAC_EPHYMISC_INT_W<'a> = crate::BitWriter<'a, u32, EPHYMISC_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Ethernet PHY Status and Clear register"]
    #[inline(always)]
    pub fn emac_ephymisc_int(&self) -> EMAC_EPHYMISC_INT_R {
        EMAC_EPHYMISC_INT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ethernet PHY Status and Clear register"]
    #[inline(always)]
    pub fn emac_ephymisc_int(&mut self) -> EMAC_EPHYMISC_INT_W {
        EMAC_EPHYMISC_INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet PHY Masked Interrupt Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ephymisc](index.html) module"]
pub struct EPHYMISC_SPEC;
impl crate::RegisterSpec for EPHYMISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ephymisc::R](R) reader structure"]
impl crate::Readable for EPHYMISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ephymisc::W](W) writer structure"]
impl crate::Writable for EPHYMISC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPHYMISC to value 0"]
impl crate::Resettable for EPHYMISC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
