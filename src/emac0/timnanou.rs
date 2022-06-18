#[doc = "Register `TIMNANOU` reader"]
pub struct R(crate::R<TIMNANOU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMNANOU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMNANOU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMNANOU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMNANOU` writer"]
pub struct W(crate::W<TIMNANOU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMNANOU_SPEC>;
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
impl From<crate::W<TIMNANOU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMNANOU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_TIMNANOU_TSSS` reader - Timestamp Sub-Second"]
pub type EMAC_TIMNANOU_TSSS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMAC_TIMNANOU_TSSS` writer - Timestamp Sub-Second"]
pub type EMAC_TIMNANOU_TSSS_W<'a> = crate::FieldWriter<'a, u32, TIMNANOU_SPEC, u32, u32, 31, 0>;
#[doc = "Field `EMAC_TIMNANOU_ADDSUB` reader - Add or subtract time"]
pub type EMAC_TIMNANOU_ADDSUB_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_TIMNANOU_ADDSUB` writer - Add or subtract time"]
pub type EMAC_TIMNANOU_ADDSUB_W<'a> = crate::BitWriter<'a, u32, TIMNANOU_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:30 - Timestamp Sub-Second"]
    #[inline(always)]
    pub fn emac_timnanou_tsss(&self) -> EMAC_TIMNANOU_TSSS_R {
        EMAC_TIMNANOU_TSSS_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - Add or subtract time"]
    #[inline(always)]
    pub fn emac_timnanou_addsub(&self) -> EMAC_TIMNANOU_ADDSUB_R {
        EMAC_TIMNANOU_ADDSUB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Timestamp Sub-Second"]
    #[inline(always)]
    pub fn emac_timnanou_tsss(&mut self) -> EMAC_TIMNANOU_TSSS_W {
        EMAC_TIMNANOU_TSSS_W::new(self)
    }
    #[doc = "Bit 31 - Add or subtract time"]
    #[inline(always)]
    pub fn emac_timnanou_addsub(&mut self) -> EMAC_TIMNANOU_ADDSUB_W {
        EMAC_TIMNANOU_ADDSUB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC System Time - Nanoseconds Update\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timnanou](index.html) module"]
pub struct TIMNANOU_SPEC;
impl crate::RegisterSpec for TIMNANOU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timnanou::R](R) reader structure"]
impl crate::Readable for TIMNANOU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timnanou::W](W) writer structure"]
impl crate::Writable for TIMNANOU_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMNANOU to value 0"]
impl crate::Resettable for TIMNANOU_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
