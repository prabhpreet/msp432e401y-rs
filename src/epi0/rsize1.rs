#[doc = "Register `RSIZE1` reader"]
pub struct R(crate::R<RSIZE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSIZE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSIZE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSIZE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSIZE1` writer"]
pub struct W(crate::W<RSIZE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSIZE1_SPEC>;
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
impl From<crate::W<RSIZE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSIZE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Current Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_RSIZE1_SIZE_A {
    #[doc = "1: Byte (8 bits)"]
    EPI_RSIZE1_SIZE_8BIT = 1,
    #[doc = "2: Half-word (16 bits)"]
    EPI_RSIZE1_SIZE_16BIT = 2,
    #[doc = "3: Word (32 bits)"]
    EPI_RSIZE1_SIZE_32BIT = 3,
}
impl From<EPI_RSIZE1_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_RSIZE1_SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_RSIZE1_SIZE` reader - Current Size"]
pub type EPI_RSIZE1_SIZE_R = crate::FieldReader<u8, EPI_RSIZE1_SIZE_A>;
impl EPI_RSIZE1_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPI_RSIZE1_SIZE_A> {
        match self.bits {
            1 => Some(EPI_RSIZE1_SIZE_A::EPI_RSIZE1_SIZE_8BIT),
            2 => Some(EPI_RSIZE1_SIZE_A::EPI_RSIZE1_SIZE_16BIT),
            3 => Some(EPI_RSIZE1_SIZE_A::EPI_RSIZE1_SIZE_32BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EPI_RSIZE1_SIZE_8BIT`"]
    #[inline(always)]
    pub fn is_epi_rsize1_size_8bit(&self) -> bool {
        *self == EPI_RSIZE1_SIZE_A::EPI_RSIZE1_SIZE_8BIT
    }
    #[doc = "Checks if the value of the field is `EPI_RSIZE1_SIZE_16BIT`"]
    #[inline(always)]
    pub fn is_epi_rsize1_size_16bit(&self) -> bool {
        *self == EPI_RSIZE1_SIZE_A::EPI_RSIZE1_SIZE_16BIT
    }
    #[doc = "Checks if the value of the field is `EPI_RSIZE1_SIZE_32BIT`"]
    #[inline(always)]
    pub fn is_epi_rsize1_size_32bit(&self) -> bool {
        *self == EPI_RSIZE1_SIZE_A::EPI_RSIZE1_SIZE_32BIT
    }
}
#[doc = "Field `EPI_RSIZE1_SIZE` writer - Current Size"]
pub type EPI_RSIZE1_SIZE_W<'a> =
    crate::FieldWriter<'a, u32, RSIZE1_SPEC, u8, EPI_RSIZE1_SIZE_A, 2, 0>;
impl<'a> EPI_RSIZE1_SIZE_W<'a> {
    #[doc = "Byte (8 bits)"]
    #[inline(always)]
    pub fn epi_rsize1_size_8bit(self) -> &'a mut W {
        self.variant(EPI_RSIZE1_SIZE_A::EPI_RSIZE1_SIZE_8BIT)
    }
    #[doc = "Half-word (16 bits)"]
    #[inline(always)]
    pub fn epi_rsize1_size_16bit(self) -> &'a mut W {
        self.variant(EPI_RSIZE1_SIZE_A::EPI_RSIZE1_SIZE_16BIT)
    }
    #[doc = "Word (32 bits)"]
    #[inline(always)]
    pub fn epi_rsize1_size_32bit(self) -> &'a mut W {
        self.variant(EPI_RSIZE1_SIZE_A::EPI_RSIZE1_SIZE_32BIT)
    }
}
impl R {
    #[doc = "Bits 0:1 - Current Size"]
    #[inline(always)]
    pub fn epi_rsize1_size(&self) -> EPI_RSIZE1_SIZE_R {
        EPI_RSIZE1_SIZE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Current Size"]
    #[inline(always)]
    pub fn epi_rsize1_size(&mut self) -> EPI_RSIZE1_SIZE_W {
        EPI_RSIZE1_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Read Size 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsize1](index.html) module"]
pub struct RSIZE1_SPEC;
impl crate::RegisterSpec for RSIZE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsize1::R](R) reader structure"]
impl crate::Readable for RSIZE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rsize1::W](W) writer structure"]
impl crate::Writable for RSIZE1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSIZE1 to value 0"]
impl crate::Resettable for RSIZE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
