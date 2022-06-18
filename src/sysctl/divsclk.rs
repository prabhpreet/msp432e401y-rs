#[doc = "Register `DIVSCLK` reader"]
pub struct R(crate::R<DIVSCLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIVSCLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIVSCLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIVSCLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIVSCLK` writer"]
pub struct W(crate::W<DIVSCLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIVSCLK_SPEC>;
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
impl From<crate::W<DIVSCLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIVSCLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_DIVSCLK_DIV` reader - Divisor Value"]
pub type SYSCTL_DIVSCLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYSCTL_DIVSCLK_DIV` writer - Divisor Value"]
pub type SYSCTL_DIVSCLK_DIV_W<'a> = crate::FieldWriter<'a, u32, DIVSCLK_SPEC, u8, u8, 8, 0>;
#[doc = "Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_DIVSCLK_SRC_A {
    #[doc = "0: System Clock"]
    SYSCTL_DIVSCLK_SRC_SYSCLK = 0,
    #[doc = "1: PIOSC"]
    SYSCTL_DIVSCLK_SRC_PIOSC = 1,
    #[doc = "2: MOSC"]
    SYSCTL_DIVSCLK_SRC_MOSC = 2,
}
impl From<SYSCTL_DIVSCLK_SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DIVSCLK_SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_DIVSCLK_SRC` reader - Clock Source"]
pub type SYSCTL_DIVSCLK_SRC_R = crate::FieldReader<u8, SYSCTL_DIVSCLK_SRC_A>;
impl SYSCTL_DIVSCLK_SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_DIVSCLK_SRC_A> {
        match self.bits {
            0 => Some(SYSCTL_DIVSCLK_SRC_A::SYSCTL_DIVSCLK_SRC_SYSCLK),
            1 => Some(SYSCTL_DIVSCLK_SRC_A::SYSCTL_DIVSCLK_SRC_PIOSC),
            2 => Some(SYSCTL_DIVSCLK_SRC_A::SYSCTL_DIVSCLK_SRC_MOSC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DIVSCLK_SRC_SYSCLK`"]
    #[inline(always)]
    pub fn is_sysctl_divsclk_src_sysclk(&self) -> bool {
        *self == SYSCTL_DIVSCLK_SRC_A::SYSCTL_DIVSCLK_SRC_SYSCLK
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DIVSCLK_SRC_PIOSC`"]
    #[inline(always)]
    pub fn is_sysctl_divsclk_src_piosc(&self) -> bool {
        *self == SYSCTL_DIVSCLK_SRC_A::SYSCTL_DIVSCLK_SRC_PIOSC
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DIVSCLK_SRC_MOSC`"]
    #[inline(always)]
    pub fn is_sysctl_divsclk_src_mosc(&self) -> bool {
        *self == SYSCTL_DIVSCLK_SRC_A::SYSCTL_DIVSCLK_SRC_MOSC
    }
}
#[doc = "Field `SYSCTL_DIVSCLK_SRC` writer - Clock Source"]
pub type SYSCTL_DIVSCLK_SRC_W<'a> =
    crate::FieldWriter<'a, u32, DIVSCLK_SPEC, u8, SYSCTL_DIVSCLK_SRC_A, 2, 16>;
impl<'a> SYSCTL_DIVSCLK_SRC_W<'a> {
    #[doc = "System Clock"]
    #[inline(always)]
    pub fn sysctl_divsclk_src_sysclk(self) -> &'a mut W {
        self.variant(SYSCTL_DIVSCLK_SRC_A::SYSCTL_DIVSCLK_SRC_SYSCLK)
    }
    #[doc = "PIOSC"]
    #[inline(always)]
    pub fn sysctl_divsclk_src_piosc(self) -> &'a mut W {
        self.variant(SYSCTL_DIVSCLK_SRC_A::SYSCTL_DIVSCLK_SRC_PIOSC)
    }
    #[doc = "MOSC"]
    #[inline(always)]
    pub fn sysctl_divsclk_src_mosc(self) -> &'a mut W {
        self.variant(SYSCTL_DIVSCLK_SRC_A::SYSCTL_DIVSCLK_SRC_MOSC)
    }
}
#[doc = "Field `SYSCTL_DIVSCLK_EN` reader - DIVSCLK Enable"]
pub type SYSCTL_DIVSCLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DIVSCLK_EN` writer - DIVSCLK Enable"]
pub type SYSCTL_DIVSCLK_EN_W<'a> = crate::BitWriter<'a, u32, DIVSCLK_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:7 - Divisor Value"]
    #[inline(always)]
    pub fn sysctl_divsclk_div(&self) -> SYSCTL_DIVSCLK_DIV_R {
        SYSCTL_DIVSCLK_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Clock Source"]
    #[inline(always)]
    pub fn sysctl_divsclk_src(&self) -> SYSCTL_DIVSCLK_SRC_R {
        SYSCTL_DIVSCLK_SRC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 31 - DIVSCLK Enable"]
    #[inline(always)]
    pub fn sysctl_divsclk_en(&self) -> SYSCTL_DIVSCLK_EN_R {
        SYSCTL_DIVSCLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Divisor Value"]
    #[inline(always)]
    pub fn sysctl_divsclk_div(&mut self) -> SYSCTL_DIVSCLK_DIV_W {
        SYSCTL_DIVSCLK_DIV_W::new(self)
    }
    #[doc = "Bits 16:17 - Clock Source"]
    #[inline(always)]
    pub fn sysctl_divsclk_src(&mut self) -> SYSCTL_DIVSCLK_SRC_W {
        SYSCTL_DIVSCLK_SRC_W::new(self)
    }
    #[doc = "Bit 31 - DIVSCLK Enable"]
    #[inline(always)]
    pub fn sysctl_divsclk_en(&mut self) -> SYSCTL_DIVSCLK_EN_W {
        SYSCTL_DIVSCLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Divisor and Source Clock Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [divsclk](index.html) module"]
pub struct DIVSCLK_SPEC;
impl crate::RegisterSpec for DIVSCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [divsclk::R](R) reader structure"]
impl crate::Readable for DIVSCLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [divsclk::W](W) writer structure"]
impl crate::Writable for DIVSCLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIVSCLK to value 0"]
impl crate::Resettable for DIVSCLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
