#[doc = "Register `ACREFCTL` reader"]
pub struct R(crate::R<ACREFCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACREFCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACREFCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACREFCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACREFCTL` writer"]
pub struct W(crate::W<ACREFCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACREFCTL_SPEC>;
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
impl From<crate::W<ACREFCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACREFCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP_ACREFCTL_VREF` reader - Resistor Ladder Voltage Ref"]
pub type COMP_ACREFCTL_VREF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP_ACREFCTL_VREF` writer - Resistor Ladder Voltage Ref"]
pub type COMP_ACREFCTL_VREF_W<'a> = crate::FieldWriter<'a, u32, ACREFCTL_SPEC, u8, u8, 4, 0>;
#[doc = "Field `COMP_ACREFCTL_RNG` reader - Resistor Ladder Range"]
pub type COMP_ACREFCTL_RNG_R = crate::BitReader<bool>;
#[doc = "Field `COMP_ACREFCTL_RNG` writer - Resistor Ladder Range"]
pub type COMP_ACREFCTL_RNG_W<'a> = crate::BitWriter<'a, u32, ACREFCTL_SPEC, bool, 8>;
#[doc = "Field `COMP_ACREFCTL_EN` reader - Resistor Ladder Enable"]
pub type COMP_ACREFCTL_EN_R = crate::BitReader<bool>;
#[doc = "Field `COMP_ACREFCTL_EN` writer - Resistor Ladder Enable"]
pub type COMP_ACREFCTL_EN_W<'a> = crate::BitWriter<'a, u32, ACREFCTL_SPEC, bool, 9>;
impl R {
    #[doc = "Bits 0:3 - Resistor Ladder Voltage Ref"]
    #[inline(always)]
    pub fn comp_acrefctl_vref(&self) -> COMP_ACREFCTL_VREF_R {
        COMP_ACREFCTL_VREF_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Resistor Ladder Range"]
    #[inline(always)]
    pub fn comp_acrefctl_rng(&self) -> COMP_ACREFCTL_RNG_R {
        COMP_ACREFCTL_RNG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Resistor Ladder Enable"]
    #[inline(always)]
    pub fn comp_acrefctl_en(&self) -> COMP_ACREFCTL_EN_R {
        COMP_ACREFCTL_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Resistor Ladder Voltage Ref"]
    #[inline(always)]
    pub fn comp_acrefctl_vref(&mut self) -> COMP_ACREFCTL_VREF_W {
        COMP_ACREFCTL_VREF_W::new(self)
    }
    #[doc = "Bit 8 - Resistor Ladder Range"]
    #[inline(always)]
    pub fn comp_acrefctl_rng(&mut self) -> COMP_ACREFCTL_RNG_W {
        COMP_ACREFCTL_RNG_W::new(self)
    }
    #[doc = "Bit 9 - Resistor Ladder Enable"]
    #[inline(always)]
    pub fn comp_acrefctl_en(&mut self) -> COMP_ACREFCTL_EN_W {
        COMP_ACREFCTL_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Comparator Reference Voltage Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acrefctl](index.html) module"]
pub struct ACREFCTL_SPEC;
impl crate::RegisterSpec for ACREFCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acrefctl::R](R) reader structure"]
impl crate::Readable for ACREFCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acrefctl::W](W) writer structure"]
impl crate::Writable for ACREFCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACREFCTL to value 0"]
impl crate::Resettable for ACREFCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
