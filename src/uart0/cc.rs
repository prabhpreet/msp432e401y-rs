#[doc = "Register `CC` reader"]
pub struct R(crate::R<CC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CC` writer"]
pub struct W(crate::W<CC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CC_SPEC>;
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
impl From<crate::W<CC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "UART Baud Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UART_CC_CS_A {
    #[doc = "0: System clock (based on clock source and divisor factor)"]
    UART_CC_CS_SYSCLK = 0,
    #[doc = "5: PIOSC"]
    UART_CC_CS_PIOSC = 5,
}
impl From<UART_CC_CS_A> for u8 {
    #[inline(always)]
    fn from(variant: UART_CC_CS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UART_CC_CS` reader - UART Baud Clock Source"]
pub type UART_CC_CS_R = crate::FieldReader<u8, UART_CC_CS_A>;
impl UART_CC_CS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UART_CC_CS_A> {
        match self.bits {
            0 => Some(UART_CC_CS_A::UART_CC_CS_SYSCLK),
            5 => Some(UART_CC_CS_A::UART_CC_CS_PIOSC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UART_CC_CS_SYSCLK`"]
    #[inline(always)]
    pub fn is_uart_cc_cs_sysclk(&self) -> bool {
        *self == UART_CC_CS_A::UART_CC_CS_SYSCLK
    }
    #[doc = "Checks if the value of the field is `UART_CC_CS_PIOSC`"]
    #[inline(always)]
    pub fn is_uart_cc_cs_piosc(&self) -> bool {
        *self == UART_CC_CS_A::UART_CC_CS_PIOSC
    }
}
#[doc = "Field `UART_CC_CS` writer - UART Baud Clock Source"]
pub type UART_CC_CS_W<'a> = crate::FieldWriter<'a, u32, CC_SPEC, u8, UART_CC_CS_A, 4, 0>;
impl<'a> UART_CC_CS_W<'a> {
    #[doc = "System clock (based on clock source and divisor factor)"]
    #[inline(always)]
    pub fn uart_cc_cs_sysclk(self) -> &'a mut W {
        self.variant(UART_CC_CS_A::UART_CC_CS_SYSCLK)
    }
    #[doc = "PIOSC"]
    #[inline(always)]
    pub fn uart_cc_cs_piosc(self) -> &'a mut W {
        self.variant(UART_CC_CS_A::UART_CC_CS_PIOSC)
    }
}
impl R {
    #[doc = "Bits 0:3 - UART Baud Clock Source"]
    #[inline(always)]
    pub fn uart_cc_cs(&self) -> UART_CC_CS_R {
        UART_CC_CS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - UART Baud Clock Source"]
    #[inline(always)]
    pub fn uart_cc_cs(&mut self) -> UART_CC_CS_W {
        UART_CC_CS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Clock Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc](index.html) module"]
pub struct CC_SPEC;
impl crate::RegisterSpec for CC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc::R](R) reader structure"]
impl crate::Readable for CC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cc::W](W) writer structure"]
impl crate::Writable for CC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CC to value 0"]
impl crate::Resettable for CC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
