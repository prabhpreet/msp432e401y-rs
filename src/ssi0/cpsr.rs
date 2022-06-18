#[doc = "Register `CPSR` reader"]
pub struct R(crate::R<CPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPSR` writer"]
pub struct W(crate::W<CPSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPSR_SPEC>;
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
impl From<crate::W<CPSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSI_CPSR_CPSDVSR` reader - SSI Clock Prescale Divisor"]
pub type SSI_CPSR_CPSDVSR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SSI_CPSR_CPSDVSR` writer - SSI Clock Prescale Divisor"]
pub type SSI_CPSR_CPSDVSR_W<'a> = crate::FieldWriter<'a, u32, CPSR_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - SSI Clock Prescale Divisor"]
    #[inline(always)]
    pub fn ssi_cpsr_cpsdvsr(&self) -> SSI_CPSR_CPSDVSR_R {
        SSI_CPSR_CPSDVSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SSI Clock Prescale Divisor"]
    #[inline(always)]
    pub fn ssi_cpsr_cpsdvsr(&mut self) -> SSI_CPSR_CPSDVSR_W {
        SSI_CPSR_CPSDVSR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SSI Clock Prescale\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpsr](index.html) module"]
pub struct CPSR_SPEC;
impl crate::RegisterSpec for CPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpsr::R](R) reader structure"]
impl crate::Readable for CPSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpsr::W](W) writer structure"]
impl crate::Writable for CPSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPSR to value 0"]
impl crate::Resettable for CPSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
