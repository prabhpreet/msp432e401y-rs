#[doc = "Register `RADDR1` reader"]
pub struct R(crate::R<RADDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RADDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RADDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RADDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RADDR1` writer"]
pub struct W(crate::W<RADDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RADDR1_SPEC>;
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
impl From<crate::W<RADDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RADDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPI_RADDR1_ADDR` reader - Current Address"]
pub type EPI_RADDR1_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EPI_RADDR1_ADDR` writer - Current Address"]
pub type EPI_RADDR1_ADDR_W<'a> = crate::FieldWriter<'a, u32, RADDR1_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Current Address"]
    #[inline(always)]
    pub fn epi_raddr1_addr(&self) -> EPI_RADDR1_ADDR_R {
        EPI_RADDR1_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Current Address"]
    #[inline(always)]
    pub fn epi_raddr1_addr(&mut self) -> EPI_RADDR1_ADDR_W {
        EPI_RADDR1_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Read Address 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [raddr1](index.html) module"]
pub struct RADDR1_SPEC;
impl crate::RegisterSpec for RADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [raddr1::R](R) reader structure"]
impl crate::Readable for RADDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [raddr1::W](W) writer structure"]
impl crate::Writable for RADDR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RADDR1 to value 0"]
impl crate::Resettable for RADDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
