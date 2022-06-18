#[doc = "Register `IRQENABLE` reader"]
pub struct R(crate::R<IRQENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQENABLE` writer"]
pub struct W(crate::W<IRQENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQENABLE_SPEC>;
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
impl From<crate::W<IRQENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHAMD5_IRQENABLE_OUTPUT_READY` reader - Mask for output ready interrupt"]
pub type SHAMD5_IRQENABLE_OUTPUT_READY_R = crate::BitReader<bool>;
#[doc = "Field `SHAMD5_IRQENABLE_OUTPUT_READY` writer - Mask for output ready interrupt"]
pub type SHAMD5_IRQENABLE_OUTPUT_READY_W<'a> = crate::BitWriter<'a, u32, IRQENABLE_SPEC, bool, 0>;
#[doc = "Field `SHAMD5_IRQENABLE_INPUT_READY` reader - Mask for input ready interrupt"]
pub type SHAMD5_IRQENABLE_INPUT_READY_R = crate::BitReader<bool>;
#[doc = "Field `SHAMD5_IRQENABLE_INPUT_READY` writer - Mask for input ready interrupt"]
pub type SHAMD5_IRQENABLE_INPUT_READY_W<'a> = crate::BitWriter<'a, u32, IRQENABLE_SPEC, bool, 1>;
#[doc = "Field `SHAMD5_IRQENABLE_CONTEXT_READY` reader - Mask for context ready interrupt"]
pub type SHAMD5_IRQENABLE_CONTEXT_READY_R = crate::BitReader<bool>;
#[doc = "Field `SHAMD5_IRQENABLE_CONTEXT_READY` writer - Mask for context ready interrupt"]
pub type SHAMD5_IRQENABLE_CONTEXT_READY_W<'a> = crate::BitWriter<'a, u32, IRQENABLE_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - Mask for output ready interrupt"]
    #[inline(always)]
    pub fn shamd5_irqenable_output_ready(&self) -> SHAMD5_IRQENABLE_OUTPUT_READY_R {
        SHAMD5_IRQENABLE_OUTPUT_READY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask for input ready interrupt"]
    #[inline(always)]
    pub fn shamd5_irqenable_input_ready(&self) -> SHAMD5_IRQENABLE_INPUT_READY_R {
        SHAMD5_IRQENABLE_INPUT_READY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask for context ready interrupt"]
    #[inline(always)]
    pub fn shamd5_irqenable_context_ready(&self) -> SHAMD5_IRQENABLE_CONTEXT_READY_R {
        SHAMD5_IRQENABLE_CONTEXT_READY_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask for output ready interrupt"]
    #[inline(always)]
    pub fn shamd5_irqenable_output_ready(&mut self) -> SHAMD5_IRQENABLE_OUTPUT_READY_W {
        SHAMD5_IRQENABLE_OUTPUT_READY_W::new(self)
    }
    #[doc = "Bit 1 - Mask for input ready interrupt"]
    #[inline(always)]
    pub fn shamd5_irqenable_input_ready(&mut self) -> SHAMD5_IRQENABLE_INPUT_READY_W {
        SHAMD5_IRQENABLE_INPUT_READY_W::new(self)
    }
    #[doc = "Bit 3 - Mask for context ready interrupt"]
    #[inline(always)]
    pub fn shamd5_irqenable_context_ready(&mut self) -> SHAMD5_IRQENABLE_CONTEXT_READY_W {
        SHAMD5_IRQENABLE_CONTEXT_READY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SHA Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqenable](index.html) module"]
pub struct IRQENABLE_SPEC;
impl crate::RegisterSpec for IRQENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irqenable::R](R) reader structure"]
impl crate::Readable for IRQENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irqenable::W](W) writer structure"]
impl crate::Writable for IRQENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRQENABLE to value 0"]
impl crate::Resettable for IRQENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
