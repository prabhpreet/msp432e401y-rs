#[doc = "Register `IF2MSK2` reader"]
pub struct R(crate::R<IF2MSK2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF2MSK2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF2MSK2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF2MSK2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IF2MSK2` writer"]
pub struct W(crate::W<IF2MSK2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF2MSK2_SPEC>;
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
impl From<crate::W<IF2MSK2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF2MSK2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN_IF2MSK2_IDMSK` reader - Identifier Mask"]
pub type CAN_IF2MSK2_IDMSK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CAN_IF2MSK2_IDMSK` writer - Identifier Mask"]
pub type CAN_IF2MSK2_IDMSK_W<'a> = crate::FieldWriter<'a, u32, IF2MSK2_SPEC, u16, u16, 13, 0>;
#[doc = "Field `CAN_IF2MSK2_MDIR` reader - Mask Message Direction"]
pub type CAN_IF2MSK2_MDIR_R = crate::BitReader<bool>;
#[doc = "Field `CAN_IF2MSK2_MDIR` writer - Mask Message Direction"]
pub type CAN_IF2MSK2_MDIR_W<'a> = crate::BitWriter<'a, u32, IF2MSK2_SPEC, bool, 14>;
#[doc = "Field `CAN_IF2MSK2_MXTD` reader - Mask Extended Identifier"]
pub type CAN_IF2MSK2_MXTD_R = crate::BitReader<bool>;
#[doc = "Field `CAN_IF2MSK2_MXTD` writer - Mask Extended Identifier"]
pub type CAN_IF2MSK2_MXTD_W<'a> = crate::BitWriter<'a, u32, IF2MSK2_SPEC, bool, 15>;
impl R {
    #[doc = "Bits 0:12 - Identifier Mask"]
    #[inline(always)]
    pub fn can_if2msk2_idmsk(&self) -> CAN_IF2MSK2_IDMSK_R {
        CAN_IF2MSK2_IDMSK_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 14 - Mask Message Direction"]
    #[inline(always)]
    pub fn can_if2msk2_mdir(&self) -> CAN_IF2MSK2_MDIR_R {
        CAN_IF2MSK2_MDIR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Mask Extended Identifier"]
    #[inline(always)]
    pub fn can_if2msk2_mxtd(&self) -> CAN_IF2MSK2_MXTD_R {
        CAN_IF2MSK2_MXTD_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - Identifier Mask"]
    #[inline(always)]
    pub fn can_if2msk2_idmsk(&mut self) -> CAN_IF2MSK2_IDMSK_W {
        CAN_IF2MSK2_IDMSK_W::new(self)
    }
    #[doc = "Bit 14 - Mask Message Direction"]
    #[inline(always)]
    pub fn can_if2msk2_mdir(&mut self) -> CAN_IF2MSK2_MDIR_W {
        CAN_IF2MSK2_MDIR_W::new(self)
    }
    #[doc = "Bit 15 - Mask Extended Identifier"]
    #[inline(always)]
    pub fn can_if2msk2_mxtd(&mut self) -> CAN_IF2MSK2_MXTD_W {
        CAN_IF2MSK2_MXTD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN IF2 Mask 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if2msk2](index.html) module"]
pub struct IF2MSK2_SPEC;
impl crate::RegisterSpec for IF2MSK2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if2msk2::R](R) reader structure"]
impl crate::Readable for IF2MSK2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if2msk2::W](W) writer structure"]
impl crate::Writable for IF2MSK2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IF2MSK2 to value 0"]
impl crate::Resettable for IF2MSK2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
