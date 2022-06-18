#[doc = "Register `BAUD2` reader"]
pub struct R(crate::R<BAUD2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAUD2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAUD2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAUD2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAUD2` writer"]
pub struct W(crate::W<BAUD2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAUD2_SPEC>;
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
impl From<crate::W<BAUD2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAUD2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPI_BAUD2_COUNT0` reader - CS2n Baud Rate Counter 0"]
pub type EPI_BAUD2_COUNT0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EPI_BAUD2_COUNT0` writer - CS2n Baud Rate Counter 0"]
pub type EPI_BAUD2_COUNT0_W<'a> = crate::FieldWriter<'a, u32, BAUD2_SPEC, u16, u16, 16, 0>;
#[doc = "Field `EPI_BAUD2_COUNT1` reader - CS3n Baud Rate Counter 1"]
pub type EPI_BAUD2_COUNT1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EPI_BAUD2_COUNT1` writer - CS3n Baud Rate Counter 1"]
pub type EPI_BAUD2_COUNT1_W<'a> = crate::FieldWriter<'a, u32, BAUD2_SPEC, u16, u16, 16, 16>;
impl R {
    #[doc = "Bits 0:15 - CS2n Baud Rate Counter 0"]
    #[inline(always)]
    pub fn epi_baud2_count0(&self) -> EPI_BAUD2_COUNT0_R {
        EPI_BAUD2_COUNT0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CS3n Baud Rate Counter 1"]
    #[inline(always)]
    pub fn epi_baud2_count1(&self) -> EPI_BAUD2_COUNT1_R {
        EPI_BAUD2_COUNT1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CS2n Baud Rate Counter 0"]
    #[inline(always)]
    pub fn epi_baud2_count0(&mut self) -> EPI_BAUD2_COUNT0_W {
        EPI_BAUD2_COUNT0_W::new(self)
    }
    #[doc = "Bits 16:31 - CS3n Baud Rate Counter 1"]
    #[inline(always)]
    pub fn epi_baud2_count1(&mut self) -> EPI_BAUD2_COUNT1_W {
        EPI_BAUD2_COUNT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Main Baud Rate\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baud2](index.html) module"]
pub struct BAUD2_SPEC;
impl crate::RegisterSpec for BAUD2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [baud2::R](R) reader structure"]
impl crate::Readable for BAUD2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [baud2::W](W) writer structure"]
impl crate::Writable for BAUD2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BAUD2 to value 0"]
impl crate::Resettable for BAUD2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
