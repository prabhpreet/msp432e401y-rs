#[doc = "Register `SCSR` reader"]
pub struct R(crate::R<I2C0_ALT_SCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C0_ALT_SCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C0_ALT_SCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C0_ALT_SCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCSR` writer"]
pub struct W(crate::W<I2C0_ALT_SCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C0_ALT_SCSR_SPEC>;
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
impl From<crate::W<I2C0_ALT_SCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C0_ALT_SCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_SCSR_DA` reader - Device Active"]
pub type I2C_SCSR_DA_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SCSR_DA` writer - Device Active"]
pub type I2C_SCSR_DA_W<'a> = crate::BitWriter<'a, u32, I2C0_ALT_SCSR_SPEC, bool, 0>;
#[doc = "Field `I2C_SCSR_TREQ` reader - Transmit Request"]
pub type I2C_SCSR_TREQ_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SCSR_TREQ` writer - Transmit Request"]
pub type I2C_SCSR_TREQ_W<'a> = crate::BitWriter<'a, u32, I2C0_ALT_SCSR_SPEC, bool, 1>;
#[doc = "Field `I2C_SCSR_RXFIFO` reader - RX FIFO Enable"]
pub type I2C_SCSR_RXFIFO_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SCSR_RXFIFO` writer - RX FIFO Enable"]
pub type I2C_SCSR_RXFIFO_W<'a> = crate::BitWriter<'a, u32, I2C0_ALT_SCSR_SPEC, bool, 2>;
impl R {
    #[doc = "Bit 0 - Device Active"]
    #[inline(always)]
    pub fn i2c_scsr_da(&self) -> I2C_SCSR_DA_R {
        I2C_SCSR_DA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Request"]
    #[inline(always)]
    pub fn i2c_scsr_treq(&self) -> I2C_SCSR_TREQ_R {
        I2C_SCSR_TREQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX FIFO Enable"]
    #[inline(always)]
    pub fn i2c_scsr_rxfifo(&self) -> I2C_SCSR_RXFIFO_R {
        I2C_SCSR_RXFIFO_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Device Active"]
    #[inline(always)]
    pub fn i2c_scsr_da(&mut self) -> I2C_SCSR_DA_W {
        I2C_SCSR_DA_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Request"]
    #[inline(always)]
    pub fn i2c_scsr_treq(&mut self) -> I2C_SCSR_TREQ_W {
        I2C_SCSR_TREQ_W::new(self)
    }
    #[doc = "Bit 2 - RX FIFO Enable"]
    #[inline(always)]
    pub fn i2c_scsr_rxfifo(&mut self) -> I2C_SCSR_RXFIFO_W {
        I2C_SCSR_RXFIFO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Slave Control/Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c0_alt_scsr](index.html) module"]
pub struct I2C0_ALT_SCSR_SPEC;
impl crate::RegisterSpec for I2C0_ALT_SCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c0_alt_scsr::R](R) reader structure"]
impl crate::Readable for I2C0_ALT_SCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c0_alt_scsr::W](W) writer structure"]
impl crate::Writable for I2C0_ALT_SCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCSR to value 0"]
impl crate::Resettable for I2C0_ALT_SCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
