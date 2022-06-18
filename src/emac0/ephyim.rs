#[doc = "Register `EPHYIM` reader"]
pub struct R(crate::R<EPHYIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPHYIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPHYIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPHYIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPHYIM` writer"]
pub struct W(crate::W<EPHYIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPHYIM_SPEC>;
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
impl From<crate::W<EPHYIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPHYIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_EPHYIM_INT` reader - Ethernet PHY Interrupt Mask"]
pub type EMAC_EPHYIM_INT_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_EPHYIM_INT` writer - Ethernet PHY Interrupt Mask"]
pub type EMAC_EPHYIM_INT_W<'a> = crate::BitWriter<'a, u32, EPHYIM_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Ethernet PHY Interrupt Mask"]
    #[inline(always)]
    pub fn emac_ephyim_int(&self) -> EMAC_EPHYIM_INT_R {
        EMAC_EPHYIM_INT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ethernet PHY Interrupt Mask"]
    #[inline(always)]
    pub fn emac_ephyim_int(&mut self) -> EMAC_EPHYIM_INT_W {
        EMAC_EPHYIM_INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet PHY Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ephyim](index.html) module"]
pub struct EPHYIM_SPEC;
impl crate::RegisterSpec for EPHYIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ephyim::R](R) reader structure"]
impl crate::Readable for EPHYIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ephyim::W](W) writer structure"]
impl crate::Writable for EPHYIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPHYIM to value 0"]
impl crate::Resettable for EPHYIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
