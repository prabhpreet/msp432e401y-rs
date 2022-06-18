#[doc = "Register `IF2CMSK` reader"]
pub struct R(crate::R<IF2CMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF2CMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF2CMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF2CMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IF2CMSK` writer"]
pub struct W(crate::W<IF2CMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF2CMSK_SPEC>;
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
impl From<crate::W<IF2CMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF2CMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN_IF2CMSK_DATAB` reader - Access Data Byte 4 to 7"]
pub type CAN_IF2CMSK_DATAB_R = crate::BitReader<bool>;
#[doc = "Field `CAN_IF2CMSK_DATAB` writer - Access Data Byte 4 to 7"]
pub type CAN_IF2CMSK_DATAB_W<'a> = crate::BitWriter<'a, u32, IF2CMSK_SPEC, bool, 0>;
#[doc = "Field `CAN_IF2CMSK_DATAA` reader - Access Data Byte 0 to 3"]
pub type CAN_IF2CMSK_DATAA_R = crate::BitReader<bool>;
#[doc = "Field `CAN_IF2CMSK_DATAA` writer - Access Data Byte 0 to 3"]
pub type CAN_IF2CMSK_DATAA_W<'a> = crate::BitWriter<'a, u32, IF2CMSK_SPEC, bool, 1>;
#[doc = "Field `CAN_IF2CMSK_NEWDAT` reader - Access New Data"]
pub type CAN_IF2CMSK_NEWDAT_R = crate::BitReader<bool>;
#[doc = "Field `CAN_IF2CMSK_NEWDAT` writer - Access New Data"]
pub type CAN_IF2CMSK_NEWDAT_W<'a> = crate::BitWriter<'a, u32, IF2CMSK_SPEC, bool, 2>;
#[doc = "Field `CAN_IF2CMSK_CLRINTPND` reader - Clear Interrupt Pending Bit"]
pub type CAN_IF2CMSK_CLRINTPND_R = crate::BitReader<bool>;
#[doc = "Field `CAN_IF2CMSK_CLRINTPND` writer - Clear Interrupt Pending Bit"]
pub type CAN_IF2CMSK_CLRINTPND_W<'a> = crate::BitWriter<'a, u32, IF2CMSK_SPEC, bool, 3>;
#[doc = "Field `CAN_IF2CMSK_CONTROL` reader - Access Control Bits"]
pub type CAN_IF2CMSK_CONTROL_R = crate::BitReader<bool>;
#[doc = "Field `CAN_IF2CMSK_CONTROL` writer - Access Control Bits"]
pub type CAN_IF2CMSK_CONTROL_W<'a> = crate::BitWriter<'a, u32, IF2CMSK_SPEC, bool, 4>;
#[doc = "Field `CAN_IF2CMSK_ARB` reader - Access Arbitration Bits"]
pub type CAN_IF2CMSK_ARB_R = crate::BitReader<bool>;
#[doc = "Field `CAN_IF2CMSK_ARB` writer - Access Arbitration Bits"]
pub type CAN_IF2CMSK_ARB_W<'a> = crate::BitWriter<'a, u32, IF2CMSK_SPEC, bool, 5>;
#[doc = "Field `CAN_IF2CMSK_MASK` reader - Access Mask Bits"]
pub type CAN_IF2CMSK_MASK_R = crate::BitReader<bool>;
#[doc = "Field `CAN_IF2CMSK_MASK` writer - Access Mask Bits"]
pub type CAN_IF2CMSK_MASK_W<'a> = crate::BitWriter<'a, u32, IF2CMSK_SPEC, bool, 6>;
#[doc = "Field `CAN_IF2CMSK_WRNRD` reader - Write, Not Read"]
pub type CAN_IF2CMSK_WRNRD_R = crate::BitReader<bool>;
#[doc = "Field `CAN_IF2CMSK_WRNRD` writer - Write, Not Read"]
pub type CAN_IF2CMSK_WRNRD_W<'a> = crate::BitWriter<'a, u32, IF2CMSK_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Access Data Byte 4 to 7"]
    #[inline(always)]
    pub fn can_if2cmsk_datab(&self) -> CAN_IF2CMSK_DATAB_R {
        CAN_IF2CMSK_DATAB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Access Data Byte 0 to 3"]
    #[inline(always)]
    pub fn can_if2cmsk_dataa(&self) -> CAN_IF2CMSK_DATAA_R {
        CAN_IF2CMSK_DATAA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Access New Data"]
    #[inline(always)]
    pub fn can_if2cmsk_newdat(&self) -> CAN_IF2CMSK_NEWDAT_R {
        CAN_IF2CMSK_NEWDAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear Interrupt Pending Bit"]
    #[inline(always)]
    pub fn can_if2cmsk_clrintpnd(&self) -> CAN_IF2CMSK_CLRINTPND_R {
        CAN_IF2CMSK_CLRINTPND_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Access Control Bits"]
    #[inline(always)]
    pub fn can_if2cmsk_control(&self) -> CAN_IF2CMSK_CONTROL_R {
        CAN_IF2CMSK_CONTROL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Access Arbitration Bits"]
    #[inline(always)]
    pub fn can_if2cmsk_arb(&self) -> CAN_IF2CMSK_ARB_R {
        CAN_IF2CMSK_ARB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Access Mask Bits"]
    #[inline(always)]
    pub fn can_if2cmsk_mask(&self) -> CAN_IF2CMSK_MASK_R {
        CAN_IF2CMSK_MASK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write, Not Read"]
    #[inline(always)]
    pub fn can_if2cmsk_wrnrd(&self) -> CAN_IF2CMSK_WRNRD_R {
        CAN_IF2CMSK_WRNRD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Access Data Byte 4 to 7"]
    #[inline(always)]
    pub fn can_if2cmsk_datab(&mut self) -> CAN_IF2CMSK_DATAB_W {
        CAN_IF2CMSK_DATAB_W::new(self)
    }
    #[doc = "Bit 1 - Access Data Byte 0 to 3"]
    #[inline(always)]
    pub fn can_if2cmsk_dataa(&mut self) -> CAN_IF2CMSK_DATAA_W {
        CAN_IF2CMSK_DATAA_W::new(self)
    }
    #[doc = "Bit 2 - Access New Data"]
    #[inline(always)]
    pub fn can_if2cmsk_newdat(&mut self) -> CAN_IF2CMSK_NEWDAT_W {
        CAN_IF2CMSK_NEWDAT_W::new(self)
    }
    #[doc = "Bit 3 - Clear Interrupt Pending Bit"]
    #[inline(always)]
    pub fn can_if2cmsk_clrintpnd(&mut self) -> CAN_IF2CMSK_CLRINTPND_W {
        CAN_IF2CMSK_CLRINTPND_W::new(self)
    }
    #[doc = "Bit 4 - Access Control Bits"]
    #[inline(always)]
    pub fn can_if2cmsk_control(&mut self) -> CAN_IF2CMSK_CONTROL_W {
        CAN_IF2CMSK_CONTROL_W::new(self)
    }
    #[doc = "Bit 5 - Access Arbitration Bits"]
    #[inline(always)]
    pub fn can_if2cmsk_arb(&mut self) -> CAN_IF2CMSK_ARB_W {
        CAN_IF2CMSK_ARB_W::new(self)
    }
    #[doc = "Bit 6 - Access Mask Bits"]
    #[inline(always)]
    pub fn can_if2cmsk_mask(&mut self) -> CAN_IF2CMSK_MASK_W {
        CAN_IF2CMSK_MASK_W::new(self)
    }
    #[doc = "Bit 7 - Write, Not Read"]
    #[inline(always)]
    pub fn can_if2cmsk_wrnrd(&mut self) -> CAN_IF2CMSK_WRNRD_W {
        CAN_IF2CMSK_WRNRD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN IF2 Command Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if2cmsk](index.html) module"]
pub struct IF2CMSK_SPEC;
impl crate::RegisterSpec for IF2CMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if2cmsk::R](R) reader structure"]
impl crate::Readable for IF2CMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if2cmsk::W](W) writer structure"]
impl crate::Writable for IF2CMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IF2CMSK to value 0"]
impl crate::Resettable for IF2CMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
