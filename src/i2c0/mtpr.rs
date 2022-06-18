#[doc = "Register `MTPR` reader"]
pub struct R(crate::R<MTPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTPR` writer"]
pub struct W(crate::W<MTPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTPR_SPEC>;
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
impl From<crate::W<MTPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_MTPR_TPR` reader - Timer Period"]
pub type I2C_MTPR_TPR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2C_MTPR_TPR` writer - Timer Period"]
pub type I2C_MTPR_TPR_W<'a> = crate::FieldWriter<'a, u32, MTPR_SPEC, u8, u8, 7, 0>;
#[doc = "Field `I2C_MTPR_HS` reader - High-Speed Enable"]
pub type I2C_MTPR_HS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MTPR_HS` writer - High-Speed Enable"]
pub type I2C_MTPR_HS_W<'a> = crate::BitWriter<'a, u32, MTPR_SPEC, bool, 7>;
#[doc = "Glitch Suppression Pulse Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2C_MTPR_PULSEL_A {
    #[doc = "0: Bypass"]
    I2C_MTPR_PULSEL_BYPASS = 0,
    #[doc = "1: 1 clock"]
    I2C_MTPR_PULSEL_1 = 1,
    #[doc = "2: 2 clocks"]
    I2C_MTPR_PULSEL_2 = 2,
    #[doc = "3: 3 clocks"]
    I2C_MTPR_PULSEL_3 = 3,
    #[doc = "4: 4 clocks"]
    I2C_MTPR_PULSEL_4 = 4,
    #[doc = "5: 8 clocks"]
    I2C_MTPR_PULSEL_8 = 5,
    #[doc = "6: 16 clocks"]
    I2C_MTPR_PULSEL_16 = 6,
    #[doc = "7: 31 clocks"]
    I2C_MTPR_PULSEL_31 = 7,
}
impl From<I2C_MTPR_PULSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C_MTPR_PULSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `I2C_MTPR_PULSEL` reader - Glitch Suppression Pulse Width"]
pub type I2C_MTPR_PULSEL_R = crate::FieldReader<u8, I2C_MTPR_PULSEL_A>;
impl I2C_MTPR_PULSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_MTPR_PULSEL_A {
        match self.bits {
            0 => I2C_MTPR_PULSEL_A::I2C_MTPR_PULSEL_BYPASS,
            1 => I2C_MTPR_PULSEL_A::I2C_MTPR_PULSEL_1,
            2 => I2C_MTPR_PULSEL_A::I2C_MTPR_PULSEL_2,
            3 => I2C_MTPR_PULSEL_A::I2C_MTPR_PULSEL_3,
            4 => I2C_MTPR_PULSEL_A::I2C_MTPR_PULSEL_4,
            5 => I2C_MTPR_PULSEL_A::I2C_MTPR_PULSEL_8,
            6 => I2C_MTPR_PULSEL_A::I2C_MTPR_PULSEL_16,
            7 => I2C_MTPR_PULSEL_A::I2C_MTPR_PULSEL_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `I2C_MTPR_PULSEL_BYPASS`"]
    #[inline(always)]
    pub fn is_i2c_mtpr_pulsel_bypass(&self) -> bool {
        *self == I2C_MTPR_PULSEL_A::I2C_MTPR_PULSEL_BYPASS
    }
    #[doc = "Checks if the value of the field is `I2C_MTPR_PULSEL_1`"]
    #[inline(always)]
    pub fn is_i2c_mtpr_pulsel_1(&self) -> bool {
        *self == I2C_MTPR_PULSEL_A::I2C_MTPR_PULSEL_1
    }
    #[doc = "Checks if the value of the field is `I2C_MTPR_PULSEL_2`"]
    #[inline(always)]
    pub fn is_i2c_mtpr_pulsel_2(&self) -> bool {
        *self == I2C_MTPR_PULSEL_A::I2C_MTPR_PULSEL_2
    }
    #[doc = "Checks if the value of the field is `I2C_MTPR_PULSEL_3`"]
    #[inline(always)]
    pub fn is_i2c_mtpr_pulsel_3(&self) -> bool {
        *self == I2C_MTPR_PULSEL_A::I2C_MTPR_PULSEL_3
    }
    #[doc = "Checks if the value of the field is `I2C_MTPR_PULSEL_4`"]
    #[inline(always)]
    pub fn is_i2c_mtpr_pulsel_4(&self) -> bool {
        *self == I2C_MTPR_PULSEL_A::I2C_MTPR_PULSEL_4
    }
    #[doc = "Checks if the value of the field is `I2C_MTPR_PULSEL_8`"]
    #[inline(always)]
    pub fn is_i2c_mtpr_pulsel_8(&self) -> bool {
        *self == I2C_MTPR_PULSEL_A::I2C_MTPR_PULSEL_8
    }
    #[doc = "Checks if the value of the field is `I2C_MTPR_PULSEL_16`"]
    #[inline(always)]
    pub fn is_i2c_mtpr_pulsel_16(&self) -> bool {
        *self == I2C_MTPR_PULSEL_A::I2C_MTPR_PULSEL_16
    }
    #[doc = "Checks if the value of the field is `I2C_MTPR_PULSEL_31`"]
    #[inline(always)]
    pub fn is_i2c_mtpr_pulsel_31(&self) -> bool {
        *self == I2C_MTPR_PULSEL_A::I2C_MTPR_PULSEL_31
    }
}
#[doc = "Field `I2C_MTPR_PULSEL` writer - Glitch Suppression Pulse Width"]
pub type I2C_MTPR_PULSEL_W<'a> =
    crate::FieldWriterSafe<'a, u32, MTPR_SPEC, u8, I2C_MTPR_PULSEL_A, 3, 16>;
impl<'a> I2C_MTPR_PULSEL_W<'a> {
    #[doc = "Bypass"]
    #[inline(always)]
    pub fn i2c_mtpr_pulsel_bypass(self) -> &'a mut W {
        self.variant(I2C_MTPR_PULSEL_A::I2C_MTPR_PULSEL_BYPASS)
    }
    #[doc = "1 clock"]
    #[inline(always)]
    pub fn i2c_mtpr_pulsel_1(self) -> &'a mut W {
        self.variant(I2C_MTPR_PULSEL_A::I2C_MTPR_PULSEL_1)
    }
    #[doc = "2 clocks"]
    #[inline(always)]
    pub fn i2c_mtpr_pulsel_2(self) -> &'a mut W {
        self.variant(I2C_MTPR_PULSEL_A::I2C_MTPR_PULSEL_2)
    }
    #[doc = "3 clocks"]
    #[inline(always)]
    pub fn i2c_mtpr_pulsel_3(self) -> &'a mut W {
        self.variant(I2C_MTPR_PULSEL_A::I2C_MTPR_PULSEL_3)
    }
    #[doc = "4 clocks"]
    #[inline(always)]
    pub fn i2c_mtpr_pulsel_4(self) -> &'a mut W {
        self.variant(I2C_MTPR_PULSEL_A::I2C_MTPR_PULSEL_4)
    }
    #[doc = "8 clocks"]
    #[inline(always)]
    pub fn i2c_mtpr_pulsel_8(self) -> &'a mut W {
        self.variant(I2C_MTPR_PULSEL_A::I2C_MTPR_PULSEL_8)
    }
    #[doc = "16 clocks"]
    #[inline(always)]
    pub fn i2c_mtpr_pulsel_16(self) -> &'a mut W {
        self.variant(I2C_MTPR_PULSEL_A::I2C_MTPR_PULSEL_16)
    }
    #[doc = "31 clocks"]
    #[inline(always)]
    pub fn i2c_mtpr_pulsel_31(self) -> &'a mut W {
        self.variant(I2C_MTPR_PULSEL_A::I2C_MTPR_PULSEL_31)
    }
}
impl R {
    #[doc = "Bits 0:6 - Timer Period"]
    #[inline(always)]
    pub fn i2c_mtpr_tpr(&self) -> I2C_MTPR_TPR_R {
        I2C_MTPR_TPR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - High-Speed Enable"]
    #[inline(always)]
    pub fn i2c_mtpr_hs(&self) -> I2C_MTPR_HS_R {
        I2C_MTPR_HS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Glitch Suppression Pulse Width"]
    #[inline(always)]
    pub fn i2c_mtpr_pulsel(&self) -> I2C_MTPR_PULSEL_R {
        I2C_MTPR_PULSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Timer Period"]
    #[inline(always)]
    pub fn i2c_mtpr_tpr(&mut self) -> I2C_MTPR_TPR_W {
        I2C_MTPR_TPR_W::new(self)
    }
    #[doc = "Bit 7 - High-Speed Enable"]
    #[inline(always)]
    pub fn i2c_mtpr_hs(&mut self) -> I2C_MTPR_HS_W {
        I2C_MTPR_HS_W::new(self)
    }
    #[doc = "Bits 16:18 - Glitch Suppression Pulse Width"]
    #[inline(always)]
    pub fn i2c_mtpr_pulsel(&mut self) -> I2C_MTPR_PULSEL_W {
        I2C_MTPR_PULSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Master Timer Period\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtpr](index.html) module"]
pub struct MTPR_SPEC;
impl crate::RegisterSpec for MTPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtpr::R](R) reader structure"]
impl crate::Readable for MTPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtpr::W](W) writer structure"]
impl crate::Writable for MTPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTPR to value 0"]
impl crate::Resettable for MTPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
