#[doc = "Register `CAL1` reader"]
pub struct R(crate::R<CAL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAL1` writer"]
pub struct W(crate::W<CAL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAL1_SPEC>;
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
impl From<crate::W<CAL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIB_CAL1_DOM` reader - Day of Month"]
pub type HIB_CAL1_DOM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HIB_CAL1_DOM` writer - Day of Month"]
pub type HIB_CAL1_DOM_W<'a> = crate::FieldWriter<'a, u32, CAL1_SPEC, u8, u8, 5, 0>;
#[doc = "Field `HIB_CAL1_MON` reader - Month"]
pub type HIB_CAL1_MON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HIB_CAL1_MON` writer - Month"]
pub type HIB_CAL1_MON_W<'a> = crate::FieldWriter<'a, u32, CAL1_SPEC, u8, u8, 4, 8>;
#[doc = "Field `HIB_CAL1_YEAR` reader - Year Value"]
pub type HIB_CAL1_YEAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HIB_CAL1_YEAR` writer - Year Value"]
pub type HIB_CAL1_YEAR_W<'a> = crate::FieldWriter<'a, u32, CAL1_SPEC, u8, u8, 7, 16>;
#[doc = "Field `HIB_CAL1_DOW` reader - Day of Week"]
pub type HIB_CAL1_DOW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HIB_CAL1_DOW` writer - Day of Week"]
pub type HIB_CAL1_DOW_W<'a> = crate::FieldWriter<'a, u32, CAL1_SPEC, u8, u8, 3, 24>;
#[doc = "Field `HIB_CAL1_VALID` reader - Valid Calendar Load"]
pub type HIB_CAL1_VALID_R = crate::BitReader<bool>;
#[doc = "Field `HIB_CAL1_VALID` writer - Valid Calendar Load"]
pub type HIB_CAL1_VALID_W<'a> = crate::BitWriter<'a, u32, CAL1_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:4 - Day of Month"]
    #[inline(always)]
    pub fn hib_cal1_dom(&self) -> HIB_CAL1_DOM_R {
        HIB_CAL1_DOM_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Month"]
    #[inline(always)]
    pub fn hib_cal1_mon(&self) -> HIB_CAL1_MON_R {
        HIB_CAL1_MON_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:22 - Year Value"]
    #[inline(always)]
    pub fn hib_cal1_year(&self) -> HIB_CAL1_YEAR_R {
        HIB_CAL1_YEAR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:26 - Day of Week"]
    #[inline(always)]
    pub fn hib_cal1_dow(&self) -> HIB_CAL1_DOW_R {
        HIB_CAL1_DOW_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - Valid Calendar Load"]
    #[inline(always)]
    pub fn hib_cal1_valid(&self) -> HIB_CAL1_VALID_R {
        HIB_CAL1_VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Day of Month"]
    #[inline(always)]
    pub fn hib_cal1_dom(&mut self) -> HIB_CAL1_DOM_W {
        HIB_CAL1_DOM_W::new(self)
    }
    #[doc = "Bits 8:11 - Month"]
    #[inline(always)]
    pub fn hib_cal1_mon(&mut self) -> HIB_CAL1_MON_W {
        HIB_CAL1_MON_W::new(self)
    }
    #[doc = "Bits 16:22 - Year Value"]
    #[inline(always)]
    pub fn hib_cal1_year(&mut self) -> HIB_CAL1_YEAR_W {
        HIB_CAL1_YEAR_W::new(self)
    }
    #[doc = "Bits 24:26 - Day of Week"]
    #[inline(always)]
    pub fn hib_cal1_dow(&mut self) -> HIB_CAL1_DOW_W {
        HIB_CAL1_DOW_W::new(self)
    }
    #[doc = "Bit 31 - Valid Calendar Load"]
    #[inline(always)]
    pub fn hib_cal1_valid(&mut self) -> HIB_CAL1_VALID_W {
        HIB_CAL1_VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernation Calendar 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal1](index.html) module"]
pub struct CAL1_SPEC;
impl crate::RegisterSpec for CAL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cal1::R](R) reader structure"]
impl crate::Readable for CAL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cal1::W](W) writer structure"]
impl crate::Writable for CAL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAL1 to value 0"]
impl crate::Resettable for CAL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
