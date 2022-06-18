#[doc = "Register `IFLS` reader"]
pub struct R(crate::R<IFLS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFLS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFLS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFLS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFLS` writer"]
pub struct W(crate::W<IFLS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFLS_SPEC>;
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
impl From<crate::W<IFLS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFLS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "UART Transmit Interrupt FIFO Level Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UART_IFLS_TX_A {
    #[doc = "0: TX FIFO &lt;= 1/8 full"]
    UART_IFLS_TX1_8 = 0,
    #[doc = "1: TX FIFO &lt;= 1/4 full"]
    UART_IFLS_TX2_8 = 1,
    #[doc = "2: TX FIFO &lt;= 1/2 full (default)"]
    UART_IFLS_TX4_8 = 2,
    #[doc = "3: TX FIFO &lt;= 3/4 full"]
    UART_IFLS_TX6_8 = 3,
    #[doc = "4: TX FIFO &lt;= 7/8 full"]
    UART_IFLS_TX7_8 = 4,
}
impl From<UART_IFLS_TX_A> for u8 {
    #[inline(always)]
    fn from(variant: UART_IFLS_TX_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UART_IFLS_TX` reader - UART Transmit Interrupt FIFO Level Select"]
pub type UART_IFLS_TX_R = crate::FieldReader<u8, UART_IFLS_TX_A>;
impl UART_IFLS_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UART_IFLS_TX_A> {
        match self.bits {
            0 => Some(UART_IFLS_TX_A::UART_IFLS_TX1_8),
            1 => Some(UART_IFLS_TX_A::UART_IFLS_TX2_8),
            2 => Some(UART_IFLS_TX_A::UART_IFLS_TX4_8),
            3 => Some(UART_IFLS_TX_A::UART_IFLS_TX6_8),
            4 => Some(UART_IFLS_TX_A::UART_IFLS_TX7_8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_TX1_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_tx1_8(&self) -> bool {
        *self == UART_IFLS_TX_A::UART_IFLS_TX1_8
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_TX2_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_tx2_8(&self) -> bool {
        *self == UART_IFLS_TX_A::UART_IFLS_TX2_8
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_TX4_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_tx4_8(&self) -> bool {
        *self == UART_IFLS_TX_A::UART_IFLS_TX4_8
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_TX6_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_tx6_8(&self) -> bool {
        *self == UART_IFLS_TX_A::UART_IFLS_TX6_8
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_TX7_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_tx7_8(&self) -> bool {
        *self == UART_IFLS_TX_A::UART_IFLS_TX7_8
    }
}
#[doc = "Field `UART_IFLS_TX` writer - UART Transmit Interrupt FIFO Level Select"]
pub type UART_IFLS_TX_W<'a> = crate::FieldWriter<'a, u32, IFLS_SPEC, u8, UART_IFLS_TX_A, 3, 0>;
impl<'a> UART_IFLS_TX_W<'a> {
    #[doc = "TX FIFO &lt;= 1/8 full"]
    #[inline(always)]
    pub fn uart_ifls_tx1_8(self) -> &'a mut W {
        self.variant(UART_IFLS_TX_A::UART_IFLS_TX1_8)
    }
    #[doc = "TX FIFO &lt;= 1/4 full"]
    #[inline(always)]
    pub fn uart_ifls_tx2_8(self) -> &'a mut W {
        self.variant(UART_IFLS_TX_A::UART_IFLS_TX2_8)
    }
    #[doc = "TX FIFO &lt;= 1/2 full (default)"]
    #[inline(always)]
    pub fn uart_ifls_tx4_8(self) -> &'a mut W {
        self.variant(UART_IFLS_TX_A::UART_IFLS_TX4_8)
    }
    #[doc = "TX FIFO &lt;= 3/4 full"]
    #[inline(always)]
    pub fn uart_ifls_tx6_8(self) -> &'a mut W {
        self.variant(UART_IFLS_TX_A::UART_IFLS_TX6_8)
    }
    #[doc = "TX FIFO &lt;= 7/8 full"]
    #[inline(always)]
    pub fn uart_ifls_tx7_8(self) -> &'a mut W {
        self.variant(UART_IFLS_TX_A::UART_IFLS_TX7_8)
    }
}
#[doc = "UART Receive Interrupt FIFO Level Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UART_IFLS_RX_A {
    #[doc = "0: RX FIFO >= 1/8 full"]
    UART_IFLS_RX1_8 = 0,
    #[doc = "1: RX FIFO >= 1/4 full"]
    UART_IFLS_RX2_8 = 1,
    #[doc = "2: RX FIFO >= 1/2 full (default)"]
    UART_IFLS_RX4_8 = 2,
    #[doc = "3: RX FIFO >= 3/4 full"]
    UART_IFLS_RX6_8 = 3,
    #[doc = "4: RX FIFO >= 7/8 full"]
    UART_IFLS_RX7_8 = 4,
}
impl From<UART_IFLS_RX_A> for u8 {
    #[inline(always)]
    fn from(variant: UART_IFLS_RX_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UART_IFLS_RX` reader - UART Receive Interrupt FIFO Level Select"]
pub type UART_IFLS_RX_R = crate::FieldReader<u8, UART_IFLS_RX_A>;
impl UART_IFLS_RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UART_IFLS_RX_A> {
        match self.bits {
            0 => Some(UART_IFLS_RX_A::UART_IFLS_RX1_8),
            1 => Some(UART_IFLS_RX_A::UART_IFLS_RX2_8),
            2 => Some(UART_IFLS_RX_A::UART_IFLS_RX4_8),
            3 => Some(UART_IFLS_RX_A::UART_IFLS_RX6_8),
            4 => Some(UART_IFLS_RX_A::UART_IFLS_RX7_8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_RX1_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_rx1_8(&self) -> bool {
        *self == UART_IFLS_RX_A::UART_IFLS_RX1_8
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_RX2_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_rx2_8(&self) -> bool {
        *self == UART_IFLS_RX_A::UART_IFLS_RX2_8
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_RX4_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_rx4_8(&self) -> bool {
        *self == UART_IFLS_RX_A::UART_IFLS_RX4_8
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_RX6_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_rx6_8(&self) -> bool {
        *self == UART_IFLS_RX_A::UART_IFLS_RX6_8
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_RX7_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_rx7_8(&self) -> bool {
        *self == UART_IFLS_RX_A::UART_IFLS_RX7_8
    }
}
#[doc = "Field `UART_IFLS_RX` writer - UART Receive Interrupt FIFO Level Select"]
pub type UART_IFLS_RX_W<'a> = crate::FieldWriter<'a, u32, IFLS_SPEC, u8, UART_IFLS_RX_A, 3, 3>;
impl<'a> UART_IFLS_RX_W<'a> {
    #[doc = "RX FIFO >= 1/8 full"]
    #[inline(always)]
    pub fn uart_ifls_rx1_8(self) -> &'a mut W {
        self.variant(UART_IFLS_RX_A::UART_IFLS_RX1_8)
    }
    #[doc = "RX FIFO >= 1/4 full"]
    #[inline(always)]
    pub fn uart_ifls_rx2_8(self) -> &'a mut W {
        self.variant(UART_IFLS_RX_A::UART_IFLS_RX2_8)
    }
    #[doc = "RX FIFO >= 1/2 full (default)"]
    #[inline(always)]
    pub fn uart_ifls_rx4_8(self) -> &'a mut W {
        self.variant(UART_IFLS_RX_A::UART_IFLS_RX4_8)
    }
    #[doc = "RX FIFO >= 3/4 full"]
    #[inline(always)]
    pub fn uart_ifls_rx6_8(self) -> &'a mut W {
        self.variant(UART_IFLS_RX_A::UART_IFLS_RX6_8)
    }
    #[doc = "RX FIFO >= 7/8 full"]
    #[inline(always)]
    pub fn uart_ifls_rx7_8(self) -> &'a mut W {
        self.variant(UART_IFLS_RX_A::UART_IFLS_RX7_8)
    }
}
impl R {
    #[doc = "Bits 0:2 - UART Transmit Interrupt FIFO Level Select"]
    #[inline(always)]
    pub fn uart_ifls_tx(&self) -> UART_IFLS_TX_R {
        UART_IFLS_TX_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - UART Receive Interrupt FIFO Level Select"]
    #[inline(always)]
    pub fn uart_ifls_rx(&self) -> UART_IFLS_RX_R {
        UART_IFLS_RX_R::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - UART Transmit Interrupt FIFO Level Select"]
    #[inline(always)]
    pub fn uart_ifls_tx(&mut self) -> UART_IFLS_TX_W {
        UART_IFLS_TX_W::new(self)
    }
    #[doc = "Bits 3:5 - UART Receive Interrupt FIFO Level Select"]
    #[inline(always)]
    pub fn uart_ifls_rx(&mut self) -> UART_IFLS_RX_W {
        UART_IFLS_RX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Interrupt FIFO Level Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifls](index.html) module"]
pub struct IFLS_SPEC;
impl crate::RegisterSpec for IFLS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ifls::R](R) reader structure"]
impl crate::Readable for IFLS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ifls::W](W) writer structure"]
impl crate::Writable for IFLS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFLS to value 0"]
impl crate::Resettable for IFLS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
