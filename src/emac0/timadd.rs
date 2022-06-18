#[doc = "Register `TIMADD` reader"]
pub struct R(crate::R<TIMADD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMADD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMADD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMADD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMADD` writer"]
pub struct W(crate::W<TIMADD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMADD_SPEC>;
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
impl From<crate::W<TIMADD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMADD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_TIMADD_TSAR` reader - Timestamp Addend Register"]
pub type EMAC_TIMADD_TSAR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMAC_TIMADD_TSAR` writer - Timestamp Addend Register"]
pub type EMAC_TIMADD_TSAR_W<'a> = crate::FieldWriter<'a, u32, TIMADD_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Addend Register"]
    #[inline(always)]
    pub fn emac_timadd_tsar(&self) -> EMAC_TIMADD_TSAR_R {
        EMAC_TIMADD_TSAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp Addend Register"]
    #[inline(always)]
    pub fn emac_timadd_tsar(&mut self) -> EMAC_TIMADD_TSAR_W {
        EMAC_TIMADD_TSAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Timestamp Addend\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timadd](index.html) module"]
pub struct TIMADD_SPEC;
impl crate::RegisterSpec for TIMADD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timadd::R](R) reader structure"]
impl crate::Readable for TIMADD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timadd::W](W) writer structure"]
impl crate::Writable for TIMADD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMADD to value 0"]
impl crate::Resettable for TIMADD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
