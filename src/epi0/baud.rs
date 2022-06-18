#[doc = "Register `BAUD` reader"]
pub struct R(crate::R<BAUD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAUD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAUD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAUD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAUD` writer"]
pub struct W(crate::W<BAUD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAUD_SPEC>;
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
impl From<crate::W<BAUD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAUD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPI_BAUD_COUNT0` reader - Baud Rate Counter 0"]
pub type EPI_BAUD_COUNT0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EPI_BAUD_COUNT0` writer - Baud Rate Counter 0"]
pub type EPI_BAUD_COUNT0_W<'a> = crate::FieldWriter<'a, u32, BAUD_SPEC, u16, u16, 16, 0>;
#[doc = "Field `EPI_BAUD_COUNT1` reader - Baud Rate Counter 1"]
pub type EPI_BAUD_COUNT1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EPI_BAUD_COUNT1` writer - Baud Rate Counter 1"]
pub type EPI_BAUD_COUNT1_W<'a> = crate::FieldWriter<'a, u32, BAUD_SPEC, u16, u16, 16, 16>;
impl R {
    #[doc = "Bits 0:15 - Baud Rate Counter 0"]
    #[inline(always)]
    pub fn epi_baud_count0(&self) -> EPI_BAUD_COUNT0_R {
        EPI_BAUD_COUNT0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Baud Rate Counter 1"]
    #[inline(always)]
    pub fn epi_baud_count1(&self) -> EPI_BAUD_COUNT1_R {
        EPI_BAUD_COUNT1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Baud Rate Counter 0"]
    #[inline(always)]
    pub fn epi_baud_count0(&mut self) -> EPI_BAUD_COUNT0_W {
        EPI_BAUD_COUNT0_W::new(self)
    }
    #[doc = "Bits 16:31 - Baud Rate Counter 1"]
    #[inline(always)]
    pub fn epi_baud_count1(&mut self) -> EPI_BAUD_COUNT1_W {
        EPI_BAUD_COUNT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Main Baud Rate\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baud](index.html) module"]
pub struct BAUD_SPEC;
impl crate::RegisterSpec for BAUD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [baud::R](R) reader structure"]
impl crate::Readable for BAUD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [baud::W](W) writer structure"]
impl crate::Writable for BAUD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BAUD to value 0"]
impl crate::Resettable for BAUD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
