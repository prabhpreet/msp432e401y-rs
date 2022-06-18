#[doc = "Register `WFIFOCNT` reader"]
pub struct R(crate::R<WFIFOCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WFIFOCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WFIFOCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WFIFOCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WFIFOCNT` writer"]
pub struct W(crate::W<WFIFOCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WFIFOCNT_SPEC>;
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
impl From<crate::W<WFIFOCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WFIFOCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPI_WFIFOCNT_WTAV` reader - Available Write Transactions"]
pub type EPI_WFIFOCNT_WTAV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPI_WFIFOCNT_WTAV` writer - Available Write Transactions"]
pub type EPI_WFIFOCNT_WTAV_W<'a> = crate::FieldWriter<'a, u32, WFIFOCNT_SPEC, u8, u8, 3, 0>;
impl R {
    #[doc = "Bits 0:2 - Available Write Transactions"]
    #[inline(always)]
    pub fn epi_wfifocnt_wtav(&self) -> EPI_WFIFOCNT_WTAV_R {
        EPI_WFIFOCNT_WTAV_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Available Write Transactions"]
    #[inline(always)]
    pub fn epi_wfifocnt_wtav(&mut self) -> EPI_WFIFOCNT_WTAV_W {
        EPI_WFIFOCNT_WTAV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Write FIFO Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wfifocnt](index.html) module"]
pub struct WFIFOCNT_SPEC;
impl crate::RegisterSpec for WFIFOCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wfifocnt::R](R) reader structure"]
impl crate::Readable for WFIFOCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wfifocnt::W](W) writer structure"]
impl crate::Writable for WFIFOCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WFIFOCNT to value 0"]
impl crate::Resettable for WFIFOCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
