#[doc = "Register `EPHYRIS` reader"]
pub struct R(crate::R<EPHYRIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPHYRIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPHYRIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPHYRIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPHYRIS` writer"]
pub struct W(crate::W<EPHYRIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPHYRIS_SPEC>;
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
impl From<crate::W<EPHYRIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPHYRIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_EPHYRIS_INT` reader - Ethernet PHY Raw Interrupt Status"]
pub type EMAC_EPHYRIS_INT_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_EPHYRIS_INT` writer - Ethernet PHY Raw Interrupt Status"]
pub type EMAC_EPHYRIS_INT_W<'a> = crate::BitWriter<'a, u32, EPHYRIS_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Ethernet PHY Raw Interrupt Status"]
    #[inline(always)]
    pub fn emac_ephyris_int(&self) -> EMAC_EPHYRIS_INT_R {
        EMAC_EPHYRIS_INT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ethernet PHY Raw Interrupt Status"]
    #[inline(always)]
    pub fn emac_ephyris_int(&mut self) -> EMAC_EPHYRIS_INT_W {
        EMAC_EPHYRIS_INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet PHY Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ephyris](index.html) module"]
pub struct EPHYRIS_SPEC;
impl crate::RegisterSpec for EPHYRIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ephyris::R](R) reader structure"]
impl crate::Readable for EPHYRIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ephyris::W](W) writer structure"]
impl crate::Writable for EPHYRIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPHYRIS to value 0"]
impl crate::Resettable for EPHYRIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
