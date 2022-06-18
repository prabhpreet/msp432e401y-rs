#[doc = "Register `RIS` reader"]
pub struct R(crate::R<RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RIS` writer"]
pub struct W(crate::W<RIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RIS_SPEC>;
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
impl From<crate::W<RIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_RIS_GPIO` reader - GPIO Interrupt Raw Status"]
pub type GPIO_RIS_GPIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIO_RIS_GPIO` writer - GPIO Interrupt Raw Status"]
pub type GPIO_RIS_GPIO_W<'a> = crate::FieldWriter<'a, u32, RIS_SPEC, u8, u8, 8, 0>;
#[doc = "Field `GPIO_RIS_DMARIS` reader - GPIO uDMA Done Interrupt Raw Status"]
pub type GPIO_RIS_DMARIS_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_RIS_DMARIS` writer - GPIO uDMA Done Interrupt Raw Status"]
pub type GPIO_RIS_DMARIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO Interrupt Raw Status"]
    #[inline(always)]
    pub fn gpio_ris_gpio(&self) -> GPIO_RIS_GPIO_R {
        GPIO_RIS_GPIO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - GPIO uDMA Done Interrupt Raw Status"]
    #[inline(always)]
    pub fn gpio_ris_dmaris(&self) -> GPIO_RIS_DMARIS_R {
        GPIO_RIS_DMARIS_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO Interrupt Raw Status"]
    #[inline(always)]
    pub fn gpio_ris_gpio(&mut self) -> GPIO_RIS_GPIO_W {
        GPIO_RIS_GPIO_W::new(self)
    }
    #[doc = "Bit 8 - GPIO uDMA Done Interrupt Raw Status"]
    #[inline(always)]
    pub fn gpio_ris_dmaris(&mut self) -> GPIO_RIS_DMARIS_W {
        GPIO_RIS_DMARIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ris::R](R) reader structure"]
impl crate::Readable for RIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ris::W](W) writer structure"]
impl crate::Writable for RIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
