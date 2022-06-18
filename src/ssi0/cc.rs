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
#[doc = "SSI Baud Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSI_CC_CS_A {
    #[doc = "0: System clock (based on clock source and divisor factor)"]
    SSI_CC_CS_SYSPLL = 0,
    #[doc = "5: PIOSC"]
    SSI_CC_CS_PIOSC = 5,
}
impl From<SSI_CC_CS_A> for u8 {
    #[inline(always)]
    fn from(variant: SSI_CC_CS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SSI_CC_CS` reader - SSI Baud Clock Source"]
pub type SSI_CC_CS_R = crate::FieldReader<u8, SSI_CC_CS_A>;
impl SSI_CC_CS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SSI_CC_CS_A> {
        match self.bits {
            0 => Some(SSI_CC_CS_A::SSI_CC_CS_SYSPLL),
            5 => Some(SSI_CC_CS_A::SSI_CC_CS_PIOSC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SSI_CC_CS_SYSPLL`"]
    #[inline(always)]
    pub fn is_ssi_cc_cs_syspll(&self) -> bool {
        *self == SSI_CC_CS_A::SSI_CC_CS_SYSPLL
    }
    #[doc = "Checks if the value of the field is `SSI_CC_CS_PIOSC`"]
    #[inline(always)]
    pub fn is_ssi_cc_cs_piosc(&self) -> bool {
        *self == SSI_CC_CS_A::SSI_CC_CS_PIOSC
    }
}
#[doc = "Field `SSI_CC_CS` writer - SSI Baud Clock Source"]
pub type SSI_CC_CS_W<'a> = crate::FieldWriter<'a, u32, CC_SPEC, u8, SSI_CC_CS_A, 4, 0>;
impl<'a> SSI_CC_CS_W<'a> {
    #[doc = "System clock (based on clock source and divisor factor)"]
    #[inline(always)]
    pub fn ssi_cc_cs_syspll(self) -> &'a mut W {
        self.variant(SSI_CC_CS_A::SSI_CC_CS_SYSPLL)
    }
    #[doc = "PIOSC"]
    #[inline(always)]
    pub fn ssi_cc_cs_piosc(self) -> &'a mut W {
        self.variant(SSI_CC_CS_A::SSI_CC_CS_PIOSC)
    }
}
impl R {
    #[doc = "Bits 0:3 - SSI Baud Clock Source"]
    #[inline(always)]
    pub fn ssi_cc_cs(&self) -> SSI_CC_CS_R {
        SSI_CC_CS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SSI Baud Clock Source"]
    #[inline(always)]
    pub fn ssi_cc_cs(&mut self) -> SSI_CC_CS_W {
        SSI_CC_CS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SSI Clock Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc](index.html) module"]
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
