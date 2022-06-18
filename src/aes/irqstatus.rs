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
#[doc = "Field `AES_IRQSTATUS_CONTEXT_IN` reader - Context In Interrupt Status"]
pub type AES_IRQSTATUS_CONTEXT_IN_R = crate::BitReader<bool>;
#[doc = "Field `AES_IRQSTATUS_CONTEXT_IN` writer - Context In Interrupt Status"]
pub type AES_IRQSTATUS_CONTEXT_IN_W<'a> = crate::BitWriter<'a, u32, IRQSTATUS_SPEC, bool, 0>;
#[doc = "Field `AES_IRQSTATUS_DATA_IN` reader - Data In Interrupt Status"]
pub type AES_IRQSTATUS_DATA_IN_R = crate::BitReader<bool>;
#[doc = "Field `AES_IRQSTATUS_DATA_IN` writer - Data In Interrupt Status"]
pub type AES_IRQSTATUS_DATA_IN_W<'a> = crate::BitWriter<'a, u32, IRQSTATUS_SPEC, bool, 1>;
#[doc = "Field `AES_IRQSTATUS_DATA_OUT` reader - Data Out Interrupt Status"]
pub type AES_IRQSTATUS_DATA_OUT_R = crate::BitReader<bool>;
#[doc = "Field `AES_IRQSTATUS_DATA_OUT` writer - Data Out Interrupt Status"]
pub type AES_IRQSTATUS_DATA_OUT_W<'a> = crate::BitWriter<'a, u32, IRQSTATUS_SPEC, bool, 2>;
#[doc = "Field `AES_IRQSTATUS_CONTEXT_OUT` reader - Context Output Interrupt Status"]
pub type AES_IRQSTATUS_CONTEXT_OUT_R = crate::BitReader<bool>;
#[doc = "Field `AES_IRQSTATUS_CONTEXT_OUT` writer - Context Output Interrupt Status"]
pub type AES_IRQSTATUS_CONTEXT_OUT_W<'a> = crate::BitWriter<'a, u32, IRQSTATUS_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - Context In Interrupt Status"]
    #[inline(always)]
    pub fn aes_irqstatus_context_in(&self) -> AES_IRQSTATUS_CONTEXT_IN_R {
        AES_IRQSTATUS_CONTEXT_IN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data In Interrupt Status"]
    #[inline(always)]
    pub fn aes_irqstatus_data_in(&self) -> AES_IRQSTATUS_DATA_IN_R {
        AES_IRQSTATUS_DATA_IN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Out Interrupt Status"]
    #[inline(always)]
    pub fn aes_irqstatus_data_out(&self) -> AES_IRQSTATUS_DATA_OUT_R {
        AES_IRQSTATUS_DATA_OUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Context Output Interrupt Status"]
    #[inline(always)]
    pub fn aes_irqstatus_context_out(&self) -> AES_IRQSTATUS_CONTEXT_OUT_R {
        AES_IRQSTATUS_CONTEXT_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Context In Interrupt Status"]
    #[inline(always)]
    pub fn aes_irqstatus_context_in(&mut self) -> AES_IRQSTATUS_CONTEXT_IN_W {
        AES_IRQSTATUS_CONTEXT_IN_W::new(self)
    }
    #[doc = "Bit 1 - Data In Interrupt Status"]
    #[inline(always)]
    pub fn aes_irqstatus_data_in(&mut self) -> AES_IRQSTATUS_DATA_IN_W {
        AES_IRQSTATUS_DATA_IN_W::new(self)
    }
    #[doc = "Bit 2 - Data Out Interrupt Status"]
    #[inline(always)]
    pub fn aes_irqstatus_data_out(&mut self) -> AES_IRQSTATUS_DATA_OUT_W {
        AES_IRQSTATUS_DATA_OUT_W::new(self)
    }
    #[doc = "Bit 3 - Context Output Interrupt Status"]
    #[inline(always)]
    pub fn aes_irqstatus_context_out(&mut self) -> AES_IRQSTATUS_CONTEXT_OUT_W {
        AES_IRQSTATUS_CONTEXT_OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqstatus](index.html) module"]
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
