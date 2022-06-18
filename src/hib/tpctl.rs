#[doc = "Register `TPCTL` reader"]
pub struct R(crate::R<TPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPCTL` writer"]
pub struct W(crate::W<TPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPCTL_SPEC>;
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
impl From<crate::W<TPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIB_TPCTL_TPEN` reader - Tamper Module Enable"]
pub type HIB_TPCTL_TPEN_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TPCTL_TPEN` writer - Tamper Module Enable"]
pub type HIB_TPCTL_TPEN_W<'a> = crate::BitWriter<'a, u32, TPCTL_SPEC, bool, 0>;
#[doc = "Field `HIB_TPCTL_TPCLR` reader - Tamper Event Clear"]
pub type HIB_TPCTL_TPCLR_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TPCTL_TPCLR` writer - Tamper Event Clear"]
pub type HIB_TPCTL_TPCLR_W<'a> = crate::BitWriter<'a, u32, TPCTL_SPEC, bool, 4>;
#[doc = "HIB Memory Clear on Tamper Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HIB_TPCTL_MEMCLR_A {
    #[doc = "0: Do not Clear HIB memory on tamper event"]
    HIB_TPCTL_MEMCLR_NONE = 0,
    #[doc = "1: Clear Lower 32 Bytes of HIB memory on tamper event"]
    HIB_TPCTL_MEMCLR_LOW32 = 1,
    #[doc = "2: Clear upper 32 Bytes of HIB memory on tamper event"]
    HIB_TPCTL_MEMCLR_HIGH32 = 2,
    #[doc = "3: Clear all HIB memory on tamper event"]
    HIB_TPCTL_MEMCLR_ALL = 3,
}
impl From<HIB_TPCTL_MEMCLR_A> for u8 {
    #[inline(always)]
    fn from(variant: HIB_TPCTL_MEMCLR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HIB_TPCTL_MEMCLR` reader - HIB Memory Clear on Tamper Event"]
pub type HIB_TPCTL_MEMCLR_R = crate::FieldReader<u8, HIB_TPCTL_MEMCLR_A>;
impl HIB_TPCTL_MEMCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIB_TPCTL_MEMCLR_A {
        match self.bits {
            0 => HIB_TPCTL_MEMCLR_A::HIB_TPCTL_MEMCLR_NONE,
            1 => HIB_TPCTL_MEMCLR_A::HIB_TPCTL_MEMCLR_LOW32,
            2 => HIB_TPCTL_MEMCLR_A::HIB_TPCTL_MEMCLR_HIGH32,
            3 => HIB_TPCTL_MEMCLR_A::HIB_TPCTL_MEMCLR_ALL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HIB_TPCTL_MEMCLR_NONE`"]
    #[inline(always)]
    pub fn is_hib_tpctl_memclr_none(&self) -> bool {
        *self == HIB_TPCTL_MEMCLR_A::HIB_TPCTL_MEMCLR_NONE
    }
    #[doc = "Checks if the value of the field is `HIB_TPCTL_MEMCLR_LOW32`"]
    #[inline(always)]
    pub fn is_hib_tpctl_memclr_low32(&self) -> bool {
        *self == HIB_TPCTL_MEMCLR_A::HIB_TPCTL_MEMCLR_LOW32
    }
    #[doc = "Checks if the value of the field is `HIB_TPCTL_MEMCLR_HIGH32`"]
    #[inline(always)]
    pub fn is_hib_tpctl_memclr_high32(&self) -> bool {
        *self == HIB_TPCTL_MEMCLR_A::HIB_TPCTL_MEMCLR_HIGH32
    }
    #[doc = "Checks if the value of the field is `HIB_TPCTL_MEMCLR_ALL`"]
    #[inline(always)]
    pub fn is_hib_tpctl_memclr_all(&self) -> bool {
        *self == HIB_TPCTL_MEMCLR_A::HIB_TPCTL_MEMCLR_ALL
    }
}
#[doc = "Field `HIB_TPCTL_MEMCLR` writer - HIB Memory Clear on Tamper Event"]
pub type HIB_TPCTL_MEMCLR_W<'a> =
    crate::FieldWriterSafe<'a, u32, TPCTL_SPEC, u8, HIB_TPCTL_MEMCLR_A, 2, 8>;
impl<'a> HIB_TPCTL_MEMCLR_W<'a> {
    #[doc = "Do not Clear HIB memory on tamper event"]
    #[inline(always)]
    pub fn hib_tpctl_memclr_none(self) -> &'a mut W {
        self.variant(HIB_TPCTL_MEMCLR_A::HIB_TPCTL_MEMCLR_NONE)
    }
    #[doc = "Clear Lower 32 Bytes of HIB memory on tamper event"]
    #[inline(always)]
    pub fn hib_tpctl_memclr_low32(self) -> &'a mut W {
        self.variant(HIB_TPCTL_MEMCLR_A::HIB_TPCTL_MEMCLR_LOW32)
    }
    #[doc = "Clear upper 32 Bytes of HIB memory on tamper event"]
    #[inline(always)]
    pub fn hib_tpctl_memclr_high32(self) -> &'a mut W {
        self.variant(HIB_TPCTL_MEMCLR_A::HIB_TPCTL_MEMCLR_HIGH32)
    }
    #[doc = "Clear all HIB memory on tamper event"]
    #[inline(always)]
    pub fn hib_tpctl_memclr_all(self) -> &'a mut W {
        self.variant(HIB_TPCTL_MEMCLR_A::HIB_TPCTL_MEMCLR_ALL)
    }
}
#[doc = "Field `HIB_TPCTL_WAKE` reader - Wake from Hibernate on a Tamper Event"]
pub type HIB_TPCTL_WAKE_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TPCTL_WAKE` writer - Wake from Hibernate on a Tamper Event"]
pub type HIB_TPCTL_WAKE_W<'a> = crate::BitWriter<'a, u32, TPCTL_SPEC, bool, 11>;
impl R {
    #[doc = "Bit 0 - Tamper Module Enable"]
    #[inline(always)]
    pub fn hib_tpctl_tpen(&self) -> HIB_TPCTL_TPEN_R {
        HIB_TPCTL_TPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Tamper Event Clear"]
    #[inline(always)]
    pub fn hib_tpctl_tpclr(&self) -> HIB_TPCTL_TPCLR_R {
        HIB_TPCTL_TPCLR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - HIB Memory Clear on Tamper Event"]
    #[inline(always)]
    pub fn hib_tpctl_memclr(&self) -> HIB_TPCTL_MEMCLR_R {
        HIB_TPCTL_MEMCLR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - Wake from Hibernate on a Tamper Event"]
    #[inline(always)]
    pub fn hib_tpctl_wake(&self) -> HIB_TPCTL_WAKE_R {
        HIB_TPCTL_WAKE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper Module Enable"]
    #[inline(always)]
    pub fn hib_tpctl_tpen(&mut self) -> HIB_TPCTL_TPEN_W {
        HIB_TPCTL_TPEN_W::new(self)
    }
    #[doc = "Bit 4 - Tamper Event Clear"]
    #[inline(always)]
    pub fn hib_tpctl_tpclr(&mut self) -> HIB_TPCTL_TPCLR_W {
        HIB_TPCTL_TPCLR_W::new(self)
    }
    #[doc = "Bits 8:9 - HIB Memory Clear on Tamper Event"]
    #[inline(always)]
    pub fn hib_tpctl_memclr(&mut self) -> HIB_TPCTL_MEMCLR_W {
        HIB_TPCTL_MEMCLR_W::new(self)
    }
    #[doc = "Bit 11 - Wake from Hibernate on a Tamper Event"]
    #[inline(always)]
    pub fn hib_tpctl_wake(&mut self) -> HIB_TPCTL_WAKE_W {
        HIB_TPCTL_WAKE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HIB Tamper Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpctl](index.html) module"]
pub struct TPCTL_SPEC;
impl crate::RegisterSpec for TPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpctl::R](R) reader structure"]
impl crate::Readable for TPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpctl::W](W) writer structure"]
impl crate::Writable for TPCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPCTL to value 0"]
impl crate::Resettable for TPCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
