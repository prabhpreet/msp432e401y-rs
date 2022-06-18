#[doc = "Register `ACCTL1` reader"]
pub struct R(crate::R<ACCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACCTL1` writer"]
pub struct W(crate::W<ACCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACCTL1_SPEC>;
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
impl From<crate::W<ACCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP_ACCTL1_CINV` reader - Comparator Output Invert"]
pub type COMP_ACCTL1_CINV_R = crate::BitReader<bool>;
#[doc = "Field `COMP_ACCTL1_CINV` writer - Comparator Output Invert"]
pub type COMP_ACCTL1_CINV_W<'a> = crate::BitWriter<'a, u32, ACCTL1_SPEC, bool, 1>;
#[doc = "Interrupt Sense\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP_ACCTL1_ISEN_A {
    #[doc = "0: Level sense, see ISLVAL"]
    COMP_ACCTL1_ISEN_LEVEL = 0,
    #[doc = "1: Falling edge"]
    COMP_ACCTL1_ISEN_FALL = 1,
    #[doc = "2: Rising edge"]
    COMP_ACCTL1_ISEN_RISE = 2,
    #[doc = "3: Either edge"]
    COMP_ACCTL1_ISEN_BOTH = 3,
}
impl From<COMP_ACCTL1_ISEN_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP_ACCTL1_ISEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP_ACCTL1_ISEN` reader - Interrupt Sense"]
pub type COMP_ACCTL1_ISEN_R = crate::FieldReader<u8, COMP_ACCTL1_ISEN_A>;
impl COMP_ACCTL1_ISEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP_ACCTL1_ISEN_A {
        match self.bits {
            0 => COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_LEVEL,
            1 => COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_FALL,
            2 => COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_RISE,
            3 => COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL1_ISEN_LEVEL`"]
    #[inline(always)]
    pub fn is_comp_acctl1_isen_level(&self) -> bool {
        *self == COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_LEVEL
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL1_ISEN_FALL`"]
    #[inline(always)]
    pub fn is_comp_acctl1_isen_fall(&self) -> bool {
        *self == COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_FALL
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL1_ISEN_RISE`"]
    #[inline(always)]
    pub fn is_comp_acctl1_isen_rise(&self) -> bool {
        *self == COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_RISE
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL1_ISEN_BOTH`"]
    #[inline(always)]
    pub fn is_comp_acctl1_isen_both(&self) -> bool {
        *self == COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_BOTH
    }
}
#[doc = "Field `COMP_ACCTL1_ISEN` writer - Interrupt Sense"]
pub type COMP_ACCTL1_ISEN_W<'a> =
    crate::FieldWriterSafe<'a, u32, ACCTL1_SPEC, u8, COMP_ACCTL1_ISEN_A, 2, 2>;
impl<'a> COMP_ACCTL1_ISEN_W<'a> {
    #[doc = "Level sense, see ISLVAL"]
    #[inline(always)]
    pub fn comp_acctl1_isen_level(self) -> &'a mut W {
        self.variant(COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_LEVEL)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn comp_acctl1_isen_fall(self) -> &'a mut W {
        self.variant(COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_FALL)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn comp_acctl1_isen_rise(self) -> &'a mut W {
        self.variant(COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_RISE)
    }
    #[doc = "Either edge"]
    #[inline(always)]
    pub fn comp_acctl1_isen_both(self) -> &'a mut W {
        self.variant(COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_BOTH)
    }
}
#[doc = "Field `COMP_ACCTL1_ISLVAL` reader - Interrupt Sense Level Value"]
pub type COMP_ACCTL1_ISLVAL_R = crate::BitReader<bool>;
#[doc = "Field `COMP_ACCTL1_ISLVAL` writer - Interrupt Sense Level Value"]
pub type COMP_ACCTL1_ISLVAL_W<'a> = crate::BitWriter<'a, u32, ACCTL1_SPEC, bool, 4>;
#[doc = "Trigger Sense\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP_ACCTL1_TSEN_A {
    #[doc = "0: Level sense, see TSLVAL"]
    COMP_ACCTL1_TSEN_LEVEL = 0,
    #[doc = "1: Falling edge"]
    COMP_ACCTL1_TSEN_FALL = 1,
    #[doc = "2: Rising edge"]
    COMP_ACCTL1_TSEN_RISE = 2,
    #[doc = "3: Either edge"]
    COMP_ACCTL1_TSEN_BOTH = 3,
}
impl From<COMP_ACCTL1_TSEN_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP_ACCTL1_TSEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP_ACCTL1_TSEN` reader - Trigger Sense"]
pub type COMP_ACCTL1_TSEN_R = crate::FieldReader<u8, COMP_ACCTL1_TSEN_A>;
impl COMP_ACCTL1_TSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP_ACCTL1_TSEN_A {
        match self.bits {
            0 => COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_LEVEL,
            1 => COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_FALL,
            2 => COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_RISE,
            3 => COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL1_TSEN_LEVEL`"]
    #[inline(always)]
    pub fn is_comp_acctl1_tsen_level(&self) -> bool {
        *self == COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_LEVEL
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL1_TSEN_FALL`"]
    #[inline(always)]
    pub fn is_comp_acctl1_tsen_fall(&self) -> bool {
        *self == COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_FALL
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL1_TSEN_RISE`"]
    #[inline(always)]
    pub fn is_comp_acctl1_tsen_rise(&self) -> bool {
        *self == COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_RISE
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL1_TSEN_BOTH`"]
    #[inline(always)]
    pub fn is_comp_acctl1_tsen_both(&self) -> bool {
        *self == COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_BOTH
    }
}
#[doc = "Field `COMP_ACCTL1_TSEN` writer - Trigger Sense"]
pub type COMP_ACCTL1_TSEN_W<'a> =
    crate::FieldWriterSafe<'a, u32, ACCTL1_SPEC, u8, COMP_ACCTL1_TSEN_A, 2, 5>;
impl<'a> COMP_ACCTL1_TSEN_W<'a> {
    #[doc = "Level sense, see TSLVAL"]
    #[inline(always)]
    pub fn comp_acctl1_tsen_level(self) -> &'a mut W {
        self.variant(COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_LEVEL)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn comp_acctl1_tsen_fall(self) -> &'a mut W {
        self.variant(COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_FALL)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn comp_acctl1_tsen_rise(self) -> &'a mut W {
        self.variant(COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_RISE)
    }
    #[doc = "Either edge"]
    #[inline(always)]
    pub fn comp_acctl1_tsen_both(self) -> &'a mut W {
        self.variant(COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_BOTH)
    }
}
#[doc = "Field `COMP_ACCTL1_TSLVAL` reader - Trigger Sense Level Value"]
pub type COMP_ACCTL1_TSLVAL_R = crate::BitReader<bool>;
#[doc = "Field `COMP_ACCTL1_TSLVAL` writer - Trigger Sense Level Value"]
pub type COMP_ACCTL1_TSLVAL_W<'a> = crate::BitWriter<'a, u32, ACCTL1_SPEC, bool, 7>;
#[doc = "Analog Source Positive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP_ACCTL1_ASRCP_A {
    #[doc = "0: Pin value of Cn+"]
    COMP_ACCTL1_ASRCP_PIN = 0,
    #[doc = "1: Pin value of C0+"]
    COMP_ACCTL1_ASRCP_PIN0 = 1,
    #[doc = "2: Internal voltage reference"]
    COMP_ACCTL1_ASRCP_REF = 2,
}
impl From<COMP_ACCTL1_ASRCP_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP_ACCTL1_ASRCP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP_ACCTL1_ASRCP` reader - Analog Source Positive"]
pub type COMP_ACCTL1_ASRCP_R = crate::FieldReader<u8, COMP_ACCTL1_ASRCP_A>;
impl COMP_ACCTL1_ASRCP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP_ACCTL1_ASRCP_A> {
        match self.bits {
            0 => Some(COMP_ACCTL1_ASRCP_A::COMP_ACCTL1_ASRCP_PIN),
            1 => Some(COMP_ACCTL1_ASRCP_A::COMP_ACCTL1_ASRCP_PIN0),
            2 => Some(COMP_ACCTL1_ASRCP_A::COMP_ACCTL1_ASRCP_REF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL1_ASRCP_PIN`"]
    #[inline(always)]
    pub fn is_comp_acctl1_asrcp_pin(&self) -> bool {
        *self == COMP_ACCTL1_ASRCP_A::COMP_ACCTL1_ASRCP_PIN
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL1_ASRCP_PIN0`"]
    #[inline(always)]
    pub fn is_comp_acctl1_asrcp_pin0(&self) -> bool {
        *self == COMP_ACCTL1_ASRCP_A::COMP_ACCTL1_ASRCP_PIN0
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL1_ASRCP_REF`"]
    #[inline(always)]
    pub fn is_comp_acctl1_asrcp_ref(&self) -> bool {
        *self == COMP_ACCTL1_ASRCP_A::COMP_ACCTL1_ASRCP_REF
    }
}
#[doc = "Field `COMP_ACCTL1_ASRCP` writer - Analog Source Positive"]
pub type COMP_ACCTL1_ASRCP_W<'a> =
    crate::FieldWriter<'a, u32, ACCTL1_SPEC, u8, COMP_ACCTL1_ASRCP_A, 2, 9>;
impl<'a> COMP_ACCTL1_ASRCP_W<'a> {
    #[doc = "Pin value of Cn+"]
    #[inline(always)]
    pub fn comp_acctl1_asrcp_pin(self) -> &'a mut W {
        self.variant(COMP_ACCTL1_ASRCP_A::COMP_ACCTL1_ASRCP_PIN)
    }
    #[doc = "Pin value of C0+"]
    #[inline(always)]
    pub fn comp_acctl1_asrcp_pin0(self) -> &'a mut W {
        self.variant(COMP_ACCTL1_ASRCP_A::COMP_ACCTL1_ASRCP_PIN0)
    }
    #[doc = "Internal voltage reference"]
    #[inline(always)]
    pub fn comp_acctl1_asrcp_ref(self) -> &'a mut W {
        self.variant(COMP_ACCTL1_ASRCP_A::COMP_ACCTL1_ASRCP_REF)
    }
}
#[doc = "Field `COMP_ACCTL1_TOEN` reader - Trigger Output Enable"]
pub type COMP_ACCTL1_TOEN_R = crate::BitReader<bool>;
#[doc = "Field `COMP_ACCTL1_TOEN` writer - Trigger Output Enable"]
pub type COMP_ACCTL1_TOEN_W<'a> = crate::BitWriter<'a, u32, ACCTL1_SPEC, bool, 11>;
impl R {
    #[doc = "Bit 1 - Comparator Output Invert"]
    #[inline(always)]
    pub fn comp_acctl1_cinv(&self) -> COMP_ACCTL1_CINV_R {
        COMP_ACCTL1_CINV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Interrupt Sense"]
    #[inline(always)]
    pub fn comp_acctl1_isen(&self) -> COMP_ACCTL1_ISEN_R {
        COMP_ACCTL1_ISEN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Interrupt Sense Level Value"]
    #[inline(always)]
    pub fn comp_acctl1_islval(&self) -> COMP_ACCTL1_ISLVAL_R {
        COMP_ACCTL1_ISLVAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Trigger Sense"]
    #[inline(always)]
    pub fn comp_acctl1_tsen(&self) -> COMP_ACCTL1_TSEN_R {
        COMP_ACCTL1_TSEN_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Trigger Sense Level Value"]
    #[inline(always)]
    pub fn comp_acctl1_tslval(&self) -> COMP_ACCTL1_TSLVAL_R {
        COMP_ACCTL1_TSLVAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Analog Source Positive"]
    #[inline(always)]
    pub fn comp_acctl1_asrcp(&self) -> COMP_ACCTL1_ASRCP_R {
        COMP_ACCTL1_ASRCP_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Trigger Output Enable"]
    #[inline(always)]
    pub fn comp_acctl1_toen(&self) -> COMP_ACCTL1_TOEN_R {
        COMP_ACCTL1_TOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Comparator Output Invert"]
    #[inline(always)]
    pub fn comp_acctl1_cinv(&mut self) -> COMP_ACCTL1_CINV_W {
        COMP_ACCTL1_CINV_W::new(self)
    }
    #[doc = "Bits 2:3 - Interrupt Sense"]
    #[inline(always)]
    pub fn comp_acctl1_isen(&mut self) -> COMP_ACCTL1_ISEN_W {
        COMP_ACCTL1_ISEN_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt Sense Level Value"]
    #[inline(always)]
    pub fn comp_acctl1_islval(&mut self) -> COMP_ACCTL1_ISLVAL_W {
        COMP_ACCTL1_ISLVAL_W::new(self)
    }
    #[doc = "Bits 5:6 - Trigger Sense"]
    #[inline(always)]
    pub fn comp_acctl1_tsen(&mut self) -> COMP_ACCTL1_TSEN_W {
        COMP_ACCTL1_TSEN_W::new(self)
    }
    #[doc = "Bit 7 - Trigger Sense Level Value"]
    #[inline(always)]
    pub fn comp_acctl1_tslval(&mut self) -> COMP_ACCTL1_TSLVAL_W {
        COMP_ACCTL1_TSLVAL_W::new(self)
    }
    #[doc = "Bits 9:10 - Analog Source Positive"]
    #[inline(always)]
    pub fn comp_acctl1_asrcp(&mut self) -> COMP_ACCTL1_ASRCP_W {
        COMP_ACCTL1_ASRCP_W::new(self)
    }
    #[doc = "Bit 11 - Trigger Output Enable"]
    #[inline(always)]
    pub fn comp_acctl1_toen(&mut self) -> COMP_ACCTL1_TOEN_W {
        COMP_ACCTL1_TOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Comparator Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acctl1](index.html) module"]
pub struct ACCTL1_SPEC;
impl crate::RegisterSpec for ACCTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acctl1::R](R) reader structure"]
impl crate::Readable for ACCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acctl1::W](W) writer structure"]
impl crate::Writable for ACCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACCTL1 to value 0"]
impl crate::Resettable for ACCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
