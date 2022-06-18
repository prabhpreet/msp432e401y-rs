#[doc = "Register `PP` reader"]
pub struct R(crate::R<PP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PP` writer"]
pub struct W(crate::W<PP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PP_SPEC>;
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
impl From<crate::W<PP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_PP_SC` reader - Smart Card Support"]
pub type UART_PP_SC_R = crate::BitReader<bool>;
#[doc = "Field `UART_PP_SC` writer - Smart Card Support"]
pub type UART_PP_SC_W<'a> = crate::BitWriter<'a, u32, PP_SPEC, bool, 0>;
#[doc = "Field `UART_PP_NB` reader - 9-Bit Support"]
pub type UART_PP_NB_R = crate::BitReader<bool>;
#[doc = "Field `UART_PP_NB` writer - 9-Bit Support"]
pub type UART_PP_NB_W<'a> = crate::BitWriter<'a, u32, PP_SPEC, bool, 1>;
#[doc = "Field `UART_PP_MS` reader - Modem Support"]
pub type UART_PP_MS_R = crate::BitReader<bool>;
#[doc = "Field `UART_PP_MS` writer - Modem Support"]
pub type UART_PP_MS_W<'a> = crate::BitWriter<'a, u32, PP_SPEC, bool, 2>;
#[doc = "Field `UART_PP_MSE` reader - Modem Support Extended"]
pub type UART_PP_MSE_R = crate::BitReader<bool>;
#[doc = "Field `UART_PP_MSE` writer - Modem Support Extended"]
pub type UART_PP_MSE_W<'a> = crate::BitWriter<'a, u32, PP_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - Smart Card Support"]
    #[inline(always)]
    pub fn uart_pp_sc(&self) -> UART_PP_SC_R {
        UART_PP_SC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 9-Bit Support"]
    #[inline(always)]
    pub fn uart_pp_nb(&self) -> UART_PP_NB_R {
        UART_PP_NB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Modem Support"]
    #[inline(always)]
    pub fn uart_pp_ms(&self) -> UART_PP_MS_R {
        UART_PP_MS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Modem Support Extended"]
    #[inline(always)]
    pub fn uart_pp_mse(&self) -> UART_PP_MSE_R {
        UART_PP_MSE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Smart Card Support"]
    #[inline(always)]
    pub fn uart_pp_sc(&mut self) -> UART_PP_SC_W {
        UART_PP_SC_W::new(self)
    }
    #[doc = "Bit 1 - 9-Bit Support"]
    #[inline(always)]
    pub fn uart_pp_nb(&mut self) -> UART_PP_NB_W {
        UART_PP_NB_W::new(self)
    }
    #[doc = "Bit 2 - Modem Support"]
    #[inline(always)]
    pub fn uart_pp_ms(&mut self) -> UART_PP_MS_W {
        UART_PP_MS_W::new(self)
    }
    #[doc = "Bit 3 - Modem Support Extended"]
    #[inline(always)]
    pub fn uart_pp_mse(&mut self) -> UART_PP_MSE_W {
        UART_PP_MSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Peripheral Properties\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pp](index.html) module"]
pub struct PP_SPEC;
impl crate::RegisterSpec for PP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pp::R](R) reader structure"]
impl crate::Readable for PP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pp::W](W) writer structure"]
impl crate::Writable for PP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PP to value 0"]
impl crate::Resettable for PP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
