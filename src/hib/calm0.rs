#[doc = "Register `CALM0` reader"]
pub struct R(crate::R<CALM0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALM0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALM0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALM0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALM0` writer"]
pub struct W(crate::W<CALM0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALM0_SPEC>;
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
impl From<crate::W<CALM0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALM0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIB_CALM0_SEC` reader - Seconds"]
pub type HIB_CALM0_SEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HIB_CALM0_SEC` writer - Seconds"]
pub type HIB_CALM0_SEC_W<'a> = crate::FieldWriter<'a, u32, CALM0_SPEC, u8, u8, 6, 0>;
#[doc = "Field `HIB_CALM0_MIN` reader - Minutes"]
pub type HIB_CALM0_MIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HIB_CALM0_MIN` writer - Minutes"]
pub type HIB_CALM0_MIN_W<'a> = crate::FieldWriter<'a, u32, CALM0_SPEC, u8, u8, 6, 8>;
#[doc = "Field `HIB_CALM0_HR` reader - Hours"]
pub type HIB_CALM0_HR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HIB_CALM0_HR` writer - Hours"]
pub type HIB_CALM0_HR_W<'a> = crate::FieldWriter<'a, u32, CALM0_SPEC, u8, u8, 5, 16>;
#[doc = "Field `HIB_CALM0_AMPM` reader - AM/PM Designation"]
pub type HIB_CALM0_AMPM_R = crate::BitReader<bool>;
#[doc = "Field `HIB_CALM0_AMPM` writer - AM/PM Designation"]
pub type HIB_CALM0_AMPM_W<'a> = crate::BitWriter<'a, u32, CALM0_SPEC, bool, 22>;
impl R {
    #[doc = "Bits 0:5 - Seconds"]
    #[inline(always)]
    pub fn hib_calm0_sec(&self) -> HIB_CALM0_SEC_R {
        HIB_CALM0_SEC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Minutes"]
    #[inline(always)]
    pub fn hib_calm0_min(&self) -> HIB_CALM0_MIN_R {
        HIB_CALM0_MIN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Hours"]
    #[inline(always)]
    pub fn hib_calm0_hr(&self) -> HIB_CALM0_HR_R {
        HIB_CALM0_HR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - AM/PM Designation"]
    #[inline(always)]
    pub fn hib_calm0_ampm(&self) -> HIB_CALM0_AMPM_R {
        HIB_CALM0_AMPM_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Seconds"]
    #[inline(always)]
    pub fn hib_calm0_sec(&mut self) -> HIB_CALM0_SEC_W {
        HIB_CALM0_SEC_W::new(self)
    }
    #[doc = "Bits 8:13 - Minutes"]
    #[inline(always)]
    pub fn hib_calm0_min(&mut self) -> HIB_CALM0_MIN_W {
        HIB_CALM0_MIN_W::new(self)
    }
    #[doc = "Bits 16:20 - Hours"]
    #[inline(always)]
    pub fn hib_calm0_hr(&mut self) -> HIB_CALM0_HR_W {
        HIB_CALM0_HR_W::new(self)
    }
    #[doc = "Bit 22 - AM/PM Designation"]
    #[inline(always)]
    pub fn hib_calm0_ampm(&mut self) -> HIB_CALM0_AMPM_W {
        HIB_CALM0_AMPM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernation Calendar Match 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calm0](index.html) module"]
pub struct CALM0_SPEC;
impl crate::RegisterSpec for CALM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calm0::R](R) reader structure"]
impl crate::Readable for CALM0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calm0::W](W) writer structure"]
impl crate::Writable for CALM0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALM0 to value 0"]
impl crate::Resettable for CALM0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
