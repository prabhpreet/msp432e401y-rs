#[doc = "Register `DMAINTR` reader"]
pub struct R(crate::R<DMAINTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAINTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAINTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAINTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAINTR` writer"]
pub struct W(crate::W<DMAINTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAINTR_SPEC>;
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
impl From<crate::W<DMAINTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAINTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_DMAINTR_CH0` reader - Channel 0 DMA Interrupt"]
pub type USB_DMAINTR_CH0_R = crate::BitReader<bool>;
#[doc = "Field `USB_DMAINTR_CH0` writer - Channel 0 DMA Interrupt"]
pub type USB_DMAINTR_CH0_W<'a> = crate::BitWriter<'a, u8, DMAINTR_SPEC, bool, 0>;
#[doc = "Field `USB_DMAINTR_CH1` reader - Channel 1 DMA Interrupt"]
pub type USB_DMAINTR_CH1_R = crate::BitReader<bool>;
#[doc = "Field `USB_DMAINTR_CH1` writer - Channel 1 DMA Interrupt"]
pub type USB_DMAINTR_CH1_W<'a> = crate::BitWriter<'a, u8, DMAINTR_SPEC, bool, 1>;
#[doc = "Field `USB_DMAINTR_CH2` reader - Channel 2 DMA Interrupt"]
pub type USB_DMAINTR_CH2_R = crate::BitReader<bool>;
#[doc = "Field `USB_DMAINTR_CH2` writer - Channel 2 DMA Interrupt"]
pub type USB_DMAINTR_CH2_W<'a> = crate::BitWriter<'a, u8, DMAINTR_SPEC, bool, 2>;
#[doc = "Field `USB_DMAINTR_CH3` reader - Channel 3 DMA Interrupt"]
pub type USB_DMAINTR_CH3_R = crate::BitReader<bool>;
#[doc = "Field `USB_DMAINTR_CH3` writer - Channel 3 DMA Interrupt"]
pub type USB_DMAINTR_CH3_W<'a> = crate::BitWriter<'a, u8, DMAINTR_SPEC, bool, 3>;
#[doc = "Field `USB_DMAINTR_CH4` reader - Channel 4 DMA Interrupt"]
pub type USB_DMAINTR_CH4_R = crate::BitReader<bool>;
#[doc = "Field `USB_DMAINTR_CH4` writer - Channel 4 DMA Interrupt"]
pub type USB_DMAINTR_CH4_W<'a> = crate::BitWriter<'a, u8, DMAINTR_SPEC, bool, 4>;
#[doc = "Field `USB_DMAINTR_CH5` reader - Channel 5 DMA Interrupt"]
pub type USB_DMAINTR_CH5_R = crate::BitReader<bool>;
#[doc = "Field `USB_DMAINTR_CH5` writer - Channel 5 DMA Interrupt"]
pub type USB_DMAINTR_CH5_W<'a> = crate::BitWriter<'a, u8, DMAINTR_SPEC, bool, 5>;
#[doc = "Field `USB_DMAINTR_CH6` reader - Channel 6 DMA Interrupt"]
pub type USB_DMAINTR_CH6_R = crate::BitReader<bool>;
#[doc = "Field `USB_DMAINTR_CH6` writer - Channel 6 DMA Interrupt"]
pub type USB_DMAINTR_CH6_W<'a> = crate::BitWriter<'a, u8, DMAINTR_SPEC, bool, 6>;
#[doc = "Field `USB_DMAINTR_CH7` reader - Channel 7 DMA Interrupt"]
pub type USB_DMAINTR_CH7_R = crate::BitReader<bool>;
#[doc = "Field `USB_DMAINTR_CH7` writer - Channel 7 DMA Interrupt"]
pub type USB_DMAINTR_CH7_W<'a> = crate::BitWriter<'a, u8, DMAINTR_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Channel 0 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch0(&self) -> USB_DMAINTR_CH0_R {
        USB_DMAINTR_CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch1(&self) -> USB_DMAINTR_CH1_R {
        USB_DMAINTR_CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch2(&self) -> USB_DMAINTR_CH2_R {
        USB_DMAINTR_CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch3(&self) -> USB_DMAINTR_CH3_R {
        USB_DMAINTR_CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch4(&self) -> USB_DMAINTR_CH4_R {
        USB_DMAINTR_CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch5(&self) -> USB_DMAINTR_CH5_R {
        USB_DMAINTR_CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch6(&self) -> USB_DMAINTR_CH6_R {
        USB_DMAINTR_CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch7(&self) -> USB_DMAINTR_CH7_R {
        USB_DMAINTR_CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch0(&mut self) -> USB_DMAINTR_CH0_W {
        USB_DMAINTR_CH0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch1(&mut self) -> USB_DMAINTR_CH1_W {
        USB_DMAINTR_CH1_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch2(&mut self) -> USB_DMAINTR_CH2_W {
        USB_DMAINTR_CH2_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch3(&mut self) -> USB_DMAINTR_CH3_W {
        USB_DMAINTR_CH3_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch4(&mut self) -> USB_DMAINTR_CH4_W {
        USB_DMAINTR_CH4_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch5(&mut self) -> USB_DMAINTR_CH5_W {
        USB_DMAINTR_CH5_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch6(&mut self) -> USB_DMAINTR_CH6_W {
        USB_DMAINTR_CH6_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch7(&mut self) -> USB_DMAINTR_CH7_W {
        USB_DMAINTR_CH7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB DMA Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaintr](index.html) module"]
pub struct DMAINTR_SPEC;
impl crate::RegisterSpec for DMAINTR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dmaintr::R](R) reader structure"]
impl crate::Readable for DMAINTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaintr::W](W) writer structure"]
impl crate::Writable for DMAINTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAINTR to value 0"]
impl crate::Resettable for DMAINTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
