#[doc = "Register `CTLBASE` reader"]
pub struct R(crate::R<CTLBASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTLBASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTLBASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTLBASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTLBASE` writer"]
pub struct W(crate::W<CTLBASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTLBASE_SPEC>;
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
impl From<crate::W<CTLBASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTLBASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UDMA_CTLBASE_ADDR` reader - Channel Control Base Address"]
pub type UDMA_CTLBASE_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `UDMA_CTLBASE_ADDR` writer - Channel Control Base Address"]
pub type UDMA_CTLBASE_ADDR_W<'a> = crate::FieldWriter<'a, u32, CTLBASE_SPEC, u32, u32, 22, 10>;
impl R {
    #[doc = "Bits 10:31 - Channel Control Base Address"]
    #[inline(always)]
    pub fn udma_ctlbase_addr(&self) -> UDMA_CTLBASE_ADDR_R {
        UDMA_CTLBASE_ADDR_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 10:31 - Channel Control Base Address"]
    #[inline(always)]
    pub fn udma_ctlbase_addr(&mut self) -> UDMA_CTLBASE_ADDR_W {
        UDMA_CTLBASE_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel Control Base Pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctlbase](index.html) module"]
pub struct CTLBASE_SPEC;
impl crate::RegisterSpec for CTLBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctlbase::R](R) reader structure"]
impl crate::Readable for CTLBASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctlbase::W](W) writer structure"]
impl crate::Writable for CTLBASE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTLBASE to value 0"]
impl crate::Resettable for CTLBASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
