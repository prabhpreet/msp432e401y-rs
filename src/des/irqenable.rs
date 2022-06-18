#[doc = "Register `IRQENABLE` reader"]
pub struct R(crate::R<IRQENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQENABLE` writer"]
pub struct W(crate::W<IRQENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQENABLE_SPEC>;
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
impl From<crate::W<IRQENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DES_IRQENABLE_M_CONTEX_IN` reader - If this bit is set to 1 the context interrupt is enabled"]
pub type DES_IRQENABLE_M_CONTEX_IN_R = crate::BitReader<bool>;
#[doc = "Field `DES_IRQENABLE_M_CONTEX_IN` writer - If this bit is set to 1 the context interrupt is enabled"]
pub type DES_IRQENABLE_M_CONTEX_IN_W<'a> = crate::BitWriter<'a, u32, IRQENABLE_SPEC, bool, 0>;
#[doc = "Field `DES_IRQENABLE_M_DATA_IN` reader - If this bit is set to 1 the data input interrupt is enabled"]
pub type DES_IRQENABLE_M_DATA_IN_R = crate::BitReader<bool>;
#[doc = "Field `DES_IRQENABLE_M_DATA_IN` writer - If this bit is set to 1 the data input interrupt is enabled"]
pub type DES_IRQENABLE_M_DATA_IN_W<'a> = crate::BitWriter<'a, u32, IRQENABLE_SPEC, bool, 1>;
#[doc = "Field `DES_IRQENABLE_M_DATA_OUT` reader - If this bit is set to 1 the data output interrupt is enabled"]
pub type DES_IRQENABLE_M_DATA_OUT_R = crate::BitReader<bool>;
#[doc = "Field `DES_IRQENABLE_M_DATA_OUT` writer - If this bit is set to 1 the data output interrupt is enabled"]
pub type DES_IRQENABLE_M_DATA_OUT_W<'a> = crate::BitWriter<'a, u32, IRQENABLE_SPEC, bool, 2>;
impl R {
    #[doc = "Bit 0 - If this bit is set to 1 the context interrupt is enabled"]
    #[inline(always)]
    pub fn des_irqenable_m_contex_in(&self) -> DES_IRQENABLE_M_CONTEX_IN_R {
        DES_IRQENABLE_M_CONTEX_IN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If this bit is set to 1 the data input interrupt is enabled"]
    #[inline(always)]
    pub fn des_irqenable_m_data_in(&self) -> DES_IRQENABLE_M_DATA_IN_R {
        DES_IRQENABLE_M_DATA_IN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If this bit is set to 1 the data output interrupt is enabled"]
    #[inline(always)]
    pub fn des_irqenable_m_data_out(&self) -> DES_IRQENABLE_M_DATA_OUT_R {
        DES_IRQENABLE_M_DATA_OUT_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is set to 1 the context interrupt is enabled"]
    #[inline(always)]
    pub fn des_irqenable_m_contex_in(&mut self) -> DES_IRQENABLE_M_CONTEX_IN_W {
        DES_IRQENABLE_M_CONTEX_IN_W::new(self)
    }
    #[doc = "Bit 1 - If this bit is set to 1 the data input interrupt is enabled"]
    #[inline(always)]
    pub fn des_irqenable_m_data_in(&mut self) -> DES_IRQENABLE_M_DATA_IN_W {
        DES_IRQENABLE_M_DATA_IN_W::new(self)
    }
    #[doc = "Bit 2 - If this bit is set to 1 the data output interrupt is enabled"]
    #[inline(always)]
    pub fn des_irqenable_m_data_out(&mut self) -> DES_IRQENABLE_M_DATA_OUT_W {
        DES_IRQENABLE_M_DATA_OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DES Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqenable](index.html) module"]
pub struct IRQENABLE_SPEC;
impl crate::RegisterSpec for IRQENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irqenable::R](R) reader structure"]
impl crate::Readable for IRQENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irqenable::W](W) writer structure"]
impl crate::Writable for IRQENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRQENABLE to value 0"]
impl crate::Resettable for IRQENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
