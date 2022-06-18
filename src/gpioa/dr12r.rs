#[doc = "Register `DR12R` reader"]
pub struct R(crate::R<DR12R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR12R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR12R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR12R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DR12R` writer"]
pub struct W(crate::W<DR12R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR12R_SPEC>;
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
impl From<crate::W<DR12R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR12R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Output Pad 12-mA Drive Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO_DR12R_DRV12_A {
    #[doc = "1: The corresponding GPIO pin has 12-mA drive. This encoding is only valid if the GPIOPP EDE bit is set and the appropriate GPIOPC EDM bit field is programmed to 0x3"]
    GPIO_DR12R_DRV12_12MA = 1,
}
impl From<GPIO_DR12R_DRV12_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO_DR12R_DRV12_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO_DR12R_DRV12` reader - Output Pad 12-mA Drive Enable"]
pub type GPIO_DR12R_DRV12_R = crate::FieldReader<u8, GPIO_DR12R_DRV12_A>;
impl GPIO_DR12R_DRV12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_DR12R_DRV12_A> {
        match self.bits {
            1 => Some(GPIO_DR12R_DRV12_A::GPIO_DR12R_DRV12_12MA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_DR12R_DRV12_12MA`"]
    #[inline(always)]
    pub fn is_gpio_dr12r_drv12_12ma(&self) -> bool {
        *self == GPIO_DR12R_DRV12_A::GPIO_DR12R_DRV12_12MA
    }
}
#[doc = "Field `GPIO_DR12R_DRV12` writer - Output Pad 12-mA Drive Enable"]
pub type GPIO_DR12R_DRV12_W<'a> =
    crate::FieldWriter<'a, u32, DR12R_SPEC, u8, GPIO_DR12R_DRV12_A, 8, 0>;
impl<'a> GPIO_DR12R_DRV12_W<'a> {
    #[doc = "The corresponding GPIO pin has 12-mA drive. This encoding is only valid if the GPIOPP EDE bit is set and the appropriate GPIOPC EDM bit field is programmed to 0x3"]
    #[inline(always)]
    pub fn gpio_dr12r_drv12_12ma(self) -> &'a mut W {
        self.variant(GPIO_DR12R_DRV12_A::GPIO_DR12R_DRV12_12MA)
    }
}
impl R {
    #[doc = "Bits 0:7 - Output Pad 12-mA Drive Enable"]
    #[inline(always)]
    pub fn gpio_dr12r_drv12(&self) -> GPIO_DR12R_DRV12_R {
        GPIO_DR12R_DRV12_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Output Pad 12-mA Drive Enable"]
    #[inline(always)]
    pub fn gpio_dr12r_drv12(&mut self) -> GPIO_DR12R_DRV12_W {
        GPIO_DR12R_DRV12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO 12-mA Drive Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr12r](index.html) module"]
pub struct DR12R_SPEC;
impl crate::RegisterSpec for DR12R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dr12r::R](R) reader structure"]
impl crate::Readable for DR12R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dr12r::W](W) writer structure"]
impl crate::Writable for DR12R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DR12R to value 0"]
impl crate::Resettable for DR12R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
