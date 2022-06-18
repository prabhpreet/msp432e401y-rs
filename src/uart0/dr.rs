#[doc = "Register `DR` reader"]
pub struct R(crate::R<DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DR` writer"]
pub struct W(crate::W<DR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR_SPEC>;
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
impl From<crate::W<DR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_DR_DATA` reader - Data Transmitted or Received"]
pub type UART_DR_DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UART_DR_DATA` writer - Data Transmitted or Received"]
pub type UART_DR_DATA_W<'a> = crate::FieldWriter<'a, u32, DR_SPEC, u8, u8, 8, 0>;
#[doc = "Field `UART_DR_FE` reader - UART Framing Error"]
pub type UART_DR_FE_R = crate::BitReader<bool>;
#[doc = "Field `UART_DR_FE` writer - UART Framing Error"]
pub type UART_DR_FE_W<'a> = crate::BitWriter<'a, u32, DR_SPEC, bool, 8>;
#[doc = "Field `UART_DR_PE` reader - UART Parity Error"]
pub type UART_DR_PE_R = crate::BitReader<bool>;
#[doc = "Field `UART_DR_PE` writer - UART Parity Error"]
pub type UART_DR_PE_W<'a> = crate::BitWriter<'a, u32, DR_SPEC, bool, 9>;
#[doc = "Field `UART_DR_BE` reader - UART Break Error"]
pub type UART_DR_BE_R = crate::BitReader<bool>;
#[doc = "Field `UART_DR_BE` writer - UART Break Error"]
pub type UART_DR_BE_W<'a> = crate::BitWriter<'a, u32, DR_SPEC, bool, 10>;
#[doc = "Field `UART_DR_OE` reader - UART Overrun Error"]
pub type UART_DR_OE_R = crate::BitReader<bool>;
#[doc = "Field `UART_DR_OE` writer - UART Overrun Error"]
pub type UART_DR_OE_W<'a> = crate::BitWriter<'a, u32, DR_SPEC, bool, 11>;
impl R {
    #[doc = "Bits 0:7 - Data Transmitted or Received"]
    #[inline(always)]
    pub fn uart_dr_data(&self) -> UART_DR_DATA_R {
        UART_DR_DATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - UART Framing Error"]
    #[inline(always)]
    pub fn uart_dr_fe(&self) -> UART_DR_FE_R {
        UART_DR_FE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART Parity Error"]
    #[inline(always)]
    pub fn uart_dr_pe(&self) -> UART_DR_PE_R {
        UART_DR_PE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART Break Error"]
    #[inline(always)]
    pub fn uart_dr_be(&self) -> UART_DR_BE_R {
        UART_DR_BE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - UART Overrun Error"]
    #[inline(always)]
    pub fn uart_dr_oe(&self) -> UART_DR_OE_R {
        UART_DR_OE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Transmitted or Received"]
    #[inline(always)]
    pub fn uart_dr_data(&mut self) -> UART_DR_DATA_W {
        UART_DR_DATA_W::new(self)
    }
    #[doc = "Bit 8 - UART Framing Error"]
    #[inline(always)]
    pub fn uart_dr_fe(&mut self) -> UART_DR_FE_W {
        UART_DR_FE_W::new(self)
    }
    #[doc = "Bit 9 - UART Parity Error"]
    #[inline(always)]
    pub fn uart_dr_pe(&mut self) -> UART_DR_PE_W {
        UART_DR_PE_W::new(self)
    }
    #[doc = "Bit 10 - UART Break Error"]
    #[inline(always)]
    pub fn uart_dr_be(&mut self) -> UART_DR_BE_W {
        UART_DR_BE_W::new(self)
    }
    #[doc = "Bit 11 - UART Overrun Error"]
    #[inline(always)]
    pub fn uart_dr_oe(&mut self) -> UART_DR_OE_W {
        UART_DR_OE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](index.html) module"]
pub struct DR_SPEC;
impl crate::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dr::R](R) reader structure"]
impl crate::Readable for DR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dr::W](W) writer structure"]
impl crate::Writable for DR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
