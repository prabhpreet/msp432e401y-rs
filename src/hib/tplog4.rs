#[doc = "Register `TPLOG4` reader"]
pub struct R(crate::R<TPLOG4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPLOG4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPLOG4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPLOG4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPLOG4` writer"]
pub struct W(crate::W<TPLOG4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPLOG4_SPEC>;
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
impl From<crate::W<TPLOG4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPLOG4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIB_TPLOG4_TIME` reader - Tamper Log Calendar Information"]
pub type HIB_TPLOG4_TIME_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HIB_TPLOG4_TIME` writer - Tamper Log Calendar Information"]
pub type HIB_TPLOG4_TIME_W<'a> = crate::FieldWriter<'a, u32, TPLOG4_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Tamper Log Calendar Information"]
    #[inline(always)]
    pub fn hib_tplog4_time(&self) -> HIB_TPLOG4_TIME_R {
        HIB_TPLOG4_TIME_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Tamper Log Calendar Information"]
    #[inline(always)]
    pub fn hib_tplog4_time(&mut self) -> HIB_TPLOG4_TIME_W {
        HIB_TPLOG4_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HIB Tamper Log 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tplog4](index.html) module"]
pub struct TPLOG4_SPEC;
impl crate::RegisterSpec for TPLOG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tplog4::R](R) reader structure"]
impl crate::Readable for TPLOG4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tplog4::W](W) writer structure"]
impl crate::Writable for TPLOG4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPLOG4 to value 0"]
impl crate::Resettable for TPLOG4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
