#[doc = "Register `MIS` reader"]
pub struct R(crate::R<MIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIS` writer"]
pub struct W(crate::W<MIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIS_SPEC>;
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
impl From<crate::W<MIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_MIS_GPIO` reader - GPIO Masked Interrupt Status"]
pub type GPIO_MIS_GPIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIO_MIS_GPIO` writer - GPIO Masked Interrupt Status"]
pub type GPIO_MIS_GPIO_W<'a> = crate::FieldWriter<'a, u32, MIS_SPEC, u8, u8, 8, 0>;
#[doc = "Field `GPIO_MIS_DMAMIS` reader - GPIO uDMA Done Masked Interrupt Status"]
pub type GPIO_MIS_DMAMIS_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_MIS_DMAMIS` writer - GPIO uDMA Done Masked Interrupt Status"]
pub type GPIO_MIS_DMAMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO Masked Interrupt Status"]
    #[inline(always)]
    pub fn gpio_mis_gpio(&self) -> GPIO_MIS_GPIO_R {
        GPIO_MIS_GPIO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - GPIO uDMA Done Masked Interrupt Status"]
    #[inline(always)]
    pub fn gpio_mis_dmamis(&self) -> GPIO_MIS_DMAMIS_R {
        GPIO_MIS_DMAMIS_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO Masked Interrupt Status"]
    #[inline(always)]
    pub fn gpio_mis_gpio(&mut self) -> GPIO_MIS_GPIO_W {
        GPIO_MIS_GPIO_W::new(self)
    }
    #[doc = "Bit 8 - GPIO uDMA Done Masked Interrupt Status"]
    #[inline(always)]
    pub fn gpio_mis_dmamis(&mut self) -> GPIO_MIS_DMAMIS_W {
        GPIO_MIS_DMAMIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](index.html) module"]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mis::R](R) reader structure"]
impl crate::Readable for MIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mis::W](W) writer structure"]
impl crate::Writable for MIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
