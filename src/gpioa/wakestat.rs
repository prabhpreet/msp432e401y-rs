#[doc = "Register `WAKESTAT` reader"]
pub struct R(crate::R<WAKESTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKESTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKESTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKESTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKESTAT` writer"]
pub struct W(crate::W<WAKESTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKESTAT_SPEC>;
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
impl From<crate::W<WAKESTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKESTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_WAKESTAT_STAT4` reader - P\\[4\\]
Wake Status"]
pub type GPIO_WAKESTAT_STAT4_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_WAKESTAT_STAT4` writer - P\\[4\\]
Wake Status"]
pub type GPIO_WAKESTAT_STAT4_W<'a> = crate::BitWriter<'a, u32, WAKESTAT_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 4 - P\\[4\\]
Wake Status"]
    #[inline(always)]
    pub fn gpio_wakestat_stat4(&self) -> GPIO_WAKESTAT_STAT4_R {
        GPIO_WAKESTAT_STAT4_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - P\\[4\\]
Wake Status"]
    #[inline(always)]
    pub fn gpio_wakestat_stat4(&mut self) -> GPIO_WAKESTAT_STAT4_W {
        GPIO_WAKESTAT_STAT4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Wake Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakestat](index.html) module"]
pub struct WAKESTAT_SPEC;
impl crate::RegisterSpec for WAKESTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wakestat::R](R) reader structure"]
impl crate::Readable for WAKESTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wakestat::W](W) writer structure"]
impl crate::Writable for WAKESTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAKESTAT to value 0"]
impl crate::Resettable for WAKESTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
