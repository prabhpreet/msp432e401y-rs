#[doc = "Register `ACSTAT0` reader"]
pub struct R(crate::R<ACSTAT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACSTAT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACSTAT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACSTAT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACSTAT0` writer"]
pub struct W(crate::W<ACSTAT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACSTAT0_SPEC>;
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
impl From<crate::W<ACSTAT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACSTAT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP_ACSTAT0_OVAL` reader - Comparator Output Value"]
pub type COMP_ACSTAT0_OVAL_R = crate::BitReader<bool>;
#[doc = "Field `COMP_ACSTAT0_OVAL` writer - Comparator Output Value"]
pub type COMP_ACSTAT0_OVAL_W<'a> = crate::BitWriter<'a, u32, ACSTAT0_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 1 - Comparator Output Value"]
    #[inline(always)]
    pub fn comp_acstat0_oval(&self) -> COMP_ACSTAT0_OVAL_R {
        COMP_ACSTAT0_OVAL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Comparator Output Value"]
    #[inline(always)]
    pub fn comp_acstat0_oval(&mut self) -> COMP_ACSTAT0_OVAL_W {
        COMP_ACSTAT0_OVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Comparator Status 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acstat0](index.html) module"]
pub struct ACSTAT0_SPEC;
impl crate::RegisterSpec for ACSTAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acstat0::R](R) reader structure"]
impl crate::Readable for ACSTAT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acstat0::W](W) writer structure"]
impl crate::Writable for ACSTAT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACSTAT0 to value 0"]
impl crate::Resettable for ACSTAT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
