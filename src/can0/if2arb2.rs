#[doc = "Register `IF2ARB2` reader"]
pub struct R(crate::R<IF2ARB2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF2ARB2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF2ARB2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF2ARB2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IF2ARB2` writer"]
pub struct W(crate::W<IF2ARB2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF2ARB2_SPEC>;
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
impl From<crate::W<IF2ARB2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF2ARB2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN_IF2ARB2_ID` reader - Message Identifier"]
pub type CAN_IF2ARB2_ID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CAN_IF2ARB2_ID` writer - Message Identifier"]
pub type CAN_IF2ARB2_ID_W<'a> = crate::FieldWriter<'a, u32, IF2ARB2_SPEC, u16, u16, 13, 0>;
#[doc = "Field `CAN_IF2ARB2_DIR` reader - Message Direction"]
pub type CAN_IF2ARB2_DIR_R = crate::BitReader<bool>;
#[doc = "Field `CAN_IF2ARB2_DIR` writer - Message Direction"]
pub type CAN_IF2ARB2_DIR_W<'a> = crate::BitWriter<'a, u32, IF2ARB2_SPEC, bool, 13>;
#[doc = "Field `CAN_IF2ARB2_XTD` reader - Extended Identifier"]
pub type CAN_IF2ARB2_XTD_R = crate::BitReader<bool>;
#[doc = "Field `CAN_IF2ARB2_XTD` writer - Extended Identifier"]
pub type CAN_IF2ARB2_XTD_W<'a> = crate::BitWriter<'a, u32, IF2ARB2_SPEC, bool, 14>;
#[doc = "Field `CAN_IF2ARB2_MSGVAL` reader - Message Valid"]
pub type CAN_IF2ARB2_MSGVAL_R = crate::BitReader<bool>;
#[doc = "Field `CAN_IF2ARB2_MSGVAL` writer - Message Valid"]
pub type CAN_IF2ARB2_MSGVAL_W<'a> = crate::BitWriter<'a, u32, IF2ARB2_SPEC, bool, 15>;
impl R {
    #[doc = "Bits 0:12 - Message Identifier"]
    #[inline(always)]
    pub fn can_if2arb2_id(&self) -> CAN_IF2ARB2_ID_R {
        CAN_IF2ARB2_ID_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - Message Direction"]
    #[inline(always)]
    pub fn can_if2arb2_dir(&self) -> CAN_IF2ARB2_DIR_R {
        CAN_IF2ARB2_DIR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Extended Identifier"]
    #[inline(always)]
    pub fn can_if2arb2_xtd(&self) -> CAN_IF2ARB2_XTD_R {
        CAN_IF2ARB2_XTD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Message Valid"]
    #[inline(always)]
    pub fn can_if2arb2_msgval(&self) -> CAN_IF2ARB2_MSGVAL_R {
        CAN_IF2ARB2_MSGVAL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - Message Identifier"]
    #[inline(always)]
    pub fn can_if2arb2_id(&mut self) -> CAN_IF2ARB2_ID_W {
        CAN_IF2ARB2_ID_W::new(self)
    }
    #[doc = "Bit 13 - Message Direction"]
    #[inline(always)]
    pub fn can_if2arb2_dir(&mut self) -> CAN_IF2ARB2_DIR_W {
        CAN_IF2ARB2_DIR_W::new(self)
    }
    #[doc = "Bit 14 - Extended Identifier"]
    #[inline(always)]
    pub fn can_if2arb2_xtd(&mut self) -> CAN_IF2ARB2_XTD_W {
        CAN_IF2ARB2_XTD_W::new(self)
    }
    #[doc = "Bit 15 - Message Valid"]
    #[inline(always)]
    pub fn can_if2arb2_msgval(&mut self) -> CAN_IF2ARB2_MSGVAL_W {
        CAN_IF2ARB2_MSGVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN IF2 Arbitration 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if2arb2](index.html) module"]
pub struct IF2ARB2_SPEC;
impl crate::RegisterSpec for IF2ARB2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if2arb2::R](R) reader structure"]
impl crate::Readable for IF2ARB2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if2arb2::W](W) writer structure"]
impl crate::Writable for IF2ARB2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IF2ARB2 to value 0"]
impl crate::Resettable for IF2ARB2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
