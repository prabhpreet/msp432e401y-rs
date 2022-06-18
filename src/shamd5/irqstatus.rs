#[doc = "Register `IRQSTATUS` reader"]
pub struct R(crate::R<IRQSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQSTATUS` writer"]
pub struct W(crate::W<IRQSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQSTATUS_SPEC>;
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
impl From<crate::W<IRQSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHAMD5_IRQSTATUS_OUTPUT_READY` reader - Output Ready Status"]
pub type SHAMD5_IRQSTATUS_OUTPUT_READY_R = crate::BitReader<bool>;
#[doc = "Field `SHAMD5_IRQSTATUS_OUTPUT_READY` writer - Output Ready Status"]
pub type SHAMD5_IRQSTATUS_OUTPUT_READY_W<'a> = crate::BitWriter<'a, u32, IRQSTATUS_SPEC, bool, 0>;
#[doc = "Field `SHAMD5_IRQSTATUS_INPUT_READY` reader - Input Ready Status"]
pub type SHAMD5_IRQSTATUS_INPUT_READY_R = crate::BitReader<bool>;
#[doc = "Field `SHAMD5_IRQSTATUS_INPUT_READY` writer - Input Ready Status"]
pub type SHAMD5_IRQSTATUS_INPUT_READY_W<'a> = crate::BitWriter<'a, u32, IRQSTATUS_SPEC, bool, 1>;
#[doc = "Field `SHAMD5_IRQSTATUS_CONTEXT_READY` reader - Context Ready Status"]
pub type SHAMD5_IRQSTATUS_CONTEXT_READY_R = crate::BitReader<bool>;
#[doc = "Field `SHAMD5_IRQSTATUS_CONTEXT_READY` writer - Context Ready Status"]
pub type SHAMD5_IRQSTATUS_CONTEXT_READY_W<'a> = crate::BitWriter<'a, u32, IRQSTATUS_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - Output Ready Status"]
    #[inline(always)]
    pub fn shamd5_irqstatus_output_ready(&self) -> SHAMD5_IRQSTATUS_OUTPUT_READY_R {
        SHAMD5_IRQSTATUS_OUTPUT_READY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input Ready Status"]
    #[inline(always)]
    pub fn shamd5_irqstatus_input_ready(&self) -> SHAMD5_IRQSTATUS_INPUT_READY_R {
        SHAMD5_IRQSTATUS_INPUT_READY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Context Ready Status"]
    #[inline(always)]
    pub fn shamd5_irqstatus_context_ready(&self) -> SHAMD5_IRQSTATUS_CONTEXT_READY_R {
        SHAMD5_IRQSTATUS_CONTEXT_READY_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Ready Status"]
    #[inline(always)]
    pub fn shamd5_irqstatus_output_ready(&mut self) -> SHAMD5_IRQSTATUS_OUTPUT_READY_W {
        SHAMD5_IRQSTATUS_OUTPUT_READY_W::new(self)
    }
    #[doc = "Bit 1 - Input Ready Status"]
    #[inline(always)]
    pub fn shamd5_irqstatus_input_ready(&mut self) -> SHAMD5_IRQSTATUS_INPUT_READY_W {
        SHAMD5_IRQSTATUS_INPUT_READY_W::new(self)
    }
    #[doc = "Bit 3 - Context Ready Status"]
    #[inline(always)]
    pub fn shamd5_irqstatus_context_ready(&mut self) -> SHAMD5_IRQSTATUS_CONTEXT_READY_W {
        SHAMD5_IRQSTATUS_CONTEXT_READY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SHA Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqstatus](index.html) module"]
pub struct IRQSTATUS_SPEC;
impl crate::RegisterSpec for IRQSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irqstatus::R](R) reader structure"]
impl crate::Readable for IRQSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irqstatus::W](W) writer structure"]
impl crate::Writable for IRQSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRQSTATUS to value 0"]
impl crate::Resettable for IRQSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
