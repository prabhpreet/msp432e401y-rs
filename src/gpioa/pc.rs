#[doc = "Register `PC` reader"]
pub struct R(crate::R<PC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PC` writer"]
pub struct W(crate::W<PC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PC_SPEC>;
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
impl From<crate::W<PC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Extended Drive Mode Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO_PC_EDM0_A {
    #[doc = "0: Drive values of 2, 4 and 8 mA are maintained. GPIO n Drive Select (GPIODRnR) registers function as normal"]
    GPIO_PC_EDM0_DISABLE = 0,
    #[doc = "1: An additional 6 mA option is provided"]
    GPIO_PC_EDM0_6MA = 1,
    #[doc = "3: A 2 mA driver is always enabled; setting the corresponding GPIODR4R register bit adds 2 mA and setting the corresponding GPIODR8R of GPIODR12R register bit adds an additional 4 mA"]
    GPIO_PC_EDM0_PLUS2MA = 3,
}
impl From<GPIO_PC_EDM0_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO_PC_EDM0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO_PC_EDM0` reader - Extended Drive Mode Bit 0"]
pub type GPIO_PC_EDM0_R = crate::FieldReader<u8, GPIO_PC_EDM0_A>;
impl GPIO_PC_EDM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_PC_EDM0_A> {
        match self.bits {
            0 => Some(GPIO_PC_EDM0_A::GPIO_PC_EDM0_DISABLE),
            1 => Some(GPIO_PC_EDM0_A::GPIO_PC_EDM0_6MA),
            3 => Some(GPIO_PC_EDM0_A::GPIO_PC_EDM0_PLUS2MA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_PC_EDM0_DISABLE`"]
    #[inline(always)]
    pub fn is_gpio_pc_edm0_disable(&self) -> bool {
        *self == GPIO_PC_EDM0_A::GPIO_PC_EDM0_DISABLE
    }
    #[doc = "Checks if the value of the field is `GPIO_PC_EDM0_6MA`"]
    #[inline(always)]
    pub fn is_gpio_pc_edm0_6ma(&self) -> bool {
        *self == GPIO_PC_EDM0_A::GPIO_PC_EDM0_6MA
    }
    #[doc = "Checks if the value of the field is `GPIO_PC_EDM0_PLUS2MA`"]
    #[inline(always)]
    pub fn is_gpio_pc_edm0_plus2ma(&self) -> bool {
        *self == GPIO_PC_EDM0_A::GPIO_PC_EDM0_PLUS2MA
    }
}
#[doc = "Field `GPIO_PC_EDM0` writer - Extended Drive Mode Bit 0"]
pub type GPIO_PC_EDM0_W<'a> = crate::FieldWriter<'a, u32, PC_SPEC, u8, GPIO_PC_EDM0_A, 2, 0>;
impl<'a> GPIO_PC_EDM0_W<'a> {
    #[doc = "Drive values of 2, 4 and 8 mA are maintained. GPIO n Drive Select (GPIODRnR) registers function as normal"]
    #[inline(always)]
    pub fn gpio_pc_edm0_disable(self) -> &'a mut W {
        self.variant(GPIO_PC_EDM0_A::GPIO_PC_EDM0_DISABLE)
    }
    #[doc = "An additional 6 mA option is provided"]
    #[inline(always)]
    pub fn gpio_pc_edm0_6ma(self) -> &'a mut W {
        self.variant(GPIO_PC_EDM0_A::GPIO_PC_EDM0_6MA)
    }
    #[doc = "A 2 mA driver is always enabled; setting the corresponding GPIODR4R register bit adds 2 mA and setting the corresponding GPIODR8R of GPIODR12R register bit adds an additional 4 mA"]
    #[inline(always)]
    pub fn gpio_pc_edm0_plus2ma(self) -> &'a mut W {
        self.variant(GPIO_PC_EDM0_A::GPIO_PC_EDM0_PLUS2MA)
    }
}
#[doc = "Field `GPIO_PC_EDM1` reader - Extended Drive Mode Bit 1"]
pub type GPIO_PC_EDM1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIO_PC_EDM1` writer - Extended Drive Mode Bit 1"]
pub type GPIO_PC_EDM1_W<'a> = crate::FieldWriter<'a, u32, PC_SPEC, u8, u8, 2, 2>;
#[doc = "Field `GPIO_PC_EDM2` reader - Extended Drive Mode Bit 2"]
pub type GPIO_PC_EDM2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIO_PC_EDM2` writer - Extended Drive Mode Bit 2"]
pub type GPIO_PC_EDM2_W<'a> = crate::FieldWriter<'a, u32, PC_SPEC, u8, u8, 2, 4>;
#[doc = "Field `GPIO_PC_EDM3` reader - Extended Drive Mode Bit 3"]
pub type GPIO_PC_EDM3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIO_PC_EDM3` writer - Extended Drive Mode Bit 3"]
pub type GPIO_PC_EDM3_W<'a> = crate::FieldWriter<'a, u32, PC_SPEC, u8, u8, 2, 6>;
#[doc = "Field `GPIO_PC_EDM4` reader - Extended Drive Mode Bit 4"]
pub type GPIO_PC_EDM4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIO_PC_EDM4` writer - Extended Drive Mode Bit 4"]
pub type GPIO_PC_EDM4_W<'a> = crate::FieldWriter<'a, u32, PC_SPEC, u8, u8, 2, 8>;
#[doc = "Field `GPIO_PC_EDM5` reader - Extended Drive Mode Bit 5"]
pub type GPIO_PC_EDM5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIO_PC_EDM5` writer - Extended Drive Mode Bit 5"]
pub type GPIO_PC_EDM5_W<'a> = crate::FieldWriter<'a, u32, PC_SPEC, u8, u8, 2, 10>;
#[doc = "Field `GPIO_PC_EDM6` reader - Extended Drive Mode Bit 6"]
pub type GPIO_PC_EDM6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIO_PC_EDM6` writer - Extended Drive Mode Bit 6"]
pub type GPIO_PC_EDM6_W<'a> = crate::FieldWriter<'a, u32, PC_SPEC, u8, u8, 2, 12>;
#[doc = "Field `GPIO_PC_EDM7` reader - Extended Drive Mode Bit 7"]
pub type GPIO_PC_EDM7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIO_PC_EDM7` writer - Extended Drive Mode Bit 7"]
pub type GPIO_PC_EDM7_W<'a> = crate::FieldWriter<'a, u32, PC_SPEC, u8, u8, 2, 14>;
impl R {
    #[doc = "Bits 0:1 - Extended Drive Mode Bit 0"]
    #[inline(always)]
    pub fn gpio_pc_edm0(&self) -> GPIO_PC_EDM0_R {
        GPIO_PC_EDM0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Extended Drive Mode Bit 1"]
    #[inline(always)]
    pub fn gpio_pc_edm1(&self) -> GPIO_PC_EDM1_R {
        GPIO_PC_EDM1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Extended Drive Mode Bit 2"]
    #[inline(always)]
    pub fn gpio_pc_edm2(&self) -> GPIO_PC_EDM2_R {
        GPIO_PC_EDM2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Extended Drive Mode Bit 3"]
    #[inline(always)]
    pub fn gpio_pc_edm3(&self) -> GPIO_PC_EDM3_R {
        GPIO_PC_EDM3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Extended Drive Mode Bit 4"]
    #[inline(always)]
    pub fn gpio_pc_edm4(&self) -> GPIO_PC_EDM4_R {
        GPIO_PC_EDM4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Extended Drive Mode Bit 5"]
    #[inline(always)]
    pub fn gpio_pc_edm5(&self) -> GPIO_PC_EDM5_R {
        GPIO_PC_EDM5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Extended Drive Mode Bit 6"]
    #[inline(always)]
    pub fn gpio_pc_edm6(&self) -> GPIO_PC_EDM6_R {
        GPIO_PC_EDM6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Extended Drive Mode Bit 7"]
    #[inline(always)]
    pub fn gpio_pc_edm7(&self) -> GPIO_PC_EDM7_R {
        GPIO_PC_EDM7_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Extended Drive Mode Bit 0"]
    #[inline(always)]
    pub fn gpio_pc_edm0(&mut self) -> GPIO_PC_EDM0_W {
        GPIO_PC_EDM0_W::new(self)
    }
    #[doc = "Bits 2:3 - Extended Drive Mode Bit 1"]
    #[inline(always)]
    pub fn gpio_pc_edm1(&mut self) -> GPIO_PC_EDM1_W {
        GPIO_PC_EDM1_W::new(self)
    }
    #[doc = "Bits 4:5 - Extended Drive Mode Bit 2"]
    #[inline(always)]
    pub fn gpio_pc_edm2(&mut self) -> GPIO_PC_EDM2_W {
        GPIO_PC_EDM2_W::new(self)
    }
    #[doc = "Bits 6:7 - Extended Drive Mode Bit 3"]
    #[inline(always)]
    pub fn gpio_pc_edm3(&mut self) -> GPIO_PC_EDM3_W {
        GPIO_PC_EDM3_W::new(self)
    }
    #[doc = "Bits 8:9 - Extended Drive Mode Bit 4"]
    #[inline(always)]
    pub fn gpio_pc_edm4(&mut self) -> GPIO_PC_EDM4_W {
        GPIO_PC_EDM4_W::new(self)
    }
    #[doc = "Bits 10:11 - Extended Drive Mode Bit 5"]
    #[inline(always)]
    pub fn gpio_pc_edm5(&mut self) -> GPIO_PC_EDM5_W {
        GPIO_PC_EDM5_W::new(self)
    }
    #[doc = "Bits 12:13 - Extended Drive Mode Bit 6"]
    #[inline(always)]
    pub fn gpio_pc_edm6(&mut self) -> GPIO_PC_EDM6_W {
        GPIO_PC_EDM6_W::new(self)
    }
    #[doc = "Bits 14:15 - Extended Drive Mode Bit 7"]
    #[inline(always)]
    pub fn gpio_pc_edm7(&mut self) -> GPIO_PC_EDM7_W {
        GPIO_PC_EDM7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Peripheral Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc](index.html) module"]
pub struct PC_SPEC;
impl crate::RegisterSpec for PC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pc::R](R) reader structure"]
impl crate::Readable for PC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pc::W](W) writer structure"]
impl crate::Writable for PC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PC to value 0"]
impl crate::Resettable for PC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
