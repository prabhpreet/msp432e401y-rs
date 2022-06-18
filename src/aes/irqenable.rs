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
#[doc = "Field `AES_IRQENABLE_CONTEXT_IN` reader - Context In Interrupt Enable"]
pub type AES_IRQENABLE_CONTEXT_IN_R = crate::BitReader<bool>;
#[doc = "Field `AES_IRQENABLE_CONTEXT_IN` writer - Context In Interrupt Enable"]
pub type AES_IRQENABLE_CONTEXT_IN_W<'a> = crate::BitWriter<'a, u32, IRQENABLE_SPEC, bool, 0>;
#[doc = "Field `AES_IRQENABLE_DATA_IN` reader - Data In Interrupt Enable"]
pub type AES_IRQENABLE_DATA_IN_R = crate::BitReader<bool>;
#[doc = "Field `AES_IRQENABLE_DATA_IN` writer - Data In Interrupt Enable"]
pub type AES_IRQENABLE_DATA_IN_W<'a> = crate::BitWriter<'a, u32, IRQENABLE_SPEC, bool, 1>;
#[doc = "Field `AES_IRQENABLE_DATA_OUT` reader - Data Out Interrupt Enable"]
pub type AES_IRQENABLE_DATA_OUT_R = crate::BitReader<bool>;
#[doc = "Field `AES_IRQENABLE_DATA_OUT` writer - Data Out Interrupt Enable"]
pub type AES_IRQENABLE_DATA_OUT_W<'a> = crate::BitWriter<'a, u32, IRQENABLE_SPEC, bool, 2>;
#[doc = "Field `AES_IRQENABLE_CONTEXT_OUT` reader - Context Out Interrupt Enable"]
pub type AES_IRQENABLE_CONTEXT_OUT_R = crate::BitReader<bool>;
#[doc = "Field `AES_IRQENABLE_CONTEXT_OUT` writer - Context Out Interrupt Enable"]
pub type AES_IRQENABLE_CONTEXT_OUT_W<'a> = crate::BitWriter<'a, u32, IRQENABLE_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - Context In Interrupt Enable"]
    #[inline(always)]
    pub fn aes_irqenable_context_in(&self) -> AES_IRQENABLE_CONTEXT_IN_R {
        AES_IRQENABLE_CONTEXT_IN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data In Interrupt Enable"]
    #[inline(always)]
    pub fn aes_irqenable_data_in(&self) -> AES_IRQENABLE_DATA_IN_R {
        AES_IRQENABLE_DATA_IN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Out Interrupt Enable"]
    #[inline(always)]
    pub fn aes_irqenable_data_out(&self) -> AES_IRQENABLE_DATA_OUT_R {
        AES_IRQENABLE_DATA_OUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Context Out Interrupt Enable"]
    #[inline(always)]
    pub fn aes_irqenable_context_out(&self) -> AES_IRQENABLE_CONTEXT_OUT_R {
        AES_IRQENABLE_CONTEXT_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Context In Interrupt Enable"]
    #[inline(always)]
    pub fn aes_irqenable_context_in(&mut self) -> AES_IRQENABLE_CONTEXT_IN_W {
        AES_IRQENABLE_CONTEXT_IN_W::new(self)
    }
    #[doc = "Bit 1 - Data In Interrupt Enable"]
    #[inline(always)]
    pub fn aes_irqenable_data_in(&mut self) -> AES_IRQENABLE_DATA_IN_W {
        AES_IRQENABLE_DATA_IN_W::new(self)
    }
    #[doc = "Bit 2 - Data Out Interrupt Enable"]
    #[inline(always)]
    pub fn aes_irqenable_data_out(&mut self) -> AES_IRQENABLE_DATA_OUT_W {
        AES_IRQENABLE_DATA_OUT_W::new(self)
    }
    #[doc = "Bit 3 - Context Out Interrupt Enable"]
    #[inline(always)]
    pub fn aes_irqenable_context_out(&mut self) -> AES_IRQENABLE_CONTEXT_OUT_W {
        AES_IRQENABLE_CONTEXT_OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqenable](index.html) module"]
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
