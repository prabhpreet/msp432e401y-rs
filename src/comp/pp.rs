#[doc = "Register `PP` reader"]
pub struct R(crate::R<PP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PP` writer"]
pub struct W(crate::W<PP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PP_SPEC>;
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
impl From<crate::W<PP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP_PP_CMP0` reader - Comparator 0 Present"]
pub type COMP_PP_CMP0_R = crate::BitReader<bool>;
#[doc = "Field `COMP_PP_CMP0` writer - Comparator 0 Present"]
pub type COMP_PP_CMP0_W<'a> = crate::BitWriter<'a, u32, PP_SPEC, bool, 0>;
#[doc = "Field `COMP_PP_CMP1` reader - Comparator 1 Present"]
pub type COMP_PP_CMP1_R = crate::BitReader<bool>;
#[doc = "Field `COMP_PP_CMP1` writer - Comparator 1 Present"]
pub type COMP_PP_CMP1_W<'a> = crate::BitWriter<'a, u32, PP_SPEC, bool, 1>;
#[doc = "Field `COMP_PP_CMP2` reader - Comparator 2 Present"]
pub type COMP_PP_CMP2_R = crate::BitReader<bool>;
#[doc = "Field `COMP_PP_CMP2` writer - Comparator 2 Present"]
pub type COMP_PP_CMP2_W<'a> = crate::BitWriter<'a, u32, PP_SPEC, bool, 2>;
#[doc = "Field `COMP_PP_C0O` reader - Comparator Output 0 Present"]
pub type COMP_PP_C0O_R = crate::BitReader<bool>;
#[doc = "Field `COMP_PP_C0O` writer - Comparator Output 0 Present"]
pub type COMP_PP_C0O_W<'a> = crate::BitWriter<'a, u32, PP_SPEC, bool, 16>;
#[doc = "Field `COMP_PP_C1O` reader - Comparator Output 1 Present"]
pub type COMP_PP_C1O_R = crate::BitReader<bool>;
#[doc = "Field `COMP_PP_C1O` writer - Comparator Output 1 Present"]
pub type COMP_PP_C1O_W<'a> = crate::BitWriter<'a, u32, PP_SPEC, bool, 17>;
#[doc = "Field `COMP_PP_C2O` reader - Comparator Output 2 Present"]
pub type COMP_PP_C2O_R = crate::BitReader<bool>;
#[doc = "Field `COMP_PP_C2O` writer - Comparator Output 2 Present"]
pub type COMP_PP_C2O_W<'a> = crate::BitWriter<'a, u32, PP_SPEC, bool, 18>;
impl R {
    #[doc = "Bit 0 - Comparator 0 Present"]
    #[inline(always)]
    pub fn comp_pp_cmp0(&self) -> COMP_PP_CMP0_R {
        COMP_PP_CMP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Present"]
    #[inline(always)]
    pub fn comp_pp_cmp1(&self) -> COMP_PP_CMP1_R {
        COMP_PP_CMP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator 2 Present"]
    #[inline(always)]
    pub fn comp_pp_cmp2(&self) -> COMP_PP_CMP2_R {
        COMP_PP_CMP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Comparator Output 0 Present"]
    #[inline(always)]
    pub fn comp_pp_c0o(&self) -> COMP_PP_C0O_R {
        COMP_PP_C0O_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Comparator Output 1 Present"]
    #[inline(always)]
    pub fn comp_pp_c1o(&self) -> COMP_PP_C1O_R {
        COMP_PP_C1O_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Comparator Output 2 Present"]
    #[inline(always)]
    pub fn comp_pp_c2o(&self) -> COMP_PP_C2O_R {
        COMP_PP_C2O_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 0 Present"]
    #[inline(always)]
    pub fn comp_pp_cmp0(&mut self) -> COMP_PP_CMP0_W {
        COMP_PP_CMP0_W::new(self)
    }
    #[doc = "Bit 1 - Comparator 1 Present"]
    #[inline(always)]
    pub fn comp_pp_cmp1(&mut self) -> COMP_PP_CMP1_W {
        COMP_PP_CMP1_W::new(self)
    }
    #[doc = "Bit 2 - Comparator 2 Present"]
    #[inline(always)]
    pub fn comp_pp_cmp2(&mut self) -> COMP_PP_CMP2_W {
        COMP_PP_CMP2_W::new(self)
    }
    #[doc = "Bit 16 - Comparator Output 0 Present"]
    #[inline(always)]
    pub fn comp_pp_c0o(&mut self) -> COMP_PP_C0O_W {
        COMP_PP_C0O_W::new(self)
    }
    #[doc = "Bit 17 - Comparator Output 1 Present"]
    #[inline(always)]
    pub fn comp_pp_c1o(&mut self) -> COMP_PP_C1O_W {
        COMP_PP_C1O_W::new(self)
    }
    #[doc = "Bit 18 - Comparator Output 2 Present"]
    #[inline(always)]
    pub fn comp_pp_c2o(&mut self) -> COMP_PP_C2O_W {
        COMP_PP_C2O_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Comparator Peripheral Properties\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pp](index.html) module"]
pub struct PP_SPEC;
impl crate::RegisterSpec for PP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pp::R](R) reader structure"]
impl crate::Readable for PP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pp::W](W) writer structure"]
impl crate::Writable for PP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PP to value 0"]
impl crate::Resettable for PP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
