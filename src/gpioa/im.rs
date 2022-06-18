#[doc = "Register `IM` reader"]
pub struct R(crate::R<IM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IM` writer"]
pub struct W(crate::W<IM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IM_SPEC>;
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
impl From<crate::W<IM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_IM_GPIO` reader - GPIO Interrupt Mask Enable"]
pub type GPIO_IM_GPIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIO_IM_GPIO` writer - GPIO Interrupt Mask Enable"]
pub type GPIO_IM_GPIO_W<'a> = crate::FieldWriter<'a, u32, IM_SPEC, u8, u8, 8, 0>;
#[doc = "Field `GPIO_IM_DMAIME` reader - GPIO uDMA Done Interrupt Mask Enable"]
pub type GPIO_IM_DMAIME_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_IM_DMAIME` writer - GPIO uDMA Done Interrupt Mask Enable"]
pub type GPIO_IM_DMAIME_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO Interrupt Mask Enable"]
    #[inline(always)]
    pub fn gpio_im_gpio(&self) -> GPIO_IM_GPIO_R {
        GPIO_IM_GPIO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - GPIO uDMA Done Interrupt Mask Enable"]
    #[inline(always)]
    pub fn gpio_im_dmaime(&self) -> GPIO_IM_DMAIME_R {
        GPIO_IM_DMAIME_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO Interrupt Mask Enable"]
    #[inline(always)]
    pub fn gpio_im_gpio(&mut self) -> GPIO_IM_GPIO_W {
        GPIO_IM_GPIO_W::new(self)
    }
    #[doc = "Bit 8 - GPIO uDMA Done Interrupt Mask Enable"]
    #[inline(always)]
    pub fn gpio_im_dmaime(&mut self) -> GPIO_IM_DMAIME_W {
        GPIO_IM_DMAIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [im](index.html) module"]
pub struct IM_SPEC;
impl crate::RegisterSpec for IM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [im::R](R) reader structure"]
impl crate::Readable for IM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [im::W](W) writer structure"]
impl crate::Writable for IM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IM to value 0"]
impl crate::Resettable for IM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
